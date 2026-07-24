#![cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

/// Build a mock Wallet-Standard wallet object with name "MockWallet",
/// one account (pubkey = 32 bytes of `0x09`), and connect/sign features.
pub fn make_mock_wallet() -> JsValue {
    let wallet = Object::new();
    set(&wallet, "version", &"1.0.0".into());
    set(&wallet, "name", &"MockWallet".into());
    set(&wallet, "icon", &"data:image/svg+xml;base64,PHN2Zz48L3N2Zz4=".into());

    let chains = Array::new();
    chains.push(&"solana:devnet".into());
    set(&wallet, "chains", &chains);

    // account { address, publicKey: Uint8Array(32 x 0x09), chains, features }
    let account = Object::new();
    set(&account, "address", &"11111111111111111111111111111111".into());
    let pk = Uint8Array::new_with_length(32);
    pk.fill(9, 0, 32);
    set(&account, "publicKey", &pk);
    let acc_chains = Array::new();
    acc_chains.push(&"solana:devnet".into());
    set(&account, "chains", &acc_chains);
    set(&account, "features", &Array::new());
    let accounts = Array::new();
    accounts.push(&account);
    set(&wallet, "accounts", &accounts);

    // features record
    let features = Object::new();
    add_connect_feature(&features, &account, &accounts);
    add_sign_message_feature(&features);
    add_sign_transaction_feature(&features);
    add_sign_and_send_feature(&features);
    add_disconnect_feature(&features);
    set(&wallet, "features", &features);

    wallet.into()
}

fn set(obj: &Object, key: &str, val: &JsValue) {
    Reflect::set(obj, &key.into(), val).unwrap();
}

fn feature(namespace: &str, method_name: &str, f: &Function) -> (JsValue, JsValue) {
    let feat = Object::new();
    set(&feat, "version", &"1.0.0".into());
    set(&feat, method_name, f);
    (namespace.into(), feat.into())
}

fn add_connect_feature(features: &Object, _account: &Object, accounts: &Array) {
    let accounts = accounts.clone();
    // connect() -> Promise<{ accounts: [...] }>
    let cb = Closure::<dyn FnMut() -> Promise>::new(move || {
        let result = Object::new();
        set(&result, "accounts", &accounts);
        Promise::resolve(&JsValue::from(result))
    });
    let f: &Function = cb.as_ref().unchecked_ref();
    let (k, v) = feature("standard:connect", "connect", f);
    Reflect::set(features, &k, &v).unwrap();
    cb.forget();
}

fn add_sign_message_feature(features: &Object) {
    // signMessage(input) -> Promise<[{ signature: Uint8Array(64), signedMessage }]>
    let cb = Closure::<dyn FnMut(JsValue) -> Promise>::new(move |_input: JsValue| {
        let out = Object::new();
        let sig = Uint8Array::new_with_length(64);
        sig.fill(1, 0, 64);
        set(&out, "signature", &sig);
        let arr = Array::new();
        arr.push(&out);
        Promise::resolve(&JsValue::from(arr))
    });
    let f: &Function = cb.as_ref().unchecked_ref();
    let (k, v) = feature("solana:signMessage", "signMessage", f);
    Reflect::set(features, &k, &v).unwrap();
    cb.forget();
}

fn add_sign_transaction_feature(features: &Object) {
    // signTransaction(input) -> Promise<[{ signedTransaction: Uint8Array }]> (echoes input tx)
    let cb = Closure::<dyn FnMut(JsValue) -> Promise>::new(move |input: JsValue| {
        let tx = Reflect::get(&input, &"transaction".into()).unwrap();
        let out = Object::new();
        set(&out, "signedTransaction", &tx);
        let arr = Array::new();
        arr.push(&out);
        Promise::resolve(&JsValue::from(arr))
    });
    let f: &Function = cb.as_ref().unchecked_ref();
    let (k, v) = feature("solana:signTransaction", "signTransaction", f);
    Reflect::set(features, &k, &v).unwrap();
    cb.forget();
}

fn add_sign_and_send_feature(features: &Object) {
    // signAndSendTransaction(input) -> Promise<[{ signature: Uint8Array(64 x 2) }]>
    let cb = Closure::<dyn FnMut(JsValue) -> Promise>::new(move |_input: JsValue| {
        let out = Object::new();
        let sig = Uint8Array::new_with_length(64);
        sig.fill(2, 0, 64);
        set(&out, "signature", &sig);
        let arr = Array::new();
        arr.push(&out);
        Promise::resolve(&JsValue::from(arr))
    });
    let f: &Function = cb.as_ref().unchecked_ref();
    let (k, v) = feature("solana:signAndSendTransaction", "signAndSendTransaction", f);
    Reflect::set(features, &k, &v).unwrap();
    cb.forget();
}

fn add_disconnect_feature(features: &Object) {
    let cb = Closure::<dyn FnMut() -> Promise>::new(move || Promise::resolve(&JsValue::UNDEFINED));
    let f: &Function = cb.as_ref().unchecked_ref();
    let (k, v) = feature("standard:disconnect", "disconnect", f);
    Reflect::set(features, &k, &v).unwrap();
    cb.forget();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn mock_wallet_has_expected_shape() {
        let w = make_mock_wallet();
        assert_eq!(js_sys::Reflect::get(&w, &"name".into()).unwrap().as_string().unwrap(), "MockWallet");
    }
}
