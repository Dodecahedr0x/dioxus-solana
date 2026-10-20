//! Solana wallets and reactive RPC for Dioxus apps on **web, Android and iOS**.
//!
//! Desktop is not supported: there is no wallet to connect to, so building for a
//! desktop target is a compile error. Enable the `server` feature only for the
//! server half of a fullstack web app (RPC works there; wallets do not).

#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "android",
    target_os = "ios",
    feature = "server",
    test,
    docsrs
)))]
compile_error!(
    "dioxus-solana supports web (wasm32), Android and iOS only; desktop is not supported. \
     Build with `dx serve --platform web|android|ios`. For the server half of a fullstack \
     web app, enable the `server` feature."
);

mod components;
pub mod format;
mod hooks;
#[allow(dead_code)]
mod native_mwa;
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
mod native_phantom;
mod platform;
mod provider;
mod state;
mod transaction;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

pub mod rpc;
#[cfg(target_arch = "wasm32")]
pub mod storage;

pub use native_mwa::MOBILE_WALLET_NAME;

pub use components::WalletButton;
pub use dioxus_solana_core::{
    deserialize_transaction, serialize_transaction, AppIdentity, Cluster, ConnectedAccount,
    CustomCluster, Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use hooks::{use_wallet, WalletHandle};
pub use provider::{WalletContext, WalletProvider, WalletProviderProps};
pub use solana_hash::Hash;
pub use solana_instruction::{AccountMeta, Instruction};
pub use solana_pubkey::Pubkey;
pub use solana_signature::Signature;
pub use solana_transaction::versioned::{TransactionVersion, VersionedTransaction};
pub use state::WalletState;
pub use transaction::{build_transaction, SendError};
