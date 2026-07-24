use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use dioxus_solana_core::WalletError;

/// Read a property, returning `None` if it is missing/undefined/null.
pub fn get(target: &JsValue, key: &str) -> Option<JsValue> {
    let v = Reflect::get(target, &JsValue::from_str(key)).ok()?;
    if v.is_undefined() || v.is_null() {
        None
    } else {
        Some(v)
    }
}

/// Read a string property.
pub fn get_string(target: &JsValue, key: &str) -> Option<String> {
    get(target, key)?.as_string()
}

/// Read a property expected to be a JS function.
pub fn get_function(target: &JsValue, key: &str) -> Option<Function> {
    get(target, key)?.dyn_into::<Function>().ok()
}

/// Convert a caught JS error value into a typed `WalletError`.
pub fn js_error(value: &JsValue) -> WalletError {
    let code = get(value, "code").and_then(|c| c.as_f64());
    let message = get_string(value, "message")
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("{value:?}"));
    WalletError::from_js(code, &message)
}

/// Await a JS value that is expected to be a `Promise`, mapping rejection to `WalletError`.
pub async fn await_promise(value: JsValue) -> Result<JsValue, WalletError> {
    let promise: Promise = value.dyn_into().map_err(|v| js_error(&v))?;
    JsFuture::from(promise).await.map_err(|e| js_error(&e))
}
