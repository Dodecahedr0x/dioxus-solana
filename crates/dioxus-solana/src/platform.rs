use dioxus_solana_core::{AppIdentity, Cluster, Wallet};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub type Registry = dioxus_solana_web::discovery::WalletRegistry;

#[cfg(target_arch = "wasm32")]
pub fn new_registry(cluster: Cluster, identity: Option<AppIdentity>) -> Rc<Registry> {
    Rc::new(Registry::new_with_identity(cluster, identity))
}

/// Run the discovery handshake and snapshot the currently-registered wallets.
#[cfg(target_arch = "wasm32")]
pub fn run_discovery(registry: &Registry) -> Vec<Rc<dyn Wallet>> {
    registry.discover();
    registry
        .wallets()
        .into_iter()
        .map(|w| Rc::new(w) as Rc<dyn Wallet>)
        .collect()
}

// --- Desktop seam: no connector yet, so discovery yields nothing. ---
#[cfg(not(target_arch = "wasm32"))]
pub struct Registry;

#[cfg(not(target_arch = "wasm32"))]
pub fn new_registry(_cluster: Cluster, _identity: Option<AppIdentity>) -> Rc<Registry> {
    Rc::new(Registry)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_discovery(_registry: &Registry) -> Vec<Rc<dyn Wallet>> {
    Vec::new()
}
