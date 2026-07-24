mod hooks;
mod platform;
mod provider;
mod state;

#[cfg(target_arch = "wasm32")]
pub mod storage;

pub use dioxus_solana_core::{
    Cluster, ConnectedAccount, Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use hooks::{use_wallet, WalletHandle};
pub use provider::{WalletContext, WalletProvider, WalletProviderProps};
pub use state::WalletState;
