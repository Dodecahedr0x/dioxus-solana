use crate::native_mwa::{
    decode_b64, encode_b64, error_from_status, pubkey_from_b64, signature_from_b64, wallet_info,
};
use dioxus_solana_core::{
    deserialize_transaction, serialize_transaction, AppIdentity, Cluster, ConnectedAccount, Wallet,
    WalletError, WalletInfo, WalletSigner,
};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

mod plugin;

pub struct MwaRegistry {
    cluster: Cluster,
    identity: Option<AppIdentity>,
}

impl MwaRegistry {
    pub fn new(cluster: Cluster, identity: Option<AppIdentity>) -> Self {
        Self { cluster, identity }
    }

    pub fn wallets(&self) -> Vec<Rc<dyn Wallet>> {
        vec![Rc::new(MwaWallet {
            cluster: self.cluster.clone(),
            identity: self.identity.clone(),
            info: wallet_info(),
        })]
    }
}

struct MwaWallet {
    cluster: Cluster,
    identity: Option<AppIdentity>,
    info: WalletInfo,
}

#[async_trait::async_trait(?Send)]
impl Wallet for MwaWallet {
    fn info(&self) -> &WalletInfo {
        &self.info
    }

    async fn connect(&self) -> Result<ConnectedAccount, WalletError> {
        self.connect_with(false).await
    }

    async fn connect_silent(&self) -> Result<ConnectedAccount, WalletError> {
        self.connect_with(true).await
    }
}

impl MwaWallet {
    async fn connect_with(&self, silent: bool) -> Result<ConnectedAccount, WalletError> {
        let (name, uri, icon) = identity_parts(&self.identity);
        let payload = plugin::call(
            "connect",
            &[
                if silent { "1" } else { "0" }.into(),
                crate::native_mwa::rpc_cluster_from_chain_id(self.cluster.chain_id()).into(),
                name,
                uri,
                icon,
            ],
        )
        .await?;
        let pubkey = pubkey_from_b64(&payload)?;
        Ok(ConnectedAccount {
            wallet_name: wallet_info().name,
            signer: Rc::new(MwaSigner { pubkey }),
        })
    }
}

struct MwaSigner {
    pubkey: Pubkey,
}

#[async_trait::async_trait(?Send)]
impl WalletSigner for MwaSigner {
    fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError> {
        let payload = plugin::call("signMessage", &[encode_b64(msg)]).await?;
        signature_from_b64(&payload)
    }

    async fn sign_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<VersionedTransaction, WalletError> {
        let payload = plugin::call(
            "signTransaction",
            &[encode_b64(&serialize_transaction(&tx)?)],
        )
        .await?;
        deserialize_transaction(&decode_b64(&payload)?)
    }

    async fn sign_and_send_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<Signature, WalletError> {
        let payload =
            plugin::call("signAndSend", &[encode_b64(&serialize_transaction(&tx)?)]).await?;
        signature_from_b64(&payload)
    }

    async fn disconnect(&self) -> Result<(), WalletError> {
        let _ = plugin::call("disconnect", &[]).await;
        Ok(())
    }
}

fn identity_parts(identity: &Option<AppIdentity>) -> (String, String, String) {
    match identity {
        Some(id) => (id.name.clone(), id.uri.clone(), id.icon.clone()),
        None => ("Dioxus App".into(), String::new(), "favicon.ico".into()),
    }
}

type Pending = Mutex<
    std::collections::HashMap<u64, futures_channel::oneshot::Sender<Result<String, WalletError>>>,
>;

fn pending() -> &'static Pending {
    static PENDING: OnceLock<Pending> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

fn next_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub(super) fn complete(id: u64, status: &str, payload: &str, error: &str) {
    let tx = pending()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&id);
    if let Some(tx) = tx {
        let result = if status == "ok" {
            Ok(payload.to_string())
        } else {
            Err(error_from_status(status, error))
        };
        let _ = tx.send(result);
    }
}

pub(super) fn register_wait() -> (
    u64,
    futures_channel::oneshot::Receiver<Result<String, WalletError>>,
) {
    let id = next_id();
    let (tx, rx) = futures_channel::oneshot::channel();
    pending()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(id, tx);
    (id, rx)
}
