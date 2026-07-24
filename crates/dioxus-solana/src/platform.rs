use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub fn discover(cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    dioxus_solana_web::discover(cluster)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn discover(_cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    // Desktop connector not yet implemented; keeps the facade compiling off-wasm.
    Vec::new()
}
