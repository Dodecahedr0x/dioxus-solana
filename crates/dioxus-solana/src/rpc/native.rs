//! Native (Android/iOS) transport: Agave's nonblocking client for HTTP and a
//! tokio-tungstenite WebSocket for PubSub, mirroring spume's wire behavior.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll};

use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use solana_rpc_client::nonblocking::rpc_client::RpcClient as AgaveClient;
use solana_rpc_client_api::client_error::ErrorKind;
use solana_rpc_client_types::request::{RpcError, RpcRequest, RpcResponseErrorData};
use tokio_tungstenite::tungstenite::Message;

use super::RpcResult;

// --- HTTP ---

#[derive(Clone)]
pub struct Http(Arc<AgaveClient>);

impl Http {
    pub fn new(url: &str) -> Self {
        Self(Arc::new(AgaveClient::new(url.to_string())))
    }

    pub async fn send<R: DeserializeOwned>(
        &self,
        request: RpcRequest,
        params: Value,
    ) -> RpcResult<R> {
        self.0.send(request, params).await.map_err(|err| {
            Box::new(match *err.kind {
                ErrorKind::RpcError(rpc) => rpc,
                kind => RpcError::RpcRequestError(kind.to_string()),
            })
        })
    }
}

// --- PubSub ---

type Reply = oneshot::Sender<RpcResult<Value>>;
type Notify = mpsc::UnboundedSender<RpcResult<Value>>;

struct Inner {
    out_tx: mpsc::UnboundedSender<String>,
    pending: Mutex<HashMap<u64, Reply>>,
    subscriptions: Mutex<HashMap<u64, Notify>>,
    next_id: AtomicU64,
}

impl Inner {
    fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed) + 1
    }
}

#[derive(Clone)]
pub struct Pubsub {
    url: Arc<str>,
    inner: Arc<Inner>,
}

impl Pubsub {
    pub fn connect(url: String) -> RpcResult<Self> {
        let runtime = tokio::runtime::Handle::try_current().map_err(|_| {
            request_error("PubsubClient::connect must be called within a tokio runtime")
        })?;
        let (out_tx, out_rx) = mpsc::unbounded();
        let inner = Arc::new(Inner {
            out_tx,
            pending: Mutex::new(HashMap::new()),
            subscriptions: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(0),
        });
        runtime.spawn(run(url.clone(), out_rx, Arc::downgrade(&inner)));
        Ok(Self {
            url: url.into(),
            inner,
        })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn subscribe<T: DeserializeOwned + 'static>(
        &self,
        subscribe_method: &'static str,
        unsubscribe_method: &'static str,
        params: Value,
    ) -> RpcResult<Subscription<T>> {
        let result = send_request(&self.inner, subscribe_method, params).await?;
        let id: u64 = serde_json::from_value(result)
            .map_err(|err| Box::new(RpcError::ParseError(err.to_string())))?;

        let (tx, rx) = mpsc::unbounded();
        self.inner.subscriptions.lock().unwrap().insert(id, tx);

        Ok(Subscription {
            id,
            unsubscribe_method,
            rx,
            inner: Arc::clone(&self.inner),
            _phantom: PhantomData,
        })
    }
}

/// Socket task. Holds only a `Weak` to the shared state, so it ends (closing
/// the socket) once every client and subscription is dropped and `out_rx`
/// drains.
async fn run(url: String, mut out_rx: mpsc::UnboundedReceiver<String>, inner: Weak<Inner>) {
    let ws = match tokio_tungstenite::connect_async(url.as_str()).await {
        Ok((ws, _)) => ws,
        Err(err) => {
            fail_all(&inner, &format!("websocket connect failed: {err}"));
            return;
        }
    };
    let (mut write, mut read) = ws.split();
    loop {
        tokio::select! {
            out = out_rx.next() => match out {
                Some(text) => {
                    if write.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
                None => break,
            },
            frame = read.next() => match frame {
                Some(Ok(Message::Text(text))) => {
                    if let (Some(inner), Ok(value)) =
                        (inner.upgrade(), serde_json::from_str::<Value>(&text))
                    {
                        dispatch_message(&inner, value);
                    }
                }
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => break,
                Some(Ok(_)) => {}
            },
        }
    }
    let _ = write.close().await;
    fail_all(&inner, "websocket connection closed");
}

/// Fail in-flight requests and live subscriptions so consumers can tell a
/// disconnect from a normal end-of-stream.
fn fail_all(inner: &Weak<Inner>, reason: &str) {
    let Some(inner) = inner.upgrade() else { return };
    for (_, tx) in inner.pending.lock().unwrap().drain() {
        let _ = tx.send(Err(request_error(reason)));
    }
    for (_, tx) in inner.subscriptions.lock().unwrap().drain() {
        let _ = tx.unbounded_send(Err(request_error(reason)));
    }
}

async fn send_request(inner: &Arc<Inner>, method: &str, params: Value) -> RpcResult<Value> {
    let id = inner.next_id();
    let body = json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });

    let (tx, rx) = oneshot::channel();
    inner.pending.lock().unwrap().insert(id, tx);
    if inner.out_tx.unbounded_send(body.to_string()).is_err() {
        inner.pending.lock().unwrap().remove(&id);
        return Err(request_error("websocket connection closed"));
    }
    rx.await
        .map_err(|_| request_error("websocket connection closed"))?
}

// Frames with an `id` answer our requests; frames with `params.subscription`
// are server-pushed notifications.
fn dispatch_message(inner: &Inner, value: Value) {
    if let Some(id) = value.get("id").and_then(Value::as_u64) {
        if let Some(tx) = inner.pending.lock().unwrap().remove(&id) {
            let response = match value.get("error").filter(|err| !err.is_null()) {
                Some(error) => Err(parse_rpc_error(error)),
                None => Ok(value.get("result").cloned().unwrap_or(Value::Null)),
            };
            let _ = tx.send(response);
        }
        return;
    }
    let Some(params) = value.get("params") else {
        return;
    };
    let Some(sub_id) = params.get("subscription").and_then(Value::as_u64) else {
        return;
    };
    let result = params.get("result").cloned().unwrap_or(Value::Null);
    if let Some(tx) = inner.subscriptions.lock().unwrap().get(&sub_id) {
        let _ = tx.unbounded_send(Ok(result));
    }
}

/// A live subscription that yields notifications as a [`Stream`].
///
/// Dropping it removes it from the dispatcher and best-effort sends the
/// matching `<x>Unsubscribe`.
pub struct Subscription<T> {
    id: u64,
    unsubscribe_method: &'static str,
    rx: mpsc::UnboundedReceiver<RpcResult<Value>>,
    inner: Arc<Inner>,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Subscription<T> {
    /// The server-assigned subscription id.
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl<T> std::fmt::Debug for Subscription<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("id", &self.id)
            .field("unsubscribe_method", &self.unsubscribe_method)
            .finish_non_exhaustive()
    }
}

impl<T: DeserializeOwned> Stream for Subscription<T> {
    type Item = RpcResult<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().rx.poll_next_unpin(cx).map(|item| {
            item.map(|res| {
                res.and_then(|value| {
                    serde_json::from_value(value)
                        .map_err(|err| Box::new(RpcError::ParseError(err.to_string())))
                })
            })
        })
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.inner.subscriptions.lock().unwrap().remove(&self.id);
        let body = json!({
            "jsonrpc": "2.0",
            "id": self.inner.next_id(),
            "method": self.unsubscribe_method,
            "params": [self.id],
        });
        let _ = self.inner.out_tx.unbounded_send(body.to_string());
    }
}

fn request_error(msg: &str) -> Box<RpcError> {
    Box::new(RpcError::RpcRequestError(msg.to_string()))
}

#[derive(Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    data: Option<Value>,
}

fn parse_rpc_error(error: &Value) -> Box<RpcError> {
    Box::new(
        match serde_json::from_value::<JsonRpcError>(error.clone()) {
            Ok(err) => {
                let data = err
                    .data
                    .as_ref()
                    .and_then(|d| d.get("numSlotsBehind"))
                    .and_then(Value::as_u64)
                    .map(|n| RpcResponseErrorData::NodeUnhealthy {
                        num_slots_behind: Some(n),
                    })
                    .unwrap_or(RpcResponseErrorData::Empty);
                RpcError::RpcResponseError {
                    code: err.code,
                    message: err.message,
                    data,
                }
            }
            Err(err) => RpcError::ParseError(err.to_string()),
        },
    )
}
