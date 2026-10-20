//! Cross-platform Solana RPC clients.
//!
//! [`RpcClient`] (HTTP) and [`PubsubClient`] (WebSocket) expose the same API on
//! every supported platform; only the transport differs:
//!
//! - **wasm** — spume's browser `fetch` / `WebSocket` providers
//! - **Android / iOS** — Agave's `reqwest` client for HTTP and a tokio
//!   WebSocket for PubSub (dioxus mobile runs inside a tokio runtime)

use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use solana_rpc_client_types::config::{CommitmentConfig, CommitmentLevel};
use solana_rpc_client_types::request::{RpcError, RpcRequest};

#[cfg(not(target_arch = "wasm32"))]
use super::native as transport;
#[cfg(target_arch = "wasm32")]
use super::wasm as transport;

pub(crate) use transport::sleep;
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
    commitment: Option<CommitmentLevel>,
}

/// HTTP methods whose last positional parameter is a config object accepting
/// `commitment`. The second list rejects `processed` (it is raised to
/// `confirmed` there).
const COMMITMENT_METHODS: &[&str] = &[
    "getAccountInfo",
    "getBalance",
    "getBlockHeight",
    "getBlockProduction",
    "getEpochInfo",
    "getFeeForMessage",
    "getInflationGovernor",
    "getInflationReward",
    "getLargestAccounts",
    "getLatestBlockhash",
    "getLeaderSchedule",
    "getMinimumBalanceForRentExemption",
    "getMultipleAccounts",
    "getProgramAccounts",
    "getSlot",
    "getSlotLeader",
    "getStakeMinimumDelegation",
    "getSupply",
    "getTokenAccountBalance",
    "getTokenAccountsByDelegate",
    "getTokenAccountsByOwner",
    "getTokenLargestAccounts",
    "getTokenSupply",
    "getTransactionCount",
    "getVoteAccounts",
    "isBlockhashValid",
    "requestAirdrop",
    "simulateTransaction",
    // PubSub
    "accountSubscribe",
    "programSubscribe",
    "logsSubscribe",
    "signatureSubscribe",
];
const CONFIRMED_MIN_METHODS: &[&str] = &[
    "getBlock",
    "getBlocks",
    "getBlocksWithLimit",
    "getSignaturesForAddress",
    "getTransaction",
    "blockSubscribe",
];

const RATE_LIMIT_RETRIES: usize = 5;
const RATE_LIMIT_BACKOFF: Duration = Duration::from_millis(500);

fn is_rate_limited(err: &RpcError) -> bool {
    match err {
        RpcError::RpcResponseError { code, .. } => *code == 429,
        RpcError::RpcRequestError(msg) => msg.contains("429"),
        _ => false,
    }
}

/// Fill `commitment` into the trailing config of `params` when the caller left
/// it unset. Explicit commitments are never overridden.
fn apply_commitment(method: &str, params: &mut Value, commitment: Option<CommitmentLevel>) {
    let Some(mut level) = commitment else { return };
    // Preflight must simulate against a bank that knows the blockhash, which
    // was fetched at the default commitment.
    let mut key = "commitment";
    if method == "sendTransaction" {
        key = "preflightCommitment";
    } else if CONFIRMED_MIN_METHODS.contains(&method) {
        if level == CommitmentLevel::Processed {
            level = CommitmentLevel::Confirmed;
        }
    } else if !COMMITMENT_METHODS.contains(&method) {
        return;
    }
    let Some(config) = params.as_array_mut().and_then(|p| p.last_mut()) else {
        return;
    };
    match config {
        Value::Null => *config = json!({ key: level }),
        // Typed configs serialize an unset option as `null`.
        Value::Object(map) if map.get(key).is_none_or(Value::is_null) => {
            map.insert(key.into(), json!(level));
        }
        _ => {}
    }
}

impl RpcClient {
    /// Client for the JSON-RPC HTTP endpoint at `url`.
    pub fn new(url: impl ToString) -> Self {
        let url = url.to_string();
        let http = transport::Http::new(&url);
        Self {
            url,
            http,
            commitment: None,
        }
    }

    /// Use `commitment` for every request that does not set its own (the node
    /// default is `finalized`). [`RpcProvider`](super::RpcProvider) applies its
    /// `commitment` prop this way.
    pub fn with_commitment(mut self, commitment: CommitmentConfig) -> Self {
        self.commitment = Some(commitment.commitment);
        self
    }

    /// The default commitment set by [`with_commitment`](Self::with_commitment).
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.commitment
            .map(|commitment| CommitmentConfig { commitment })
    }

    /// The endpoint URL this client was constructed with.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Send a raw JSON-RPC request. `params` must be a JSON array.
    ///
    /// Rate-limited requests (HTTP 429, common on public endpoints) are retried
    /// with exponential backoff, like web3.js' `Connection`.
    pub async fn send<R: DeserializeOwned>(
        &self,
        request: RpcRequest,
        mut params: Value,
    ) -> RpcResult<R> {
        apply_commitment(request.as_str(), &mut params, self.commitment);
        let mut delay = RATE_LIMIT_BACKOFF;
        for _ in 0..RATE_LIMIT_RETRIES {
            match self.http.send(request, params.clone()).await {
                Err(err) if is_rate_limited(&err) => {
                    sleep(delay).await;
                    delay *= 2;
                }
                result => return result,
            }
        }
        self.http.send(request, params).await
    }
}

/// Clients are equal when they target the same endpoint.
impl PartialEq for RpcClient {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.commitment == other.commitment
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
    commitment: Option<CommitmentLevel>,
}

impl PubsubClient {
    /// Open a connection to the PubSub WebSocket endpoint at `url`.
    ///
    /// On Android/iOS this must be called from within a tokio runtime (any
    /// dioxus component or `spawn`ed task is).
    pub fn connect(url: impl ToString) -> RpcResult<Self> {
        Ok(Self {
            pubsub: transport::Pubsub::connect(url.to_string())?,
            commitment: None,
        })
    }

    /// The default commitment set by [`with_commitment`](Self::with_commitment).
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.commitment
            .map(|commitment| CommitmentConfig { commitment })
    }

    /// Use `commitment` for every subscription that does not set its own.
    pub fn with_commitment(mut self, commitment: CommitmentConfig) -> Self {
        self.commitment = Some(commitment.commitment);
        self
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
        mut params: Value,
    ) -> RpcResult<Subscription<T>> {
        apply_commitment(subscribe_method, &mut params, self.commitment);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_missing_commitment_only() {
        let confirmed = Some(CommitmentLevel::Confirmed);
        let mut p = json!(["addr", null]);
        apply_commitment("getBalance", &mut p, confirmed);
        assert_eq!(p, json!(["addr", { "commitment": "confirmed" }]));

        let mut p = json!(["addr", { "encoding": "base64" }]);
        apply_commitment("accountSubscribe", &mut p, confirmed);
        assert_eq!(
            p,
            json!(["addr", { "encoding": "base64", "commitment": "confirmed" }])
        );

        let mut p = json!(["addr", { "commitment": "finalized" }]);
        apply_commitment("getBalance", &mut p, confirmed);
        assert_eq!(p, json!(["addr", { "commitment": "finalized" }]));

        let mut p = json!(["tx", { "encoding": "base64", "preflightCommitment": null }]);
        apply_commitment("sendTransaction", &mut p, confirmed);
        assert_eq!(
            p,
            json!(["tx", { "encoding": "base64", "preflightCommitment": "confirmed" }])
        );

        let mut p = json!([42]);
        apply_commitment("getBlockTime", &mut p, confirmed);
        assert_eq!(p, json!([42]));

        let mut p = json!(["sig", null]);
        apply_commitment("getTransaction", &mut p, Some(CommitmentLevel::Processed));
        assert_eq!(p, json!(["sig", { "commitment": "confirmed" }]));
    }
}
