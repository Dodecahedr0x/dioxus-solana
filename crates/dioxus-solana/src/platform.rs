use dioxus_solana_core::{AppIdentity, Cluster, Wallet};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub type Registry = dioxus_solana_web::discovery::WalletRegistry;

#[cfg(target_os = "android")]
pub type Registry = crate::android::MwaRegistry;

#[cfg(target_os = "ios")]
pub type Registry = crate::ios::PhantomRegistry;

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub struct Registry;

#[cfg(target_arch = "wasm32")]
pub fn new_registry(cluster: Cluster, identity: Option<AppIdentity>) -> Rc<Registry> {
    Rc::new(Registry::new_with_identity(cluster, identity))
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn new_registry(cluster: Cluster, identity: Option<AppIdentity>) -> Rc<Registry> {
    Rc::new(Registry::new(cluster, identity))
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub fn new_registry(_cluster: Cluster, _identity: Option<AppIdentity>) -> Rc<Registry> {
    Rc::new(Registry)
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

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn run_discovery(registry: &Registry) -> Vec<Rc<dyn Wallet>> {
    registry.wallets()
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub fn run_discovery(_registry: &Registry) -> Vec<Rc<dyn Wallet>> {
    Vec::new()
}
