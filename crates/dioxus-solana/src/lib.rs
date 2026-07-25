mod hooks;
mod platform;
mod provider;
mod state;

pub mod rpc;
#[cfg(target_arch = "wasm32")]
pub mod storage;

pub use dioxus_solana_core::{
    deserialize_transaction, serialize_transaction, Cluster, ConnectedAccount, CustomCluster,
    Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use hooks::{use_wallet, WalletHandle};
pub use provider::{WalletContext, WalletProvider, WalletProviderProps};
pub use solana_pubkey::Pubkey;
pub use state::WalletState;
