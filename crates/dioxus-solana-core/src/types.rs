use solana_pubkey::Pubkey;
use std::rc::Rc;

use crate::wallet::WalletSigner;

/// Metadata about a discovered wallet (pre-connection), for building a picker UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletInfo {
    pub name: String,
    /// data-URI icon, if the wallet provided one.
    pub icon: Option<String>,
    /// Feature keys the wallet advertises, e.g. `"solana:signTransaction"`.
    pub features: Vec<String>,
}

/// An active connection to a chosen account.
#[derive(Clone)]
pub struct ConnectedAccount {
    pub wallet_name: String,
    pub signer: Rc<dyn WalletSigner>,
}

impl ConnectedAccount {
    pub fn pubkey(&self) -> Pubkey {
        self.signer.pubkey()
    }
}
