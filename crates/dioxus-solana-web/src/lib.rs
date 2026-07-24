// crates/dioxus-solana-web/src/lib.rs
pub mod discovery;
pub mod js;
pub mod signer;
pub mod wallet;

use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

/// Discover installed Wallet-Standard wallets as core trait objects.
///
/// Runs the window handshake synchronously and returns whatever registered.
/// Call again to re-snapshot (e.g. after a wallet loads late).
pub fn discover(cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    let registry = discovery::WalletRegistry::new(cluster);
    registry.discover();
    registry
        .wallets()
        .into_iter()
        .map(|w| Rc::new(w) as Rc<dyn Wallet>)
        .collect()
}
