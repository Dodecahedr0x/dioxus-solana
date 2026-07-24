# Dioxus-Solana Wallet Library Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust/WASM workspace that gives Dioxus web apps browser-wallet connection + signing (Wallet Standard), exposed via a `WalletProvider` context and `use_wallet()` hook, with a platform-agnostic core so desktop can drop in later.

**Architecture:** Three-crate workspace. `dioxus-solana-core` holds platform-agnostic async traits + types (no wasm deps). `dioxus-solana-web` implements those traits against the browser Wallet Standard registry via `wasm-bindgen`. `dioxus-solana` is the facade: re-exports core, selects the platform impl by `cfg(target_arch)`, and layers Dioxus hooks/context. RPC and transaction-building are out of scope (clients use `spume` + the `solana-*` crates).

**Tech Stack:** Rust, Dioxus 0.7, wasm-bindgen 0.2 / js-sys / web-sys 0.3, gloo-events 0.3, async-trait 0.1, modular `solana-*` crates (`>=3`), bincode 1.

**Companion design doc:** `docs/plans/2026-07-24-dioxus-solana-wallet-design.md`

---

## Verified API facts (do not re-guess these)

- `solana_pubkey::Pubkey` — `Pubkey::new_from_array([u8; 32])`, `pubkey.to_bytes() -> [u8; 32]`. Build from the Wallet Standard account `publicKey` (32 raw bytes) → no base58 `decode` feature needed.
- `solana_transaction::versioned::VersionedTransaction` — derives serde `Serialize`/`Deserialize` when the `serde` feature is on. Round-trip with `bincode::serialize(&tx)` / `bincode::deserialize::<VersionedTransaction>(&bytes)` (bincode 1.x).
- `solana_signature::Signature` — 64-byte; construct via `Signature::from([u8; 64])`. No inherent `new`/`from_bytes`.
- `solana_message::VersionedMessage` — enum, pulled transitively by `solana-transaction`.
- Dioxus 0.7 `prelude`: `use_context_provider`, `use_context`, `use_signal`, `Signal` (Copy), `spawn`, `#[component]`, `rsx!`. Run async from an event handler with `spawn(async move { ... })`.
- wasm-bindgen 0.2.126, js-sys 0.3.103, web-sys 0.3.103, wasm-bindgen-futures 0.4.76, gloo-events 0.3.0, async-trait 0.1.
- web-sys is feature-gated per type: enable `Window`, `Storage`, `CustomEvent`, `CustomEventInit`, `EventTarget`, `Event`.
- Wallet Standard handshake: app listens for `wallet-standard:register-wallet` (detail is a callback `(api) => void`) and dispatches `wallet-standard:app-ready` (detail is the app API `{ register(...wallets) }`). Registered wallet object fields: `version`, `name`, `icon` (data URI), `chains`, `accounts` (each `{ address, publicKey: Uint8Array(32), chains, features, label? }`), `features` (record). Feature keys: `standard:connect`, `standard:disconnect`, `standard:events`, `solana:signTransaction`, `solana:signMessage`, `solana:signAndSendTransaction`, `solana:signIn`. Chains: `solana:mainnet`, `solana:devnet`, `solana:testnet`, `solana:localnet`.

---

## Task 0: Workspace scaffolding

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `crates/dioxus-solana-core/Cargo.toml`, `crates/dioxus-solana-core/src/lib.rs`
- Create: `crates/dioxus-solana-web/Cargo.toml`, `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana/Cargo.toml`, `crates/dioxus-solana/src/lib.rs`

**Step 1: Root `Cargo.toml`**

```toml
[workspace]
resolver = "2"
members = ["crates/*", "examples/*"]

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/aursen-labs/dioxus-solana"

[workspace.dependencies]
# Solana modular crates — floor at v3, no strict pin (cargo resolves 4.x)
solana-pubkey      = ">=3"
solana-signature   = ">=3"
solana-transaction = { version = ">=3", features = ["serde"] }
solana-message     = ">=3"
bincode            = "1"
async-trait        = "0.1"
thiserror          = "2"
# WASM / web
wasm-bindgen         = "0.2"
wasm-bindgen-futures = "0.4"
js-sys               = "0.3"
web-sys              = "0.3"
gloo-events          = "0.3"
# UI
dioxus = "0.7"
# internal
dioxus-solana-core = { path = "crates/dioxus-solana-core" }
dioxus-solana-web  = { path = "crates/dioxus-solana-web" }
```

**Step 2: `crates/dioxus-solana-core/Cargo.toml`**

```toml
[package]
name = "dioxus-solana-core"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
solana-pubkey.workspace = true
solana-signature.workspace = true
solana-transaction.workspace = true
solana-message.workspace = true
bincode.workspace = true
async-trait.workspace = true
thiserror.workspace = true
```

**Step 3: `crates/dioxus-solana-web/Cargo.toml`**

```toml
[package]
name = "dioxus-solana-web"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
dioxus-solana-core.workspace = true
solana-pubkey.workspace = true
solana-signature.workspace = true
solana-transaction.workspace = true
async-trait.workspace = true
wasm-bindgen.workspace = true
wasm-bindgen-futures.workspace = true
js-sys.workspace = true
gloo-events.workspace = true

[dependencies.web-sys]
workspace = true
features = ["Window", "Storage", "CustomEvent", "CustomEventInit", "EventTarget", "Event"]

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

**Step 4: `crates/dioxus-solana/Cargo.toml`**

```toml
[package]
name = "dioxus-solana"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
dioxus-solana-core.workspace = true
dioxus.workspace = true
solana-pubkey.workspace = true
solana-transaction.workspace = true
solana-signature.workspace = true

[target.'cfg(target_arch = "wasm32")'.dependencies]
dioxus-solana-web.workspace = true
web-sys = { workspace = true, features = ["Window", "Storage"] }
```

**Step 5: Stub `lib.rs` for each crate**

```rust
// crates/dioxus-solana-core/src/lib.rs
// crates/dioxus-solana-web/src/lib.rs
// crates/dioxus-solana/src/lib.rs
// (empty for now)
```

**Step 6: Verify the workspace builds**

Run: `cargo build`
Expected: PASS (empty crates compile; cargo resolves solana 4.x / 3.x).

**Step 7: Commit**

```bash
git add Cargo.toml crates/
git commit -m "chore: scaffold dioxus-solana workspace"
```

---

## Task 1: Core — `Cluster` type + chain identifiers

**Files:**
- Create: `crates/dioxus-solana-core/src/cluster.rs`
- Modify: `crates/dioxus-solana-core/src/lib.rs`

**Step 1: Write the failing test** (append to `cluster.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_id_maps_each_cluster() {
        assert_eq!(Cluster::MainnetBeta.chain_id(), "solana:mainnet");
        assert_eq!(Cluster::Devnet.chain_id(), "solana:devnet");
        assert_eq!(Cluster::Testnet.chain_id(), "solana:testnet");
        assert_eq!(Cluster::Localnet.chain_id(), "solana:localnet");
    }
}
```

**Step 2: Run to verify it fails**

Run: `cargo test -p dioxus-solana-core cluster`
Expected: FAIL — `Cluster` not found.

**Step 3: Implement** (top of `cluster.rs`)

```rust
/// A Solana cluster, used to build Wallet Standard chain identifier strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cluster {
    #[default]
    MainnetBeta,
    Devnet,
    Testnet,
    Localnet,
}

impl Cluster {
    /// The Wallet Standard chain identifier, e.g. `"solana:devnet"`.
    pub fn chain_id(&self) -> &'static str {
        match self {
            Cluster::MainnetBeta => "solana:mainnet",
            Cluster::Devnet => "solana:devnet",
            Cluster::Testnet => "solana:testnet",
            Cluster::Localnet => "solana:localnet",
        }
    }
}
```

Add to `lib.rs`:

```rust
mod cluster;
pub use cluster::Cluster;
```

**Step 4: Run to verify it passes**

Run: `cargo test -p dioxus-solana-core cluster`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-core/
git commit -m "feat(core): add Cluster with Wallet Standard chain ids"
```

---

## Task 2: Core — `WalletError` + JS error-code mapping

**Files:**
- Create: `crates/dioxus-solana-core/src/error.rs`
- Modify: `crates/dioxus-solana-core/src/lib.rs`

**Step 1: Write the failing test** (append to `error.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_user_rejection_code() {
        assert_eq!(WalletError::from_js(Some(4001.0), "User rejected"), WalletError::UserRejected);
    }

    #[test]
    fn maps_rejected_by_message() {
        assert_eq!(WalletError::from_js(None, "Request was rejected by user"), WalletError::UserRejected);
    }

    #[test]
    fn falls_back_to_js_variant() {
        assert_eq!(WalletError::from_js(None, "boom"), WalletError::Js("boom".into()));
    }
}
```

**Step 2: Run to verify it fails**

Run: `cargo test -p dioxus-solana-core error`
Expected: FAIL — `WalletError` not found.

**Step 3: Implement** (top of `error.rs`)

```rust
use thiserror::Error;

/// Errors surfaced by wallet operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum WalletError {
    #[error("wallet not installed")]
    NotInstalled,
    #[error("user rejected the request")]
    UserRejected,
    #[error("wallet is disconnected")]
    Disconnected,
    #[error("wallet is missing required feature: {0}")]
    Feature(String),
    #[error("javascript error: {0}")]
    Js(String),
}

impl WalletError {
    /// Map a JS error `code` (e.g. `4001`) and message onto a typed error.
    pub fn from_js(code: Option<f64>, message: &str) -> Self {
        if code == Some(4001.0) || message.to_lowercase().contains("reject") {
            return WalletError::UserRejected;
        }
        WalletError::Js(message.to_string())
    }
}
```

Add to `lib.rs`:

```rust
mod error;
pub use error::WalletError;
```

**Step 4: Run to verify it passes**

Run: `cargo test -p dioxus-solana-core error`
Expected: PASS (all 3).

**Step 5: Commit**

```bash
git add crates/dioxus-solana-core/
git commit -m "feat(core): add WalletError with JS error-code mapping"
```

---

## Task 3: Core — codec helpers (transaction bytes, signature, pubkey)

These are pure functions, plain-testable, that the web crate reuses at the JS boundary.

**Files:**
- Create: `crates/dioxus-solana-core/src/codec.rs`
- Modify: `crates/dioxus-solana-core/src/lib.rs`

**Step 1: Write the failing test** (append to `codec.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use solana_transaction::versioned::VersionedTransaction;

    #[test]
    fn transaction_round_trips_through_bytes() {
        let tx = VersionedTransaction::default();
        let bytes = serialize_transaction(&tx).unwrap();
        let back = deserialize_transaction(&bytes).unwrap();
        assert_eq!(tx, back);
    }

    #[test]
    fn signature_requires_64_bytes() {
        assert!(signature_from_slice(&[0u8; 64]).is_ok());
        assert!(signature_from_slice(&[0u8; 10]).is_err());
    }

    #[test]
    fn pubkey_from_32_bytes() {
        let pk = pubkey_from_slice(&[7u8; 32]).unwrap();
        assert_eq!(pk.to_bytes(), [7u8; 32]);
        assert!(pubkey_from_slice(&[0u8; 5]).is_err());
    }
}
```

**Step 2: Run to verify it fails**

Run: `cargo test -p dioxus-solana-core codec`
Expected: FAIL — functions not found.

**Step 3: Implement** (top of `codec.rs`)

```rust
use crate::WalletError;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

/// Serialize a versioned transaction to bincode bytes (wire format wallets expect).
pub fn serialize_transaction(tx: &VersionedTransaction) -> Result<Vec<u8>, WalletError> {
    bincode::serialize(tx).map_err(|e| WalletError::Js(format!("serialize tx: {e}")))
}

/// Deserialize bincode bytes back into a versioned transaction.
pub fn deserialize_transaction(bytes: &[u8]) -> Result<VersionedTransaction, WalletError> {
    bincode::deserialize(bytes).map_err(|e| WalletError::Js(format!("deserialize tx: {e}")))
}

/// Build a `Signature` from exactly 64 bytes.
pub fn signature_from_slice(bytes: &[u8]) -> Result<Signature, WalletError> {
    let arr: [u8; 64] = bytes
        .try_into()
        .map_err(|_| WalletError::Js(format!("expected 64-byte signature, got {}", bytes.len())))?;
    Ok(Signature::from(arr))
}

/// Build a `Pubkey` from exactly 32 bytes.
pub fn pubkey_from_slice(bytes: &[u8]) -> Result<Pubkey, WalletError> {
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| WalletError::Js(format!("expected 32-byte pubkey, got {}", bytes.len())))?;
    Ok(Pubkey::new_from_array(arr))
}
```

Add to `lib.rs`:

```rust
mod codec;
pub use codec::{deserialize_transaction, pubkey_from_slice, serialize_transaction, signature_from_slice};
```

**Step 4: Run to verify it passes**

Run: `cargo test -p dioxus-solana-core codec`
Expected: PASS. If `serialize_transaction` fails to compile, confirm the `serde` feature is enabled on `solana-transaction` in the workspace deps.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-core/
git commit -m "feat(core): add codec helpers for tx/signature/pubkey bytes"
```

---

## Task 4: Core — types + async traits

**Files:**
- Create: `crates/dioxus-solana-core/src/types.rs`
- Create: `crates/dioxus-solana-core/src/wallet.rs`
- Modify: `crates/dioxus-solana-core/src/lib.rs`

**Step 1: Write the failing test** (append to `wallet.rs`) — a mock impl proves the traits are usable & object-safe

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::WalletError;
    use solana_pubkey::Pubkey;
    use solana_signature::Signature;
    use solana_transaction::versioned::VersionedTransaction;
    use std::rc::Rc;

    struct MockSigner(Pubkey);

    #[async_trait::async_trait(?Send)]
    impl WalletSigner for MockSigner {
        fn pubkey(&self) -> Pubkey { self.0 }
        async fn sign_message(&self, _m: &[u8]) -> Result<Signature, WalletError> {
            Ok(Signature::from([1u8; 64]))
        }
        async fn sign_transaction(&self, tx: VersionedTransaction) -> Result<VersionedTransaction, WalletError> {
            Ok(tx)
        }
        async fn sign_and_send_transaction(&self, _tx: VersionedTransaction) -> Result<Signature, WalletError> {
            Ok(Signature::from([2u8; 64]))
        }
        async fn disconnect(&self) -> Result<(), WalletError> { Ok(()) }
    }

    #[tokio::test]
    async fn signer_is_object_safe_and_usable() {
        let signer: Rc<dyn WalletSigner> = Rc::new(MockSigner(Pubkey::new_from_array([9u8; 32])));
        let account = ConnectedAccount { wallet_name: "Mock".into(), signer };
        assert_eq!(account.pubkey(), Pubkey::new_from_array([9u8; 32]));
        assert_eq!(account.signer.sign_message(b"hi").await.unwrap(), Signature::from([1u8; 64]));
    }
}
```

Add `tokio = { version = "1", features = ["macros", "rt"] }` to `[dev-dependencies]` of `dioxus-solana-core/Cargo.toml`.

**Step 2: Run to verify it fails**

Run: `cargo test -p dioxus-solana-core wallet`
Expected: FAIL — `WalletSigner` / `ConnectedAccount` not found.

**Step 3: Implement `types.rs`**

```rust
use solana_pubkey::Pubkey;
use std::rc::Rc;

use crate::wallet::WalletSigner;

/// Metadata about a discovered wallet (pre-connection), for building a picker UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletInfo {
    pub name: String,
    /// data-URI icon, if the wallet provided one.
    pub icon: Option<String>,
    /// Feature keys the wallet advertises, e.g. `"solana:signTransaction"`.
    pub features: Vec<String>,
}

/// An active connection to a chosen account.
#[derive(Clone)]
pub struct ConnectedAccount {
    pub wallet_name: String,
    pub signer: Rc<dyn WalletSigner>,
}

impl ConnectedAccount {
    pub fn pubkey(&self) -> Pubkey {
        self.signer.pubkey()
    }
}
```

**Step 4: Implement `wallet.rs`** (top of file)

```rust
use crate::{types::ConnectedAccount, WalletError, WalletInfo};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

/// A discovered wallet, before connection.
#[async_trait::async_trait(?Send)]
pub trait Wallet {
    fn info(&self) -> &WalletInfo;
    async fn connect(&self) -> Result<ConnectedAccount, WalletError>;
}

/// An active signing session.
#[async_trait::async_trait(?Send)]
pub trait WalletSigner {
    fn pubkey(&self) -> Pubkey;
    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError>;
    async fn sign_transaction(&self, tx: VersionedTransaction) -> Result<VersionedTransaction, WalletError>;
    async fn sign_and_send_transaction(&self, tx: VersionedTransaction) -> Result<Signature, WalletError>;
    async fn disconnect(&self) -> Result<(), WalletError>;
}
```

Add to `lib.rs`:

```rust
mod types;
mod wallet;
pub use types::{ConnectedAccount, WalletInfo};
pub use wallet::{Wallet, WalletSigner};
```

**Step 5: Run to verify it passes**

Run: `cargo test -p dioxus-solana-core`
Expected: PASS (all core tests).

**Step 6: Commit**

```bash
git add crates/dioxus-solana-core/
git commit -m "feat(core): add Wallet/WalletSigner traits and connection types"
```

---

## Task 5: Web — JS reflection helpers

Encapsulate the fiddly `Reflect::get` / call-and-await patterns once, tested against a mock JS object.

**Files:**
- Create: `crates/dioxus-solana-web/src/js.rs`
- Modify: `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana-web/tests/js.rs`

**Step 1: Write the failing test** (`crates/dioxus-solana-web/tests/js.rs`)

```rust
#![cfg(target_arch = "wasm32")]
use dioxus_solana_web::js::{get, get_string};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn reads_nested_properties() {
    // Build { name: "Phantom", nested: { x: 5 } } via JS.
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"name".into(), &"Phantom".into()).unwrap();
    let nested = js_sys::Object::new();
    js_sys::Reflect::set(&nested, &"x".into(), &JsValue::from_f64(5.0)).unwrap();
    js_sys::Reflect::set(&obj, &"nested".into(), &nested).unwrap();

    assert_eq!(get_string(&obj, "name").as_deref(), Some("Phantom"));
    let n = get(&obj, "nested").unwrap();
    assert_eq!(get(&n, "x").unwrap().as_f64(), Some(5.0));
    assert!(get(&obj, "missing").is_none());
}
```

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: FAIL — `js` module / functions not found. (Install `wasm-pack` if missing; Chrome is also fine via `--chrome`.)

**Step 3: Implement `js.rs`**

```rust
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
```

Add to `lib.rs`:

```rust
pub mod js;
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-web/
git commit -m "feat(web): add JS reflection + promise-await helpers"
```

---

## Task 6: Web — mock wallet test fixture

A reusable JS mock wallet so later binding tests don't need a real extension.

**Files:**
- Create: `crates/dioxus-solana-web/tests/mock.rs` (shared helper module used by later test files via `#[path]`)

**Step 1: Implement the fixture** (no separate failing test — it's exercised by Task 7+). Write it, then assert it constructs.

```rust
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
        Promise::resolve(&result.into())
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
        Promise::resolve(&arr.into())
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
        Promise::resolve(&arr.into())
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
        Promise::resolve(&arr.into())
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
```

**Step 2: Sanity test** — add to `tests/mock.rs`:

```rust
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
```

**Step 3: Run**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS.

**Step 4: Commit**

```bash
git add crates/dioxus-solana-web/tests/mock.rs
git commit -m "test(web): add mock Wallet-Standard wallet fixture"
```

---

## Task 7: Web — `StandardSigner` implementing `WalletSigner`

**Files:**
- Create: `crates/dioxus-solana-web/src/signer.rs`
- Modify: `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana-web/tests/signer.rs`

**Step 1: Write the failing test** (`tests/signer.rs`)

```rust
#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, WalletSigner};
use dioxus_solana_web::signer::StandardSigner;
use solana_transaction::versioned::VersionedTransaction;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn signer() -> StandardSigner {
    let wallet = mock::make_mock_wallet();
    let account = js_sys::Reflect::get(&wallet, &"accounts".into()).unwrap();
    let account = js_sys::Array::from(&account).get(0);
    StandardSigner::new(wallet, account, Cluster::Devnet).unwrap()
}

#[wasm_bindgen_test]
async fn pubkey_reads_account_bytes() {
    assert_eq!(signer().pubkey().to_bytes(), [9u8; 32]);
}

#[wasm_bindgen_test]
async fn sign_message_returns_signature() {
    let sig = signer().sign_message(b"hello").await.unwrap();
    assert_eq!(sig.as_ref(), &[1u8; 64]);
}

#[wasm_bindgen_test]
async fn sign_transaction_round_trips() {
    let tx = VersionedTransaction::default();
    let signed = signer().sign_transaction(tx.clone()).await.unwrap();
    assert_eq!(signed, tx);
}

#[wasm_bindgen_test]
async fn sign_and_send_returns_signature() {
    let sig = signer().sign_and_send_transaction(VersionedTransaction::default()).await.unwrap();
    assert_eq!(sig.as_ref(), &[2u8; 64]);
}
```

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: FAIL — `StandardSigner` not found.

**Step 3: Implement `signer.rs`**

```rust
use async_trait::async_trait;
use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

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
        let bytes = Uint8Array::new(&pk).to_vec();
        let pubkey = pubkey_from_slice(&bytes)?;
        Ok(Self { wallet, account, pubkey, cluster })
    }

    /// Fetch the callable function for a feature, e.g. ("solana:signMessage", "signMessage").
    fn feature_fn(&self, namespace: &str, method: &str) -> Result<(JsValue, js_sys::Function), WalletError> {
        let features = get(&self.wallet, "features")
            .ok_or_else(|| WalletError::Js("wallet missing features".into()))?;
        let feature = get(&features, namespace).ok_or_else(|| WalletError::Feature(namespace.into()))?;
        let f = get_function(&feature, method).ok_or_else(|| WalletError::Feature(namespace.into()))?;
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
        let out = await_promise(f.call1(&feature, &input).map_err(|e| crate::js::js_error(&e))?).await?;
        // out is an array; first element has `signature: Uint8Array`
        let first = Array::from(&out).get(0);
        let sig = get(&first, "signature").ok_or_else(|| WalletError::Js("no signature".into()))?;
        signature_from_slice(&Uint8Array::new(&sig).to_vec())
    }

    async fn sign_transaction(&self, tx: VersionedTransaction) -> Result<VersionedTransaction, WalletError> {
        let (feature, f) = self.feature_fn("solana:signTransaction", "signTransaction")?;
        let input = Object::new();
        Reflect::set(&input, &"account".into(), &self.account).unwrap();
        Reflect::set(&input, &"transaction".into(), &Uint8Array::from(serialize_transaction(&tx)?.as_slice())).unwrap();
        Reflect::set(&input, &"chain".into(), &self.cluster.chain_id().into()).unwrap();
        let out = await_promise(f.call1(&feature, &input).map_err(|e| crate::js::js_error(&e))?).await?;
        let first = Array::from(&out).get(0);
        let signed = get(&first, "signedTransaction").ok_or_else(|| WalletError::Js("no signedTransaction".into()))?;
        deserialize_transaction(&Uint8Array::new(&signed).to_vec())
    }

    async fn sign_and_send_transaction(&self, tx: VersionedTransaction) -> Result<Signature, WalletError> {
        let (feature, f) = self.feature_fn("solana:signAndSendTransaction", "signAndSendTransaction")?;
        let input = Object::new();
        Reflect::set(&input, &"account".into(), &self.account).unwrap();
        Reflect::set(&input, &"transaction".into(), &Uint8Array::from(serialize_transaction(&tx)?.as_slice())).unwrap();
        Reflect::set(&input, &"chain".into(), &self.cluster.chain_id().into()).unwrap();
        let out = await_promise(f.call1(&feature, &input).map_err(|e| crate::js::js_error(&e))?).await?;
        let first = Array::from(&out).get(0);
        let sig = get(&first, "signature").ok_or_else(|| WalletError::Js("no signature".into()))?;
        signature_from_slice(&Uint8Array::new(&sig).to_vec())
    }

    async fn disconnect(&self) -> Result<(), WalletError> {
        let (feature, f) = self.feature_fn("standard:disconnect", "disconnect")?;
        await_promise(f.call0(&feature).map_err(|e| crate::js::js_error(&e))?).await?;
        Ok(())
    }
}
```

Make `js_error` public in `js.rs` (change `fn js_error` → `pub fn js_error`). Add to `lib.rs`:

```rust
pub mod signer;
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS (all 4 signer tests).

**Step 5: Commit**

```bash
git add crates/dioxus-solana-web/
git commit -m "feat(web): implement StandardSigner over Wallet Standard features"
```

---

## Task 8: Web — `StandardWallet` implementing `Wallet` + connect

**Files:**
- Create: `crates/dioxus-solana-web/src/wallet.rs`
- Modify: `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana-web/tests/wallet.rs`

**Step 1: Write the failing test** (`tests/wallet.rs`)

```rust
#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, Wallet};
use dioxus_solana_web::wallet::StandardWallet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn info_reads_name_and_features() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    assert_eq!(w.info().name, "MockWallet");
    assert!(w.info().features.iter().any(|f| f == "solana:signTransaction"));
}

#[wasm_bindgen_test]
async fn connect_returns_account_with_pubkey() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    let account = w.connect().await.unwrap();
    assert_eq!(account.wallet_name, "MockWallet");
    assert_eq!(account.pubkey().to_bytes(), [9u8; 32]);
}
```

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: FAIL — `StandardWallet` not found.

**Step 3: Implement `wallet.rs`**

```rust
use async_trait::async_trait;
use js_sys::{Array, Object, Reflect};
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
```

Add to `lib.rs`:

```rust
pub mod wallet;
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-web/
git commit -m "feat(web): implement StandardWallet discovery info + connect"
```

---

## Task 9: Web — registry discovery handshake

**Files:**
- Create: `crates/dioxus-solana-web/src/discovery.rs`
- Modify: `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana-web/tests/discovery.rs`

**Step 1: Write the failing test** (`tests/discovery.rs`)

```rust
#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::Cluster;
use dioxus_solana_web::discovery::WalletRegistry;
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn app_ready_dispatch_registers_a_listening_wallet() {
    // Simulate a wallet: on `wallet-standard:app-ready`, call detail.register(mockWallet).
    let wallet = mock::make_mock_wallet();
    let cb = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |e: web_sys::CustomEvent| {
        let api = e.detail();
        let register: Function = Reflect::get(&api, &"register".into()).unwrap().dyn_into().unwrap();
        register.call1(&api, &wallet).unwrap();
    });
    let win = web_sys::window().unwrap();
    win.add_event_listener_with_callback("wallet-standard:app-ready", cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    let registry = WalletRegistry::new(Cluster::Devnet);
    registry.discover(); // dispatches app-ready; the wallet registers synchronously
    let wallets = registry.wallets();
    assert!(wallets.iter().any(|w| w.info().name == "MockWallet"));
}
```

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: FAIL — `WalletRegistry` not found.

**Step 3: Implement `discovery.rs`**

```rust
use gloo_events::EventListener;
use js_sys::{Array, Function, Object, Reflect};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

use dioxus_solana_core::Cluster;

use crate::wallet::StandardWallet;

/// Collects wallets registered through the Wallet Standard window handshake.
pub struct WalletRegistry {
    cluster: Cluster,
    raw: Rc<RefCell<Vec<JsValue>>>,
    _listener: Rc<RefCell<Option<EventListener>>>,
}

impl WalletRegistry {
    pub fn new(cluster: Cluster) -> Self {
        Self {
            cluster,
            raw: Rc::new(RefCell::new(Vec::new())),
            _listener: Rc::new(RefCell::new(None)),
        }
    }

    /// Build the app-side `{ register(...wallets) }` API object.
    fn make_api(raw: Rc<RefCell<Vec<JsValue>>>) -> Object {
        let api = Object::new();
        let register = Closure::<dyn FnMut(JsValue)>::new(move |wallet: JsValue| {
            raw.borrow_mut().push(wallet);
        });
        // NOTE: wallets call register(wallet) with a single arg in practice.
        Reflect::set(&api, &"register".into(), register.as_ref().unchecked_ref()).unwrap();
        register.forget();
        api
    }

    /// Run the handshake: listen for late registrations, then dispatch app-ready.
    pub fn discover(&self) {
        let window = web_sys::window().expect("no window");
        let api = Self::make_api(self.raw.clone());

        // 1. Answer any `register-wallet` events (wallet -> app callback).
        let api_for_listener = api.clone();
        let listener = EventListener::new(&window, "wallet-standard:register-wallet", move |event| {
            if let Ok(ce) = event.clone().dyn_into::<web_sys::CustomEvent>() {
                if let Ok(cb) = ce.detail().dyn_into::<Function>() {
                    let _ = cb.call1(&JsValue::NULL, &api_for_listener);
                }
            }
        });
        *self._listener.borrow_mut() = Some(listener);

        // 2. Dispatch `app-ready` with our api as detail (app -> wallet).
        let mut init = web_sys::CustomEventInit::new();
        init.detail(&api);
        let event = web_sys::CustomEvent::new_with_event_init_dict("wallet-standard:app-ready", &init)
            .expect("construct app-ready");
        let _ = window.dispatch_event(&event);
    }

    /// Snapshot the currently-registered wallets as core `StandardWallet`s.
    pub fn wallets(&self) -> Vec<StandardWallet> {
        self.raw
            .borrow()
            .iter()
            .cloned()
            .map(|raw| StandardWallet::new(raw, self.cluster))
            .collect()
    }
}
```

Add to `lib.rs`:

```rust
pub mod discovery;
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS. If `CustomEventInit::detail` is deprecated in your web-sys minor, use the generated setter (`init.set_detail(&api)`); check `cargo doc -p web-sys`.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-web/
git commit -m "feat(web): implement Wallet Standard registry discovery handshake"
```

---

## Task 10: Web — public `discover()` entry point returning trait objects

**Files:**
- Modify: `crates/dioxus-solana-web/src/lib.rs`
- Create: `crates/dioxus-solana-web/tests/entry.rs`

**Step 1: Write the failing test** (`tests/entry.rs`)

```rust
#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::Cluster;
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn discover_returns_boxed_trait_objects() {
    let wallet = mock::make_mock_wallet();
    let cb = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |e: web_sys::CustomEvent| {
        let api = e.detail();
        let register: Function = Reflect::get(&api, &"register".into()).unwrap().dyn_into().unwrap();
        register.call1(&api, &wallet).unwrap();
    });
    let win = web_sys::window().unwrap();
    win.add_event_listener_with_callback("wallet-standard:app-ready", cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    let wallets = dioxus_solana_web::discover(Cluster::Devnet);
    assert!(wallets.iter().any(|w| w.info().name == "MockWallet"));
}
```

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: FAIL — `discover` not found.

**Step 3: Implement** (append to `lib.rs`)

```rust
use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

/// Discover installed Wallet-Standard wallets as core trait objects.
///
/// Runs the window handshake synchronously and returns whatever registered.
/// Call again to re-snapshot (e.g. after a wallet loads late).
pub fn discover(cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    let registry = discovery::WalletRegistry::new(cluster);
    registry.discover();
    registry
        .wallets()
        .into_iter()
        .map(|w| Rc::new(w) as Rc<dyn Wallet>)
        .collect()
}
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/dioxus-solana-web/
git commit -m "feat(web): expose discover() returning Rc<dyn Wallet>"
```

---

## Task 11: Facade — platform selection + state type

**Files:**
- Create: `crates/dioxus-solana/src/platform.rs`
- Create: `crates/dioxus-solana/src/state.rs`
- Modify: `crates/dioxus-solana/src/lib.rs`

**Step 1: Write the failing test** (append to `state.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_helpers() {
        assert!(WalletState::Disconnected.pubkey().is_none());
        assert!(!WalletState::Connecting.is_connected());
    }
}
```

**Step 2: Run to verify it fails**

Run: `cargo test -p dioxus-solana state`
Expected: FAIL — `WalletState` not found.

**Step 3: Implement `state.rs`**

```rust
use dioxus_solana_core::{ConnectedAccount, WalletError};
use solana_pubkey::Pubkey;

/// Reactive connection state held in a Dioxus `Signal`.
#[derive(Clone, Default)]
pub enum WalletState {
    #[default]
    Disconnected,
    Connecting,
    Connected(ConnectedAccount),
    Error(WalletError),
}

impl WalletState {
    pub fn is_connected(&self) -> bool {
        matches!(self, WalletState::Connected(_))
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        match self {
            WalletState::Connected(a) => Some(a.pubkey()),
            _ => None,
        }
    }

    pub fn account(&self) -> Option<&ConnectedAccount> {
        match self {
            WalletState::Connected(a) => Some(a),
            _ => None,
        }
    }
}
```

**Step 4: Implement `platform.rs`** (the desktop seam — non-wasm compiles to an empty stub)

```rust
use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub fn discover(cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    dioxus_solana_web::discover(cluster)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn discover(_cluster: Cluster) -> Vec<Rc<dyn Wallet>> {
    // Desktop connector not yet implemented; keeps the facade compiling off-wasm.
    Vec::new()
}
```

Update `lib.rs`:

```rust
mod platform;
mod state;

pub use dioxus_solana_core::{
    Cluster, ConnectedAccount, Wallet, WalletError, WalletInfo, WalletSigner,
};
pub use state::WalletState;
```

**Step 5: Run to verify it passes**

Run: `cargo test -p dioxus-solana state`
Expected: PASS.

**Step 6: Commit**

```bash
git add crates/dioxus-solana/
git commit -m "feat(facade): add WalletState and platform discover seam"
```

---

## Task 12: Facade — `localStorage` persistence helpers

**Files:**
- Create: `crates/dioxus-solana/src/storage.rs`
- Modify: `crates/dioxus-solana/src/lib.rs`

**Step 1: Write the failing test** (`crates/dioxus-solana/tests/storage.rs`)

```rust
#![cfg(target_arch = "wasm32")]
use dioxus_solana::storage;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn remembers_and_clears_last_wallet() {
    storage::set_last_wallet("Phantom");
    assert_eq!(storage::last_wallet().as_deref(), Some("Phantom"));
    storage::clear_last_wallet();
    assert_eq!(storage::last_wallet(), None);
}
```

Add `[dev-dependencies] wasm-bindgen-test = "0.3"` to `crates/dioxus-solana/Cargo.toml`.

**Step 2: Run to verify it fails**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana`
Expected: FAIL — `storage` not found.

**Step 3: Implement `storage.rs`**

```rust
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
```

Add to `lib.rs`:

```rust
#[cfg(target_arch = "wasm32")]
pub mod storage;
```

**Step 4: Run to verify it passes**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/dioxus-solana/
git commit -m "feat(facade): add localStorage persistence for last wallet"
```

---

## Task 13: Facade — `WalletProvider` + `use_wallet` hook

This is UI wiring; it is verified by compilation + the example app (Task 14), not unit tests (Dioxus component runtime tests are out of scope).

**Files:**
- Create: `crates/dioxus-solana/src/provider.rs`
- Create: `crates/dioxus-solana/src/hooks.rs`
- Modify: `crates/dioxus-solana/src/lib.rs`

**Step 1: Implement `provider.rs`**

```rust
use dioxus::prelude::*;
use dioxus_solana_core::{Cluster, Wallet, WalletInfo};
use std::rc::Rc;

use crate::state::WalletState;

/// Shared reactive state placed in Dioxus context by `WalletProvider`.
#[derive(Clone, Copy)]
pub struct WalletContext {
    pub cluster: Signal<Cluster>,
    pub state: Signal<WalletState>,
    pub wallets: Signal<Vec<Rc<dyn Wallet>>>,
}

/// Props for [`WalletProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct WalletProviderProps {
    #[props(default = Cluster::MainnetBeta)]
    pub cluster: Cluster,
    #[props(default = true)]
    pub autoconnect: bool,
    pub children: Element,
}

/// Provides wallet context to descendants. Mount once near the app root.
#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    let cluster = use_signal(|| props.cluster);
    let state = use_signal(WalletState::default);
    let wallets = use_signal(Vec::<Rc<dyn Wallet>>::new);

    let ctx = WalletContext { cluster, state, wallets };
    use_context_provider(|| ctx);

    // Discover on mount (and attempt autoconnect).
    let autoconnect = props.autoconnect;
    use_effect(move || {
        let mut wallets = wallets;
        let found = crate::platform::discover(cluster());
        wallets.set(found.clone());

        #[cfg(target_arch = "wasm32")]
        if autoconnect {
            if let Some(name) = crate::storage::last_wallet() {
                if let Some(w) = found.iter().find(|w| w.info().name == name).cloned() {
                    let mut state = state;
                    spawn(async move {
                        state.set(WalletState::Connecting);
                        match w.connect().await {
                            Ok(acc) => state.set(WalletState::Connected(acc)),
                            Err(_) => state.set(WalletState::Disconnected),
                        }
                    });
                }
            }
        }
        let _ = autoconnect; // silence unused off-wasm
    });

    // Expose infos for pickers without leaking trait objects.
    let _infos: Vec<WalletInfo> = wallets().iter().map(|w| w.info().clone()).collect();

    rsx! { {props.children} }
}
```

**Step 2: Implement `hooks.rs`**

```rust
use dioxus::prelude::*;
use dioxus_solana_core::{WalletError, WalletInfo};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

use crate::{provider::WalletContext, state::WalletState};

/// Cheap `Copy` handle over the wallet context. Get it with [`use_wallet`].
#[derive(Clone, Copy)]
pub struct WalletHandle {
    ctx: WalletContext,
}

/// Access wallet state + actions from any descendant of `WalletProvider`.
pub fn use_wallet() -> WalletHandle {
    WalletHandle { ctx: use_context::<WalletContext>() }
}

impl WalletHandle {
    pub fn state(&self) -> WalletState {
        (self.ctx.state)()
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        (self.ctx.state)().pubkey()
    }

    pub fn wallets(&self) -> Vec<WalletInfo> {
        (self.ctx.wallets)().iter().map(|w| w.info().clone()).collect()
    }

    /// Connect a wallet by name (as reported in `wallets()`).
    pub fn connect(&self, name: String) {
        let mut state = self.ctx.state;
        let wallets = self.ctx.wallets;
        spawn(async move {
            let Some(w) = wallets().iter().find(|w| w.info().name == name).cloned() else {
                state.set(WalletState::Error(WalletError::NotInstalled));
                return;
            };
            state.set(WalletState::Connecting);
            match w.connect().await {
                Ok(acc) => {
                    #[cfg(target_arch = "wasm32")]
                    crate::storage::set_last_wallet(&acc.wallet_name);
                    state.set(WalletState::Connected(acc));
                }
                Err(e) => state.set(WalletState::Error(e)),
            }
        });
    }

    pub fn disconnect(&self) {
        let mut state = self.ctx.state;
        spawn(async move {
            if let WalletState::Connected(acc) = state() {
                let _ = acc.signer.disconnect().await;
            }
            #[cfg(target_arch = "wasm32")]
            crate::storage::clear_last_wallet();
            state.set(WalletState::Disconnected);
        });
    }

    /// Await a message signature. Returns `Disconnected` if not connected.
    pub async fn sign_message(&self, msg: Vec<u8>) -> Result<Signature, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_message(&msg).await,
            _ => Err(WalletError::Disconnected),
        }
    }

    pub async fn sign_transaction(&self, tx: VersionedTransaction) -> Result<VersionedTransaction, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_transaction(tx).await,
            _ => Err(WalletError::Disconnected),
        }
    }

    pub async fn sign_and_send(&self, tx: VersionedTransaction) -> Result<Signature, WalletError> {
        match (self.ctx.state)() {
            WalletState::Connected(acc) => acc.signer.sign_and_send_transaction(tx).await,
            _ => Err(WalletError::Disconnected),
        }
    }
}
```

Update `lib.rs`:

```rust
mod hooks;
mod provider;

pub use hooks::{use_wallet, WalletHandle};
pub use provider::{WalletContext, WalletProvider, WalletProviderProps};
```

**Step 3: Verify it compiles for wasm**

Run: `cargo build -p dioxus-solana --target wasm32-unknown-unknown`
Expected: PASS. (Install target: `rustup target add wasm32-unknown-unknown`.)

**Step 4: Verify it compiles off-wasm (desktop seam)**

Run: `cargo build -p dioxus-solana`
Expected: PASS (platform stub returns no wallets; storage cfg'd out).

**Step 5: Commit**

```bash
git add crates/dioxus-solana/
git commit -m "feat(facade): add WalletProvider context and use_wallet hook"
```

---

## Task 14: Example — `connect-demo` app

**Files:**
- Create: `examples/connect-demo/Cargo.toml`
- Create: `examples/connect-demo/src/main.rs`
- Create: `examples/connect-demo/Dioxus.toml`

**Step 1: `examples/connect-demo/Cargo.toml`**

```toml
[package]
name = "connect-demo"
version = "0.1.0"
edition.workspace = true
publish = false

[dependencies]
dioxus = { workspace = true, features = ["web"] }
dioxus-solana = { path = "../../crates/dioxus-solana" }
```

**Step 2: `examples/connect-demo/src/main.rs`**

```rust
use dioxus::prelude::*;
use dioxus_solana::{use_wallet, Cluster, WalletProvider, WalletState};

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        WalletProvider { cluster: Cluster::Devnet, autoconnect: true, Demo {} }
    }
}

#[component]
fn Demo() -> Element {
    let wallet = use_wallet();
    let mut last_sig = use_signal(|| Option::<String>::None);

    rsx! {
        h1 { "dioxus-solana demo" }
        match wallet.state() {
            WalletState::Connected(_) => rsx! {
                p { "Connected: {wallet.pubkey().unwrap()}" }
                button { onclick: move |_| wallet.disconnect(), "Disconnect" }
                button {
                    onclick: move |_| async move {
                        if let Ok(sig) = wallet.sign_message(b"gm from dioxus".to_vec()).await {
                            last_sig.set(Some(sig.to_string()));
                        }
                    },
                    "Sign message"
                }
                if let Some(sig) = last_sig() {
                    p { "Signature: {sig}" }
                }
            },
            WalletState::Connecting => rsx! { p { "Connecting…" } },
            _ => rsx! {
                p { "Choose a wallet:" }
                for info in wallet.wallets() {
                    button {
                        key: "{info.name}",
                        onclick: {
                            let name = info.name.clone();
                            move |_| wallet.connect(name.clone())
                        },
                        "{info.name}"
                    }
                }
                if let WalletState::Error(e) = wallet.state() {
                    p { style: "color:red", "Error: {e}" }
                }
            }
        }
    }
}
```

**Step 3: `examples/connect-demo/Dioxus.toml`**

```toml
[application]
name = "connect-demo"
default_platform = "web"

[web.app]
title = "dioxus-solana demo"
```

**Step 4: Verify it builds**

Run: `cargo build -p connect-demo --target wasm32-unknown-unknown`
Expected: PASS.

**Step 5: Manual smoke test** (real wallet)

Run: `dx serve --package connect-demo` (install Dioxus CLI: `cargo install dioxus-cli`)
Open the served URL in a browser with Phantom or Solflare set to Devnet. Confirm: the wallet appears in the picker, connecting shows the pubkey, "Sign message" produces a signature, disconnect clears state, and a reload auto-reconnects.

**Step 6: Commit**

```bash
git add examples/
git commit -m "feat(example): add connect-demo Dioxus web app"
```

---

## Task 15: Workspace-wide verification + README

**Files:**
- Create: `README.md`

**Step 1: Run the whole non-wasm suite**

Run: `cargo test --workspace`
Expected: PASS (core + facade native tests).

**Step 2: Run the wasm suites**

Run: `wasm-pack test --headless --firefox crates/dioxus-solana-web`
Run: `wasm-pack test --headless --firefox crates/dioxus-solana`
Expected: PASS.

**Step 3: Lint**

Run: `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
Expected: clean (fix anything that isn't; re-run).

**Step 4: Write `README.md`** — quickstart: add `dioxus-solana`, wrap app in `WalletProvider`, call `use_wallet()`; note RPC via `spume` and tx-building via `solana-*` crates; note web-only today, desktop seam ready.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: add README quickstart"
```

---

## Notes for the executor

- **`?Send` everywhere:** never add `Send`/`Sync` bounds — wasm futures aren't `Send`, and the whole stack is single-threaded by design.
- **web-sys minor drift:** if `CustomEventInit::detail(...)` is deprecated, switch to the generated `set_detail`; verify with `cargo doc -p web-sys --open`.
- **bincode:** if `solana-transaction` doesn't expose serde derives, confirm the `serde` feature is active (Task 0 sets it). The tx wire format wallets expect is plain bincode of `VersionedTransaction`.
- **Variadic `register`:** real wallets call `register(wallet)` with one argument; the single-arg `Closure` handles them. If a wallet passes multiple, only the first is captured — acceptable for v1 (log a follow-up if you hit it).
- **Don't add RPC:** if you feel the urge to fetch balances, stop — that's the app's job via `spume`. Keep this crate RPC-free.
- Follow @superpowers:test-driven-development and commit after every green step.
```
