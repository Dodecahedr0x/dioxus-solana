use dioxus::prelude::*;
use dioxus_solana_core::{WalletError, WalletInfo};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

use crate::{provider::WalletContext, state::WalletState};

/// Cheap `Copy` handle over the wallet context. Get it with [`use_wallet`].
#[derive(Clone, Copy)]
pub struct WalletHandle {
    ctx: WalletContext,
}

/// Access wallet state + actions from any descendant of `WalletProvider`.
pub fn use_wallet() -> WalletHandle {
    WalletHandle {
        ctx: use_context::<WalletContext>(),
    }
}

impl WalletHandle {
    pub fn state(&self) -> WalletState {
        (self.ctx.state)()
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        (self.ctx.state)().pubkey()
    }

    pub fn wallets(&self) -> Vec<WalletInfo> {
        (self.ctx.wallets)()
            .iter()
            .map(|w| w.info().clone())
            .collect()
    }

    pub fn cluster(&self) -> dioxus_solana_core::Cluster {
        (self.ctx.cluster)()
    }

    /// Connect a wallet by name (as reported in `wallets()`).
    pub fn connect(&self, name: String) {
        let mut state = self.ctx.state;
        let wallets = self.ctx.wallets;
        spawn(async move {
            let Some(w) = wallets().iter().find(|w| w.info().name == name).cloned() else {
                state.set(WalletState::Error(WalletError::NotInstalled));
                return;
            };
            state.set(WalletState::Connecting);
            match w.connect().await {
                Ok(acc) => {
                    #[cfg(target_arch = "wasm32")]
                    crate::storage::set_last_wallet(&acc.wallet_name);
                    state.set(WalletState::Connected(acc));
                }
                Err(e) => state.set(WalletState::Error(e)),
            }
        });
    }

    pub fn disconnect(&self) {
        let mut state = self.ctx.state;
        spawn(async move {
            if let WalletState::Connected(acc) = state() {
                let _ = acc.signer.disconnect().await;
            }
            #[cfg(target_arch = "wasm32")]
            crate::storage::clear_last_wallet();
            state.set(WalletState::Disconnected);
        });
    }

    /// Await a message signature. Returns `Disconnected` if not connected.
    pub async fn sign_message(&self, msg: Vec<u8>) -> Result<Signature, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_message(&msg).await,
            _ => Err(WalletError::Disconnected),
        }
    }

    pub async fn sign_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<VersionedTransaction, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_transaction(tx).await,
            _ => Err(WalletError::Disconnected),
        }
    }

    pub async fn sign_and_send(&self, tx: VersionedTransaction) -> Result<Signature, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_and_send_transaction(tx).await,
            _ => Err(WalletError::Disconnected),
        }
    }
}
