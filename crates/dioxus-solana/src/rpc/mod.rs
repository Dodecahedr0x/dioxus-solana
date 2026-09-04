//! Reactive Solana RPC providers and hooks.
//!
//! Mount an [`RpcProvider`] near the app root. On wasm, hooks are backed by
//! [`spume`]; on native, [`use_rpc`] returns Agave's
//! [`solana_rpc_client::nonblocking::rpc_client::RpcClient`].
//!
//! Wasm-only hook families:
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

#[cfg(target_arch = "wasm32")]
mod fetch;
#[cfg(target_arch = "wasm32")]
mod live;
#[cfg(target_arch = "wasm32")]
mod subscription;

use dioxus::prelude::*;
use dioxus_solana_core::Cluster;

#[cfg(target_arch = "wasm32")]
pub use dioxus_solana_web::rpc::{
    config, response, spume, EncodedConfirmedTransactionWithStatusMeta, EpochInfo, EpochSchedule,
    TransactionStatus, WasmClient,
};

#[cfg(target_arch = "wasm32")]
pub use fetch::*;
#[cfg(target_arch = "wasm32")]
pub use live::*;
#[cfg(target_arch = "wasm32")]
pub use subscription::*;

#[cfg(not(target_arch = "wasm32"))]
pub use solana_rpc_client::nonblocking::rpc_client::RpcClient;

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

/// Cloneable HTTP RPC client for arbitrary calls (including mutating methods
/// like `send_transaction` / `request_airdrop`, which have no auto-firing hook).
///
/// - **wasm** — spume [`WasmClient`]
/// - **native** — Agave [`RpcClient`] behind an [`Arc`]
#[cfg(target_arch = "wasm32")]
pub fn use_rpc() -> WasmClient {
    let ctx = use_context::<RpcContext>();
    use_hook(|| WasmClient::new(ctx.cluster.peek().rpc_url().to_string()))
}

/// Cloneable HTTP RPC client for arbitrary calls.
///
/// Native builds use Agave's nonblocking [`RpcClient`].
#[cfg(not(target_arch = "wasm32"))]
pub fn use_rpc() -> std::sync::Arc<RpcClient> {
    let ctx = use_context::<RpcContext>();
    use_hook(|| std::sync::Arc::new(RpcClient::new(ctx.cluster.peek().rpc_url().to_string())))
}
