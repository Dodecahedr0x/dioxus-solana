//! Cross-platform Solana RPC clients.
//!
//! [`RpcClient`] (HTTP) and [`PubsubClient`] (WebSocket) expose the same API on
//! every supported platform; only the transport differs:
//!
//! - **wasm** — spume's browser `fetch` / `WebSocket` providers
//! - **Android / iOS** — Agave's `reqwest` client for HTTP and a tokio
//!   WebSocket for PubSub (dioxus mobile runs inside a tokio runtime)

use serde::de::DeserializeOwned;
use serde_json::Value;
use solana_rpc_client_types::request::{RpcError, RpcRequest};

#[cfg(not(target_arch = "wasm32"))]
use super::native as transport;
#[cfg(target_arch = "wasm32")]
use super::wasm as transport;

pub use transport::Subscription;

/// Result of an RPC call.
pub type RpcResult<T> = Result<T, Box<RpcError>>;

/// Solana JSON-RPC client over HTTP.
///
/// Cheap to construct (no connection is opened until the first request) and
/// cheap to clone. One typed method per JSON-RPC method (`get_balance`,
/// `send_transaction`, …); [`send`](Self::send) covers anything else.
#[derive(Clone)]
pub struct RpcClient {
    url: String,
    http: transport::Http,
}

impl RpcClient {
    /// Client for the JSON-RPC HTTP endpoint at `url`.
    pub fn new(url: impl ToString) -> Self {
        let url = url.to_string();
        let http = transport::Http::new(&url);
        Self { url, http }
    }

    /// The endpoint URL this client was constructed with.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Send a raw JSON-RPC request. `params` must be a JSON array.
    pub async fn send<R: DeserializeOwned>(
        &self,
        request: RpcRequest,
        params: Value,
    ) -> RpcResult<R> {
        self.http.send(request, params).await
    }
}

impl std::fmt::Debug for RpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcClient").field("url", &self.url).finish()
    }
}

/// Solana JSON-RPC PubSub client over a single WebSocket connection.
///
/// [`connect`](Self::connect) returns immediately; requests queue until the
/// socket opens. Subscriptions share the socket, which closes once the client
/// and every [`Subscription`] are dropped.
#[derive(Clone)]
pub struct PubsubClient {
    pubsub: transport::Pubsub,
}

impl PubsubClient {
    /// Open a connection to the PubSub WebSocket endpoint at `url`.
    ///
    /// On Android/iOS this must be called from within a tokio runtime (any
    /// dioxus component or `spawn`ed task is).
    pub fn connect(url: impl ToString) -> RpcResult<Self> {
        Ok(Self {
            pubsub: transport::Pubsub::connect(url.to_string())?,
        })
    }

    /// The endpoint URL this client was opened with.
    pub fn url(&self) -> &str {
        self.pubsub.url()
    }

    /// Issue a raw `<x>Subscribe` request; dropping the returned
    /// [`Subscription`] sends `unsubscribe_method`.
    pub async fn subscribe<T: DeserializeOwned + 'static>(
        &self,
        subscribe_method: &'static str,
        unsubscribe_method: &'static str,
        params: Value,
    ) -> RpcResult<Subscription<T>> {
        self.pubsub
            .subscribe(subscribe_method, unsubscribe_method, params)
            .await
    }
}

impl std::fmt::Debug for PubsubClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PubsubClient")
            .field("url", &self.url())
            .finish()
    }
}
