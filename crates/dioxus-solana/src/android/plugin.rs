//! JNI calls into `dev.dioxus.solana.MwaPlugin`. Kotlin starts MWA on the UI
//! thread and completes [`super::complete`] when the wallet returns.

use super::{complete, register_wait};
use dioxus_solana_core::WalletError;
use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

#[manganis::ffi("android")]
extern "Kotlin" {
    pub type MwaPlugin;
    pub fn connect(
        this: &MwaPlugin,
        request_id: i64,
        silent: String,
        cluster: String,
        name: String,
        uri: String,
        icon: String,
    );
    pub fn sign_message(this: &MwaPlugin, request_id: i64, message_b64: String);
    pub fn sign_transaction(this: &MwaPlugin, request_id: i64, tx_b64: String);
    pub fn sign_and_send(this: &MwaPlugin, request_id: i64, tx_b64: String);
    pub fn disconnect(this: &MwaPlugin, request_id: i64);
}

pub async fn call(method: &str, args: &[String]) -> Result<String, WalletError> {
    let (id, rx) = register_wait();
    start(id, method, args)?;
    rx.await.unwrap_or(Err(WalletError::Disconnected))
}

fn start(id: u64, method: &str, args: &[String]) -> Result<(), WalletError> {
    let plugin = MwaPlugin::new().map_err(WalletError::Js)?;
    let id = id as i64;
    match method {
        "connect" => {
            let silent = args.first().cloned().unwrap_or_else(|| "0".into());
            let cluster = args.get(1).cloned().unwrap_or_default();
            let name = args.get(2).cloned().unwrap_or_default();
            let uri = args.get(3).cloned().unwrap_or_default();
            let icon = args.get(4).cloned().unwrap_or_default();
            connect(&plugin, id, silent, cluster, name, uri, icon).map_err(WalletError::Js)?;
        }
        "signMessage" => {
            let msg = args.first().cloned().unwrap_or_default();
            sign_message(&plugin, id, msg).map_err(WalletError::Js)?;
        }
        "signTransaction" => {
            let tx = args.first().cloned().unwrap_or_default();
            sign_transaction(&plugin, id, tx).map_err(WalletError::Js)?;
        }
        "signAndSend" => {
            let tx = args.first().cloned().unwrap_or_default();
            sign_and_send(&plugin, id, tx).map_err(WalletError::Js)?;
        }
        "disconnect" => {
            disconnect(&plugin, id).map_err(WalletError::Js)?;
        }
        other => return Err(WalletError::Js(format!("unknown mwa method {other}"))),
    }
    Ok(())
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_solana_MwaPlugin_onResult(
    mut env: JNIEnv,
    _class: JClass,
    id: jlong,
    status: JString,
    payload: JString,
    error: JString,
) {
    let status: String = env
        .get_string(&status)
        .map(String::from)
        .unwrap_or_default();
    let payload: String = env
        .get_string(&payload)
        .map(String::from)
        .unwrap_or_default();
    let error: String = env.get_string(&error).map(String::from).unwrap_or_default();
    complete(id as u64, &status, &payload, &error);
}
