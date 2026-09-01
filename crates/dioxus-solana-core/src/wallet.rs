use crate::{types::ConnectedAccount, WalletError, WalletInfo};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

/// A discovered wallet, before connection.
#[async_trait::async_trait(?Send)]
pub trait Wallet {
    fn info(&self) -> &WalletInfo;
    async fn connect(&self) -> Result<ConnectedAccount, WalletError>;

    /// Reconnect without prompting the user.
    ///
    /// Mobile Wallet Adapter uses this for autoconnect: a cached authorization
    /// is restored, and if there is none the call returns [`WalletError::Disconnected`]
    /// instead of opening the wallet. The default forwards to [`Wallet::connect`].
    async fn connect_silent(&self) -> Result<ConnectedAccount, WalletError> {
        self.connect().await
    }
}

/// An active signing session.
#[async_trait::async_trait(?Send)]
pub trait WalletSigner {
    fn pubkey(&self) -> Pubkey;
    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError>;
    async fn sign_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<VersionedTransaction, WalletError>;
    async fn sign_and_send_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<Signature, WalletError>;
    async fn disconnect(&self) -> Result<(), WalletError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WalletError;
    use solana_pubkey::Pubkey;
    use solana_signature::Signature;
    use solana_transaction::versioned::VersionedTransaction;
    use std::rc::Rc;

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

    #[tokio::test]
    async fn signer_is_object_safe_and_usable() {
        let signer: Rc<dyn WalletSigner> = Rc::new(MockSigner(Pubkey::new_from_array([9u8; 32])));
        let account = ConnectedAccount {
            wallet_name: "Mock".into(),
            signer,
        };
        assert_eq!(account.pubkey(), Pubkey::new_from_array([9u8; 32]));
        assert_eq!(
            account.signer.sign_message(b"hi").await.unwrap(),
            Signature::from([1u8; 64])
        );
    }

    struct MockWallet(WalletInfo);

    #[async_trait::async_trait(?Send)]
    impl Wallet for MockWallet {
        fn info(&self) -> &WalletInfo {
            &self.0
        }

        async fn connect(&self) -> Result<ConnectedAccount, WalletError> {
            Ok(ConnectedAccount {
                wallet_name: self.0.name.clone(),
                signer: Rc::new(MockSigner(Pubkey::new_from_array([9u8; 32]))),
            })
        }
    }

    #[tokio::test]
    async fn wallet_is_object_safe_and_usable() {
        let info = WalletInfo {
            name: "MockWallet".into(),
            icon: None,
            features: vec![],
        };
        let wallet: Rc<dyn Wallet> = Rc::new(MockWallet(info));
        assert_eq!(wallet.info().name, "MockWallet");
        let account = wallet.connect().await.unwrap();
        assert_eq!(account.wallet_name, "MockWallet");
    }

    #[tokio::test]
    async fn connect_silent_defaults_to_connect() {
        let info = WalletInfo {
            name: "MockWallet".into(),
            icon: None,
            features: vec![],
        };
        let wallet: Rc<dyn Wallet> = Rc::new(MockWallet(info));
        let account = wallet.connect_silent().await.unwrap();
        assert_eq!(account.wallet_name, "MockWallet");
    }
}
