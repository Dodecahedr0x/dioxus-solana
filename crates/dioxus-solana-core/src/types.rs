use solana_pubkey::Pubkey;
use std::fmt;
use std::rc::Rc;

use crate::wallet::WalletSigner;

/// Identity presented to a wallet during Mobile Wallet Adapter authorization.
///
/// `icon` must be a **relative** URI (resolved against `uri`). Absolute URLs are
/// rejected by MWA 2.0 wallets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppIdentity {
    pub name: String,
    /// Origin of the dapp, e.g. `"https://example.com"`. Empty means "use the
    /// current page origin" when MWA is registered in the browser.
    pub uri: String,
    /// Relative icon path, e.g. `"favicon.ico"`.
    pub icon: String,
}

impl AppIdentity {
    /// Name plus the usual `favicon.ico` icon. The URI is filled from the page
    /// origin at registration time when left empty.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uri: String::new(),
            icon: "favicon.ico".into(),
        }
    }
}

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
        async fn sign_transaction(
            &self,
            tx: VersionedTransaction,
        ) -> Result<VersionedTransaction, WalletError> {
            Ok(tx)
        }
        async fn sign_and_send_transaction(
            &self,
            _tx: VersionedTransaction,
        ) -> Result<Signature, WalletError> {
            Ok(Signature::from([2u8; 64]))
        }
        async fn disconnect(&self) -> Result<(), WalletError> {
            Ok(())
        }
    }

    #[test]
    fn named_identity_defaults_icon_and_empty_uri() {
        let id = AppIdentity::named("Demo");
        assert_eq!(id.name, "Demo");
        assert!(id.uri.is_empty());
        assert_eq!(id.icon, "favicon.ico");
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
