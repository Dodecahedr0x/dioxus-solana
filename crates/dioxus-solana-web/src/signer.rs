use async_trait::async_trait;
use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;

use dioxus_solana_core::{
    deserialize_transaction, pubkey_from_slice, serialize_transaction, signature_from_slice,
    Cluster, WalletError, WalletSigner,
};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

use crate::js::{await_promise, get, get_function};

/// A signing session bound to one wallet + account, implementing the core trait.
pub struct StandardSigner {
    wallet: JsValue,
    account: JsValue,
    pubkey: Pubkey,
    cluster: Cluster,
}

impl StandardSigner {
    /// Build from a raw wallet JS object and one of its account objects.
    pub fn new(wallet: JsValue, account: JsValue, cluster: Cluster) -> Result<Self, WalletError> {
        let pk = get(&account, "publicKey")
            .ok_or_else(|| WalletError::Js("account missing publicKey".into()))?;
        let bytes = crate::js::as_byte_vec(&pk)?;
        let pubkey = pubkey_from_slice(&bytes)?;
        Ok(Self {
            wallet,
            account,
            pubkey,
            cluster,
        })
    }

    /// Fetch the callable function for a feature, e.g. ("solana:signMessage", "signMessage").
    fn feature_fn(
        &self,
        namespace: &str,
        method: &str,
    ) -> Result<(JsValue, js_sys::Function), WalletError> {
        let features = get(&self.wallet, "features")
            .ok_or_else(|| WalletError::Js("wallet missing features".into()))?;
        let feature =
            get(&features, namespace).ok_or_else(|| WalletError::Feature(namespace.into()))?;
        let f =
            get_function(&feature, method).ok_or_else(|| WalletError::Feature(namespace.into()))?;
        Ok((feature, f))
    }
}

#[async_trait(?Send)]
impl WalletSigner for StandardSigner {
    fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError> {
        let (feature, f) = self.feature_fn("solana:signMessage", "signMessage")?;
        let input = Object::new();
        Reflect::set(&input, &"account".into(), &self.account).unwrap();
        Reflect::set(&input, &"message".into(), &Uint8Array::from(msg)).unwrap();
        let out = await_promise(
            f.call1(&feature, &input)
                .map_err(|e| crate::js::js_error(&e))?,
        )
        .await?;
        // out is an array; first element has `signature: Uint8Array`
        let first = Array::from(&out).get(0);
        let sig = get(&first, "signature").ok_or_else(|| WalletError::Js("no signature".into()))?;
        signature_from_slice(&crate::js::as_byte_vec(&sig)?)
    }

    async fn sign_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<VersionedTransaction, WalletError> {
        let (feature, f) = self.feature_fn("solana:signTransaction", "signTransaction")?;
        let input = Object::new();
        Reflect::set(&input, &"account".into(), &self.account).unwrap();
        Reflect::set(
            &input,
            &"transaction".into(),
            &Uint8Array::from(serialize_transaction(&tx)?.as_slice()),
        )
        .unwrap();
        Reflect::set(&input, &"chain".into(), &self.cluster.chain_id().into()).unwrap();
        let out = await_promise(
            f.call1(&feature, &input)
                .map_err(|e| crate::js::js_error(&e))?,
        )
        .await?;
        let first = Array::from(&out).get(0);
        let signed = get(&first, "signedTransaction")
            .ok_or_else(|| WalletError::Js("no signedTransaction".into()))?;
        deserialize_transaction(&crate::js::as_byte_vec(&signed)?)
    }

    async fn sign_and_send_transaction(
        &self,
        tx: VersionedTransaction,
    ) -> Result<Signature, WalletError> {
        let (feature, f) =
            self.feature_fn("solana:signAndSendTransaction", "signAndSendTransaction")?;
        let input = Object::new();
        Reflect::set(&input, &"account".into(), &self.account).unwrap();
        Reflect::set(
            &input,
            &"transaction".into(),
            &Uint8Array::from(serialize_transaction(&tx)?.as_slice()),
        )
        .unwrap();
        Reflect::set(&input, &"chain".into(), &self.cluster.chain_id().into()).unwrap();
        let out = await_promise(
            f.call1(&feature, &input)
                .map_err(|e| crate::js::js_error(&e))?,
        )
        .await?;
        let first = Array::from(&out).get(0);
        let sig = get(&first, "signature").ok_or_else(|| WalletError::Js("no signature".into()))?;
        signature_from_slice(&crate::js::as_byte_vec(&sig)?)
    }

    async fn disconnect(&self) -> Result<(), WalletError> {
        let (feature, f) = self.feature_fn("standard:disconnect", "disconnect")?;
        await_promise(f.call0(&feature).map_err(|e| crate::js::js_error(&e))?).await?;
        Ok(())
    }
}
