mod hooks;
#[allow(dead_code)]
mod native_mwa;
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
mod native_phantom;
mod platform;
mod provider;
mod state;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_arch = "wasm32")]
pub mod rpc;
#[cfg(target_arch = "wasm32")]
pub mod storage;

pub use native_mwa::MOBILE_WALLET_NAME;

pub use dioxus_solana_core::{
    deserialize_transaction, serialize_transaction, AppIdentity, Cluster, ConnectedAccount,
    CustomCluster, Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use hooks::{use_wallet, WalletHandle};
pub use provider::{WalletContext, WalletProvider, WalletProviderProps};
pub use solana_pubkey::Pubkey;
pub use state::WalletState;
