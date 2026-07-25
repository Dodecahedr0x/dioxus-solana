//! Reactive Solana RPC providers and hooks, backed by [`spume`].
//!
//! Mount an [`RpcProvider`] near the app root, then call the hooks from any
//! descendant. Hooks come in three families:
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
//! Result/config types are re-exported here (flat, and via the [`config`] /
//! [`response`] modules). Mutating methods (`sendTransaction`, `requestAirdrop`)
//! are deliberately *not* hooks — auto-firing them on render would be a bug; use
//! [`use_rpc`] from an event handler instead.
//!
//! ```ignore
//! rsx! {
//!     RpcProvider { cluster: Cluster::Devnet,
//!         Body {}
//!     }
//! }
//!
//! // inside Body:
//! let slot = use_slot_subscription();  // ReadSignal<Option<u64>>
//! let health = use_get_health();       // Resource<Option<String>>
//! ```

mod fetch;
mod live;
mod subscription;

use dioxus::prelude::*;
use dioxus_solana_core::Cluster;

// Re-exported so consumers can name what the hooks return/take. The `config` and
// `response` modules carry the long tail of `Rpc*` types.
pub use dioxus_solana_web::rpc::{
    config, response, spume, EncodedConfirmedTransactionWithStatusMeta, EpochInfo, EpochSchedule,
    TransactionStatus, WasmClient,
};

pub use fetch::*;
pub use live::*;
pub use subscription::*;

/// The resolved cluster shared with descendant RPC hooks. Provided by
/// [`RpcProvider`]; hooks read its endpoints via [`Cluster::rpc_url`] /
/// [`Cluster::ws_url`].
#[derive(Clone, Copy)]
pub struct RpcContext {
    pub cluster: Signal<Cluster>,
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
    pub children: Element,
}

/// Provides the cluster (RPC/WS endpoints) to descendants. Usable on its own,
/// and a nested [`WalletProvider`] reads its cluster from here — so this is the
/// single place to choose the network.
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
    let cluster = use_signal(|| resolved);
    use_context_provider(|| RpcContext { cluster });
    rsx! { {props.children} }
}

/// A cloneable spume HTTP client for arbitrary calls (including the mutating
/// `send_transaction` / `request_airdrop`, which have no auto-firing hook).
pub fn use_rpc() -> WasmClient {
    let ctx = use_context::<RpcContext>();
    // Build once on mount, not per render.
    use_hook(|| WasmClient::new(ctx.cluster.peek().rpc_url().to_string()))
}
