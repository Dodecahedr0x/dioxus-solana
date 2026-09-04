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
    // second parameter (`_` is a reserved identifier).
    let request = format!("{id}\n{url}");
    open_url(&plugin, request).map_err(|e| WalletError::Js(e.into()))?;
    rx.await.unwrap_or(Err(WalletError::Disconnected))
}

/// Called from Swift when Phantom redirects back, or when the wallet cannot open.
#[no_mangle]
pub unsafe extern "C" fn dioxus_solana_phantom_on_result(
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
