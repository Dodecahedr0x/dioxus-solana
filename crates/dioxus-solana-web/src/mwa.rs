use dioxus_solana_core::AppIdentity;

/// True when local Mobile Wallet Adapter association can work (Android Chrome
/// in a secure context). Matches `@solana-mobile/wallet-standard-mobile`.
pub fn is_local_mwa_environment(user_agent: &str, is_secure_context: bool) -> bool {
    is_secure_context && user_agent.to_ascii_lowercase().contains("android")
}

/// Register the Mobile Wallet Adapter Wallet Standard wallet, if this browser
/// can actually use it. No-op on desktop, in insecure contexts, and if the
/// bundled script fails to evaluate.
pub fn register(identity: Option<&AppIdentity>, chain_id: &str) {
    register_impl(identity, chain_id)
}

#[cfg(target_arch = "wasm32")]
fn register_impl(identity: Option<&AppIdentity>, chain_id: &str) {
    use crate::js::get_function;
    use js_sys::{Array, Object, Reflect};

    let Some(window) = web_sys::window() else {
        return;
    };
    let ua = window.navigator().user_agent().unwrap_or_default();
    if !is_local_mwa_environment(&ua, window.is_secure_context()) {
        return;
    }
    if load_bundle().is_err() {
        return;
    }

    let Ok(mwa) = Reflect::get(&js_sys::global(), &"SolanaMobileWalletStandard".into()) else {
        return;
    };
    if mwa.is_undefined() || mwa.is_null() {
        return;
    }

    let identity = identity
        .cloned()
        .unwrap_or_else(|| default_identity(&window));
    let uri = if identity.uri.is_empty() {
        window.location().origin().unwrap_or_default()
    } else {
        identity.uri.clone()
    };

    let Some(register_fn) = get_function(&mwa, "registerMwa") else {
        return;
    };
    let Some(cache_fn) = get_function(&mwa, "createDefaultAuthorizationCache") else {
        return;
    };
    let Some(selector_fn) = get_function(&mwa, "createDefaultChainSelector") else {
        return;
    };
    let Some(not_found_fn) = get_function(&mwa, "createDefaultWalletNotFoundHandler") else {
        return;
    };

    let Ok(cache) = cache_fn.call0(&mwa) else {
        return;
    };
    let Ok(selector) = selector_fn.call0(&mwa) else {
        return;
    };
    let Ok(on_not_found) = not_found_fn.call0(&mwa) else {
        return;
    };

    let app = Object::new();
    let _ = Reflect::set(&app, &"name".into(), &identity.name.as_str().into());
    let _ = Reflect::set(&app, &"uri".into(), &uri.as_str().into());
    let _ = Reflect::set(&app, &"icon".into(), &identity.icon.as_str().into());

    let chains = Array::new();
    chains.push(&chain_id.into());

    let config = Object::new();
    let _ = Reflect::set(&config, &"appIdentity".into(), &app);
    let _ = Reflect::set(&config, &"authorizationCache".into(), &cache);
    let _ = Reflect::set(&config, &"chains".into(), &chains);
    let _ = Reflect::set(&config, &"chainSelector".into(), &selector);
    let _ = Reflect::set(&config, &"onWalletNotFound".into(), &on_not_found);

    let _ = register_fn.call1(&mwa, &config);
}

#[cfg(not(target_arch = "wasm32"))]
fn register_impl(_identity: Option<&AppIdentity>, _chain_id: &str) {}

#[cfg(target_arch = "wasm32")]
fn default_identity(window: &web_sys::Window) -> AppIdentity {
    let name = window
        .document()
        .and_then(|d| {
            let t = d.title();
            (!t.is_empty()).then_some(t)
        })
        .unwrap_or_else(|| "Dioxus App".into());
    AppIdentity {
        name,
        uri: String::new(),
        icon: "favicon.ico".into(),
    }
}

#[cfg(target_arch = "wasm32")]
fn load_bundle() -> Result<(), wasm_bindgen::JsValue> {
    use std::cell::Cell;
    thread_local! {
        static LOADED: Cell<bool> = const { Cell::new(false) };
    }
    if LOADED.with(Cell::get) {
        return Ok(());
    }
    js_sys::eval(include_str!("../js/wallet-standard-mobile.iife.js"))?;
    LOADED.with(|c| c.set(true));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_mwa_only_on_android_https() {
        assert!(is_local_mwa_environment(
            "Mozilla/5.0 (Linux; Android 14; Pixel 8) Chrome/120.0.0.0",
            true
        ));
        assert!(!is_local_mwa_environment(
            "Mozilla/5.0 (Linux; Android 14; Pixel 8) Chrome/120.0.0.0",
            false
        ));
        assert!(!is_local_mwa_environment(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/120.0.0.0",
            true
        ));
        assert!(!is_local_mwa_environment(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)",
            true
        ));
    }
}
