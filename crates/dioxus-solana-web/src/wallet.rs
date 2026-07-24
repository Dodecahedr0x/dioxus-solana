use async_trait::async_trait;
use js_sys::{Array, Object};
use wasm_bindgen::JsValue;

use dioxus_solana_core::{Cluster, ConnectedAccount, Wallet, WalletError, WalletInfo};
use std::rc::Rc;

use crate::{
    js::{await_promise, get, get_function, get_string, js_error},
    signer::StandardSigner,
};

/// A discovered Wallet-Standard wallet.
pub struct StandardWallet {
    raw: JsValue,
    info: WalletInfo,
    cluster: Cluster,
}

impl StandardWallet {
    pub fn new(raw: JsValue, cluster: Cluster) -> Self {
        let name = get_string(&raw, "name").unwrap_or_default();
        let icon = get_string(&raw, "icon");
        let features = get(&raw, "features")
            .map(|f| Object::keys(&Object::from(f)).iter().filter_map(|k| k.as_string()).collect())
            .unwrap_or_default();
        let info = WalletInfo { name, icon, features };
        Self { raw, info, cluster }
    }
}

#[async_trait(?Send)]
impl Wallet for StandardWallet {
    fn info(&self) -> &WalletInfo {
        &self.info
    }

    async fn connect(&self) -> Result<ConnectedAccount, WalletError> {
        let features = get(&self.raw, "features").ok_or_else(|| WalletError::Js("no features".into()))?;
        let feature = get(&features, "standard:connect").ok_or_else(|| WalletError::Feature("standard:connect".into()))?;
        let f = get_function(&feature, "connect").ok_or_else(|| WalletError::Feature("standard:connect".into()))?;
        let out = await_promise(f.call0(&feature).map_err(|e| js_error(&e))?).await?;
        // out = { accounts: [account, ...] }
        let accounts = get(&out, "accounts").ok_or_else(|| WalletError::Js("connect returned no accounts".into()))?;
        let account = Array::from(&accounts).get(0);
        if account.is_undefined() {
            return Err(WalletError::Js("connect returned empty accounts".into()));
        }
        let signer = StandardSigner::new(self.raw.clone(), account, self.cluster)?;
        Ok(ConnectedAccount { wallet_name: self.info.name.clone(), signer: Rc::new(signer) })
    }
}
