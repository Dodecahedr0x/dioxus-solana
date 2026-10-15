//! Swift `PhantomPlugin` opens Phantom universal links. The wallet returns
//! through a custom URL scheme and [`super::complete`] resumes the oneshot.

use super::{complete, register_wait};
use dioxus_solana_core::WalletError;
use std::ffi::{c_char, CStr};

#[manganis::ffi("ios")]
extern "Swift" {
    pub type PhantomPlugin;
    pub fn open_url(this: &PhantomPlugin, request: String) -> String;
}

pub async fn open(url: String) -> Result<String, WalletError> {
    let (id, rx) = register_wait();
    let plugin = PhantomPlugin::new().map_err(|e| WalletError::Js(e.into()))?;
    // One Swift argument: manganis+objc2 cannot emit unlabeled `_:` for a
    // second parameter (`_` is a reserved identifier). The callback travels by
    // address: the plugin is a dynamically loaded framework, so it cannot link
    // against a symbol in the app executable.
    let callback = on_result as OnResult as usize;
    let request = format!("{callback}\n{id}\n{url}");
    let err = open_url(&plugin, request).map_err(|e| WalletError::Js(e.into()))?;
    if !err.is_empty() {
        return Err(WalletError::Js(err));
    }
    rx.await.unwrap_or(Err(WalletError::Disconnected))
}

type OnResult = unsafe extern "C" fn(i64, *const c_char, *const c_char, *const c_char);

/// Called from Swift when Phantom redirects back, or when the wallet cannot open.
unsafe extern "C" fn on_result(
    id: i64,
    status: *const c_char,
    payload: *const c_char,
    error: *const c_char,
) {
    complete(id as u64, &cstr(status), &cstr(payload), &cstr(error));
}

fn cstr(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}
