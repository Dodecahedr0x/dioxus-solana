//! JNI calls into `dev.dioxus.solana.MwaPlugin`. Kotlin starts MWA on the UI
//! thread and completes [`super::complete`] when the wallet returns.

use super::{complete, register_wait};
use dioxus_solana_core::WalletError;
use manganis::jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use manganis::jni::sys::jlong;
use manganis::jni::{self as jni, JNIEnv};

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
    let plugin = new_plugin()?;
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

/// Instantiate `MwaPlugin` through the activity's class loader.
///
/// The `manganis::ffi`-generated `MwaPlugin::new` uses `FindClass`, which on a
/// Rust-spawned thread resolves against the system class loader. That cannot
/// see app classes, and the pending `ClassNotFoundException` aborts the
/// process when the thread detaches.
fn new_plugin() -> Result<MwaPlugin, WalletError> {
    manganis::android::with_activity(|env, activity| {
        let plugin = load_plugin(env, activity);
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
        }
        Some(plugin.map(MwaPlugin::from_global_ref))
    })
    .unwrap_or_else(|| Err(WalletError::Js("no JNI environment".into())))
}

fn load_plugin(env: &mut JNIEnv, activity: &JObject) -> Result<GlobalRef, WalletError> {
    let err = |e: jni::errors::Error| WalletError::Js(format!("load MwaPlugin: {e}"));
    let loader = env
        .call_method(activity, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])
        .and_then(|v| v.l())
        .map_err(err)?;
    let name = env.new_string("dev.dioxus.solana.MwaPlugin").map_err(err)?;
    let class: JClass = env
        .call_method(
            &loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&name)],
        )
        .and_then(|v| v.l())
        .map_err(err)?
        .into();
    let plugin = env
        .new_object(
            &class,
            "(Landroid/app/Activity;)V",
            &[JValue::Object(activity)],
        )
        .map_err(err)?;
    env.new_global_ref(plugin).map_err(err)
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
