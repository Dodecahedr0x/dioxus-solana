use solana_pubkey::Pubkey;
use std::fmt;
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

impl fmt::Debug for ConnectedAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectedAccount")
            .field("wallet_name", &self.wallet_name)
            .field("pubkey", &self.pubkey())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WalletError;
    use solana_signature::Signature;
    use solana_transaction::versioned::VersionedTransaction;

    struct MockSigner(Pubkey);

    #[async_trait::async_trait(?Send)]
    impl WalletSigner for MockSigner {
        fn pubkey(&self) -> Pubkey {
            self.0
        }
        async fn sign_message(&self, _m: &[u8]) -> Result<Signature, WalletError> {
            Ok(Signature::from([1u8; 64]))
        }
        async fn sign_transaction(&self, tx: VersionedTransaction) -> Result<VersionedTransaction, WalletError> {
            Ok(tx)
        }
        async fn sign_and_send_transaction(&self, _tx: VersionedTransaction) -> Result<Signature, WalletError> {
            Ok(Signature::from([2u8; 64]))
        }
        async fn disconnect(&self) -> Result<(), WalletError> {
            Ok(())
        }
    }

    #[test]
    fn debug_impl_includes_wallet_name() {
        let account = ConnectedAccount {
            wallet_name: "Mock".into(),
            signer: Rc::new(MockSigner(Pubkey::new_from_array([9u8; 32]))),
        };
        assert!(format!("{account:?}").contains("Mock"));
    }
}
