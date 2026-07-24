//! Persist the last-connected wallet name in `localStorage` (web only).
#![cfg(target_arch = "wasm32")]

const KEY: &str = "dioxus-solana:last-wallet";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn last_wallet() -> Option<String> {
    storage()?.get_item(KEY).ok().flatten()
}

pub fn set_last_wallet(name: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY, name);
    }
}

pub fn clear_last_wallet() {
    if let Some(s) = storage() {
        let _ = s.remove_item(KEY);
    }
}
