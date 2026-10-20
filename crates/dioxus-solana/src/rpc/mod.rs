//! Reactive Solana RPC providers and hooks.
//!
//! Mount an [`RpcProvider`] near the app root. Every hook and client below
//! works identically on web, Android and iOS (see [`RpcClient`] for the
//! per-platform transports).
//!
//! - **Fetch** — one `use_get_*` per JSON-RPC read method (and the generic
//!   [`use_fetch`]). Each does one HTTP request and returns a
//!   [`Resource`]`<Option<T>>`; read it with `res().flatten()`, refetch with
//!   `res.restart()`.
//! - **Subscription** — `use_*_subscription` (and the generic
//!   [`use_subscription`]) hold a live WebSocket and return a [`ReadSignal`]
//!   that updates on each push.
//! - **Live values** — [`use_account`], [`use_account_data`], [`use_balance`]
//!   fetch immediately *and* subscribe, so a value is available at once and then
//!   kept current.
//!
//! Mutating methods (`sendTransaction`, `requestAirdrop`) are deliberately *not*
//! hooks — auto-firing them on render would be a bug; use [`use_rpc`] from an
//! event handler instead.
//!
//! ```ignore
//! rsx! {
//!     RpcProvider { cluster: Cluster::Devnet,
//!         Body {}
//!     }
//! }
//! ```

mod accounts;
mod client;
mod fetch;
mod live;
mod methods;
mod pubsub_methods;
mod subscription;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use dioxus::prelude::*;
use dioxus_solana_core::Cluster;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_rpc_client_types::config::CommitmentConfig;

pub use accounts::*;
pub(crate) use client::sleep;
pub use client::{PubsubClient, RpcClient, RpcResult, Subscription};
pub use fetch::*;
pub use live::*;
pub use methods::CheckAddress;
pub use subscription::*;

pub use solana_account_decoder_client_types::UiAccountData;
pub use solana_epoch_info::EpochInfo;
pub use solana_epoch_schedule::EpochSchedule;
pub use solana_rpc_client_types::request::{RpcError, RpcRequest};
pub use solana_rpc_client_types::{config, filter, response};
/// Full transaction/block response types (`UiTransactionStatusMeta`,
/// `OptionSerializer`, …).
pub use solana_transaction_status_client_types as transaction_status;
pub use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, TransactionStatus,
};

/// A failed RPC request, as surfaced by the fetch hooks. Cheap to clone, so a
/// `Resource<Result<T, FetchError>>` can be read by value.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct FetchError {
    /// JSON-RPC error code, when the node answered with one.
    pub code: Option<i64>,
    pub message: String,
}

impl From<Box<RpcError>> for FetchError {
    fn from(err: Box<RpcError>) -> Self {
        let code = match &*err {
            RpcError::RpcResponseError { code, .. } => Some(*code),
            _ => None,
        };
        Self {
            code,
            message: err.to_string(),
        }
    }
}

/// Convenience readers for a fetch hook's `Resource<Result<T, FetchError>>`.
///
/// ```ignore
/// let version = use_get_version();
/// rsx! {
///     if let Some(v) = version.data() { "node v{v.solana_core}" }
///     if let Some(e) = version.error() { "unreachable: {e}" }
/// }
/// ```
pub trait FetchResult<T> {
    /// The value, once loaded successfully (reactive read).
    fn data(&self) -> Option<T>;
    /// The error, if the last fetch failed (reactive read).
    fn error(&self) -> Option<FetchError>;
}

impl<T: Clone + 'static> FetchResult<T> for Resource<Result<T, FetchError>> {
    fn data(&self) -> Option<T> {
        self.read().as_ref()?.as_ref().ok().cloned()
    }

    fn error(&self) -> Option<FetchError> {
        self.read().as_ref()?.as_ref().err().cloned()
    }
}

/// Account-info config requesting base64-encoded data, so [`UiAccountData::decode`]
/// yields raw bytes for the caller to deserialize.
pub fn base64_account_config() -> config::RpcAccountInfoConfig {
    config::RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        ..Default::default()
    }
}

/// Shared RPC state provided by [`RpcProvider`]: the cluster, one HTTP client
/// for it, and one lazily-opened PubSub socket that every subscription hook
/// multiplexes over.
///
/// Every hook reads the cluster reactively, so switching it (by re-rendering
/// `RpcProvider` with a different `cluster`) refetches and resubscribes
/// everything underneath.
#[derive(Clone, Copy)]
pub struct RpcContext {
    pub cluster: Signal<Cluster>,
    client: Memo<RpcClient>,
    /// Bumped by [`invalidate`](Self::invalidate); every fetch hook reads it.
    generation: Signal<u64>,
    pubsub: CopyValue<(u64, Option<PubsubClient>)>,
}

impl RpcContext {
    /// The HTTP client for the current cluster. Reactive: reading it from a
    /// component or hook subscribes to cluster changes.
    pub fn client(&self) -> RpcClient {
        self.client.cloned()
    }

    /// Refetch every mounted fetch hook (`use_get_*`, `use_fetch*`, …) — e.g.
    /// after a transaction lands, so balances and history everywhere catch up.
    /// Subscriptions and live hooks are already current and are left alone.
    pub fn invalidate(&self) {
        let mut generation = self.generation;
        *generation.write() += 1;
    }

    /// Reactive read of the endpoint settings (cluster, commitment), for
    /// subscription hooks.
    pub(crate) fn track_endpoint(&self) {
        self.client.read();
    }

    /// Reactive read of the invalidation counter, for fetch hooks.
    pub(crate) fn track_invalidation(&self) {
        self.generation.read();
    }

    /// The shared PubSub connection for the current cluster, opened on first use,
    /// tagged with a generation for [`reset_pubsub`](Self::reset_pubsub).
    /// Not reactive — callers read the cluster themselves.
    pub(crate) fn pubsub(&self) -> RpcResult<(u64, PubsubClient)> {
        let ws_url = self.cluster.peek().ws_url().to_string();
        let commitment = self.client.peek().commitment();
        let mut slot = self.pubsub;
        if let (generation, Some(client)) = &*slot.peek() {
            if client.url() == ws_url && client.commitment() == commitment {
                return Ok((*generation, client.clone()));
            }
        }
        let mut client = PubsubClient::connect(ws_url)?;
        if let Some(commitment) = commitment {
            client = client.with_commitment(commitment);
        }
        let mut slot = slot.write();
        slot.0 += 1;
        slot.1 = Some(client.clone());
        Ok((slot.0, client))
    }

    /// Forget the shared PubSub connection of `generation` (it dropped) so the
    /// next [`pubsub`](Self::pubsub) call reconnects. A no-op if another hook
    /// already replaced it.
    pub(crate) fn reset_pubsub(&self, generation: u64) {
        let mut slot = self.pubsub;
        if slot.peek().0 == generation {
            slot.write().1 = None;
        }
    }
}

/// Props for [`RpcProvider`]. `rpc_url` / `ws_url` override the cluster defaults.
#[derive(Props, Clone, PartialEq)]
pub struct RpcProviderProps {
    #[props(default = Cluster::MainnetBeta)]
    pub cluster: Cluster,
    #[props(optional)]
    pub rpc_url: Option<String>,
    #[props(optional)]
    pub ws_url: Option<String>,
    /// Commitment for every request and subscription that does not set its
    /// own. Defaults to `confirmed` — fresher than the node's `finalized`
    /// default and what [`RpcClient::confirm_transaction`] waits for, so
    /// balances and history reflect a transaction as soon as it confirms.
    #[props(default = CommitmentConfig::confirmed())]
    pub commitment: CommitmentConfig,
    pub children: Element,
}

/// Provides the cluster (RPC/WS endpoints) to descendants. Usable on its own,
/// and a nested [`WalletProvider`] reads its cluster from here — so this is the
/// single place to choose the network.
///
/// The props are reactive: pass a different `cluster` (e.g. from a cluster
/// picker's signal) and every hook below refetches / resubscribes against it.
///
/// [`WalletProvider`]: crate::WalletProvider
#[component]
pub fn RpcProvider(props: RpcProviderProps) -> Element {
    // Fold any URL overrides into the cluster so it stays the single source of
    // endpoint truth (mirroring `WalletProvider`'s `Signal<Cluster>`).
    let resolved = if props.rpc_url.is_none() && props.ws_url.is_none() {
        props.cluster.clone()
    } else {
        Cluster::custom(
            props.cluster.clone(),
            props
                .rpc_url
                .clone()
                .unwrap_or_else(|| props.cluster.rpc_url().to_string()),
            props
                .ws_url
                .clone()
                .unwrap_or_else(|| props.cluster.ws_url().to_string()),
        )
    };
    let cluster = use_dep(resolved);
    let commitment = use_dep(props.commitment);
    let client =
        use_memo(move || RpcClient::new(cluster.read().rpc_url()).with_commitment(commitment()));
    let pubsub = use_hook(|| CopyValue::new((0, None)));
    let generation = use_signal(|| 0);
    use_context_provider(|| RpcContext {
        cluster,
        client,
        generation,
        pubsub,
    });
    rsx! { {props.children} }
}

/// Track a plain value across renders as a signal: it is written whenever the
/// caller passes something different, so hooks reading it re-run.
pub(crate) fn use_dep<D: Clone + PartialEq + 'static>(value: D) -> Signal<D> {
    let mut dep = use_signal(|| value.clone());
    if *dep.peek() != value {
        dep.set(value);
    }
    dep
}

/// The HTTP RPC client for the current cluster, for arbitrary calls — including
/// mutating methods like `send_transaction` / `request_airdrop`, which have no
/// auto-firing hook.
///
/// The calling component re-renders when the cluster changes, so handlers that
/// capture the returned client always talk to the current cluster.
pub fn use_rpc() -> RpcClient {
    use_context::<RpcContext>().client()
}

/// The provider's [`RpcContext`] — for [`invalidate`](RpcContext::invalidate)
/// after a mutation, or reading the current cluster.
pub fn use_rpc_context() -> RpcContext {
    use_context::<RpcContext>()
}
