mod platform;
mod state;

#[cfg(target_arch = "wasm32")]
pub mod storage;

pub use dioxus_solana_core::{
    Cluster, ConnectedAccount, Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use state::WalletState;
