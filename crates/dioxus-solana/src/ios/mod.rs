use crate::native_mwa::{error_from_status, wallet_info};
use crate::native_phantom::{
    connect_account, connect_url, decrypt_payload, encrypt_payload, encrypted_method_url,
    generate_dapp_keys, parse_callback, pubkey_from_b58, sign_message_json, sign_transaction_json,
    signature_from_json, transaction_from_json, DappKeys, PHANTOM_REDIRECT,
    PHANTOM_SIGN_MESSAGE_URL, PHANTOM_SIGN_TRANSACTION_URL,
};
use crypto_box::SecretKey;
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

struct HeldSession {
    secret: SecretKey,
    dapp_pk_b58: String,
    phantom_pk_b58: String,
    session: String,
    user_pubkey: Pubkey,
}

fn in_flight() -> &'static Mutex<Option<DappKeys>> {
    static KEYS: OnceLock<Mutex<Option<DappKeys>>> = OnceLock::new();
    KEYS.get_or_init(|| Mutex::new(None))
}

fn session() -> &'static Mutex<Option<HeldSession>> {
    static SESSION: OnceLock<Mutex<Option<HeldSession>>> = OnceLock::new();
    SESSION.get_or_init(|| Mutex::new(None))
}

pub struct PhantomRegistry {
    cluster: Cluster,
    identity: Option<AppIdentity>,
}

impl PhantomRegistry {
    pub fn new(cluster: Cluster, identity: Option<AppIdentity>) -> Self {
        Self { cluster, identity }
    }

    pub fn wallets(&self) -> Vec<Rc<dyn Wallet>> {
        vec![Rc::new(PhantomWallet {
            cluster: self.cluster.clone(),
            identity: self.identity.clone(),
            info: wallet_info(),
        })]
    }
}

struct PhantomWallet {
    cluster: Cluster,
    identity: Option<AppIdentity>,
    info: WalletInfo,
}

#[async_trait::async_trait(?Send)]
impl Wallet for PhantomWallet {
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

impl PhantomWallet {
    async fn connect_with(&self, silent: bool) -> Result<ConnectedAccount, WalletError> {
        if silent {
            let held = session().lock().unwrap_or_else(|e| e.into_inner());
            return match held.as_ref() {
                Some(s) => Ok(account(s.user_pubkey)),
                None => Err(WalletError::Disconnected),
            };
        }

        let keys = generate_dapp_keys();
        *in_flight().lock().unwrap_or_else(|e| e.into_inner()) = Some(keys.clone());
        let (_name, uri, _icon) = identity_parts(&self.identity);
        let app_url = if uri.is_empty() {
            "https://dioxuslabs.com".into()
        } else {
            uri
        };
        let cluster = crate::native_mwa::rpc_cluster_from_chain_id(self.cluster.chain_id());
        let url = connect_url(&app_url, &keys.public_b58, PHANTOM_REDIRECT, cluster);
        let callback = plugin::open(url).await?;
        let cb = parse_callback(&callback)?;
        let keys = in_flight()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
            .ok_or(WalletError::Disconnected)?;
        let phantom_pk = cb.phantom_encryption_public_key;
        let plain = decrypt_payload(&keys.secret, &phantom_pk, &cb.nonce, &cb.data)?;
        let (pk_b58, token) = connect_account(&plain)?;
        let user_pubkey = pubkey_from_b58(&pk_b58)?;
        *session().lock().unwrap_or_else(|e| e.into_inner()) = Some(HeldSession {
            secret: keys.secret,
            dapp_pk_b58: keys.public_b58,
            phantom_pk_b58: phantom_pk,
            session: token,
            user_pubkey,
        });
        Ok(account(user_pubkey))
    }
}

fn account(pubkey: Pubkey) -> ConnectedAccount {
    ConnectedAccount {
        wallet_name: wallet_info().name,
        signer: Rc::new(PhantomSigner { pubkey }),
    }
}

struct PhantomSigner {
    pubkey: Pubkey,
}

#[async_trait::async_trait(?Send)]
impl WalletSigner for PhantomSigner {
    fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError> {
        let (url, secret, phantom_pk) = snapshot(|s| {
            let json = sign_message_json(msg, &s.session);
            encrypt_url(s, PHANTOM_SIGN_MESSAGE_URL, json.to_string().as_bytes())
        })?;
        let plain = open_decrypt(url, &secret, &phantom_pk).await?;
        Ok(Signature::try_from(
            signature_from_json(&plain)?.as_slice(),
        )?)
    }

    async fn sign_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<VersionedTransaction, WalletError> {
        let raw = serialize_transaction(&tx)?;
        let (url, secret, phantom_pk) = snapshot(|s| {
            let json = sign_transaction_json(&raw, &s.session);
            encrypt_url(s, PHANTOM_SIGN_TRANSACTION_URL, json.to_string().as_bytes())
        })?;
        let plain = open_decrypt(url, &secret, &phantom_pk).await?;
        deserialize_transaction(&transaction_from_json(&plain)?)
    }

    async fn sign_and_send_transaction(
        &self,
        _tx: VersionedTransaction,
    ) -> Result<Signature, WalletError> {
        Err(WalletError::Feature("solana:signAndSendTransaction".into()))
    }

    async fn disconnect(&self) -> Result<(), WalletError> {
        *session().lock().unwrap_or_else(|e| e.into_inner()) = None;
        *in_flight().lock().unwrap_or_else(|e| e.into_inner()) = None;
        Ok(())
    }
}

fn snapshot<T>(f: impl FnOnce(&HeldSession) -> Result<T, WalletError>) -> Result<T, WalletError> {
    let held = session().lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = held.as_ref() else {
        return Err(WalletError::Disconnected);
    };
    f(s)
}

fn encrypt_url(
    s: &HeldSession,
    base: &str,
    plaintext: &[u8],
) -> Result<(String, SecretKey, String), WalletError> {
    let (nonce, payload) = encrypt_payload(&s.secret, &s.phantom_pk_b58, plaintext)?;
    Ok((
        encrypted_method_url(base, &s.dapp_pk_b58, &nonce, PHANTOM_REDIRECT, &payload),
        s.secret.clone(),
        s.phantom_pk_b58.clone(),
    ))
}

async fn open_decrypt(
    url: String,
    secret: &SecretKey,
    phantom_pk: &str,
) -> Result<Vec<u8>, WalletError> {
    let callback = plugin::open(url).await?;
    let cb = parse_callback(&callback)?;
    let pk = if cb.phantom_encryption_public_key.is_empty() {
        phantom_pk
    } else {
        &cb.phantom_encryption_public_key
    };
    decrypt_payload(secret, pk, &cb.nonce, &cb.data)
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
