# Dioxus-Solana: Wallet Connection & Signing for Dioxus

**Date:** 2026-07-24
**Status:** Design approved, pending implementation

## Summary

A Rust/WASM library that gives Dioxus web apps browser-wallet connection and
signing, exposed through idiomatic Dioxus context + hooks. It deliberately does
**not** reimplement RPC or transaction construction: clients use the modular
`solana-*` crates for types/tx-building and [`spume`](https://github.com/aursen-labs/spume)
(a WASM-compatible Solana JSON-RPC client) for RPC. This library fills the one
gap those don't cover — wallet connection, signing, and Dioxus reactivity.

Web (WASM) ships first. The core is platform-agnostic so a desktop connector
(keystore / hardware) can drop in later without touching the hook layer.

## Goals

- Detect and list installed browser wallets (Phantom, Solflare, Backpack, …)
- Connect / disconnect; expose the connected pubkey reactively
- `signMessage`, `signTransaction`, `signAndSendTransaction`
- Dioxus `WalletProvider` context + `use_wallet()` hook with `Signal`-backed
  state so components re-render on connection changes
- Autoconnect / persistence of the last-used wallet

## Non-Goals (v1)

- RPC client — delegated to `spume` or the app's own client
- Transaction construction / program helpers — use the `solana-*` crates
- Desktop / mobile connectors — future work; the seam is designed in
- `use_balance` and other RPC-backed hooks — may be added later as thin
  `spume`-backed helpers; cut now to keep the crate RPC-free

## Architecture

```
dioxus-solana/
├── Cargo.toml              # workspace, shared deps, solana crates ">=3" (no strict pins)
├── crates/
│   ├── dioxus-solana-core/ # platform-agnostic: traits, types, errors
│   ├── dioxus-solana-web/  # Wallet Standard impl (wasm-bindgen), no JS shipped
│   └── dioxus-solana/      # facade: re-exports core + active platform + Dioxus hooks/context
├── examples/
│   └── connect-demo/       # minimal Dioxus web app
└── docs/plans/
```

**Dependency rule:** `core` never imports `web`. Hooks/context live in the
facade and speak only to `core` traits. That is the desktop seam.

### `dioxus-solana-core`

No wasm, no web deps. Defines the contracts and reuses solana types directly.

```rust
/// A discovered wallet (e.g. Phantom), before connection.
#[async_trait(?Send)]   // ?Send: wasm futures aren't Send
pub trait Wallet {
    fn info(&self) -> &WalletInfo;                  // name, icon (data URI), features
    async fn connect(&self) -> Result<ConnectedAccount, WalletError>;
}

/// An active session with a chosen account.
#[async_trait(?Send)]
pub trait WalletSigner {
    fn pubkey(&self) -> Pubkey;                     // solana-pubkey
    async fn sign_message(&self, msg: &[u8]) -> Result<Signature, WalletError>;
    async fn sign_transaction(&self, tx: VersionedTransaction)
        -> Result<VersionedTransaction, WalletError>;
    async fn sign_and_send_transaction(&self, tx: VersionedTransaction)
        -> Result<Signature, WalletError>;          // wallet submits via its own RPC
    async fn disconnect(&self) -> Result<(), WalletError>;
}
```

Shared types: `WalletInfo`, `WalletAccount` / `ConnectedAccount`, `Cluster`
(`MainnetBeta` / `Devnet` / `Testnet`), and:

```rust
pub enum WalletError {
    NotInstalled,
    UserRejected,
    Disconnected,
    Feature(String),   // wallet lacks a required feature
    Js(String),        // any other JS-side failure
}
```

**Signing model decisions:**

- Solana's own `Signer` trait is synchronous; browser signing is inherently
  async (popup approval). Core therefore defines its own async traits rather
  than implementing `solana_signer::Signer`.
- Uses `VersionedTransaction` (`solana-transaction`) to cover legacy and v0
  messages. Callers build txs with the raw solana crates, then hand them over.
- `sign_and_send_transaction` is first-class: Wallet Standard exposes
  `solana:signAndSendTransaction`, letting the wallet pick RPC/priority-fee.
  Both sign-then-send-yourself (via spume) and wallet-sends paths are supported.
- `?Send` bounds throughout because wasm-bindgen futures aren't `Send`. A future
  desktop impl lives with `?Send` (fine — Dioxus desktop futures run on a
  local set too).

### `dioxus-solana-web`

Implements the core traits against the [Wallet Standard](https://github.com/wallet-standard/wallet-standard)
registry — pure Rust over `wasm-bindgen` / `js-sys` / `web-sys`. No bundled JS.

**Discovery handshake:**

1. Register a listener for the `wallet-standard:register-wallet` window event.
2. Dispatch `wallet-standard:app-ready` with a callback; already-loaded wallets
   register synchronously, late ones fire the event.
3. Wrap each registered JS wallet object in `StandardWallet`; read `name`,
   `icon`, `chains`, `features` via `js_sys::Reflect::get`.

**Feature invocation** (encapsulated once): for each feature
(`standard:connect`, `solana:signMessage`, `solana:signTransaction`,
`solana:signAndSendTransaction`, `standard:disconnect`):

- `Reflect::get(features, "solana:signTransaction")` → feature object
- `Reflect::get(feature, "signTransaction")` → the `Function`
- build the JS input (`{ transaction: Uint8Array, account, chain: "solana:mainnet" }`)
- `Function::call` → `Promise` → `wasm_bindgen_futures::JsFuture::from(promise).await`
- read returned `signedTransaction` bytes into a `VersionedTransaction` via `bincode`

**Serialization boundary:** transactions cross as `Uint8Array` (serialized
`VersionedTransaction`); signatures return as 64-byte arrays →
`solana_signature::Signature`. Chain string derived from `Cluster`.

**Error mapping:** caught JS exceptions inspect the wallet error code/name;
`4001` / "rejected" → `UserRejected`, otherwise → `Js(msg)`.

### `dioxus-solana` (facade)

Layers reactive Dioxus ergonomics over the core traits — the API most users touch.

```rust
rsx! {
    WalletProvider {
        cluster: Cluster::Devnet,   // used to build chain strings for signing
        autoconnect: true,          // restore last wallet from localStorage
        App {}
    }
}
```

```rust
pub enum WalletState {
    Disconnected,
    Connecting,
    Connected(ConnectedAccount),   // holds the active WalletSigner
    Error(WalletError),
}

let wallet = use_wallet();          // -> WalletHandle (cheap Copy, Signal-backed)

wallet.wallets();                   // Vec<WalletInfo> discovered (for a picker UI)
wallet.state();                     // reactive WalletState
wallet.pubkey();                    // Option<Pubkey>
wallet.connect(name).await?;
wallet.disconnect().await?;
wallet.sign_message(bytes).await?;
wallet.sign_transaction(tx).await?;
wallet.sign_and_send(tx).await?;
```

State is a `Signal`, so any component reading `wallet.state()` / `wallet.pubkey()`
re-renders automatically on connect/disconnect — the reactive win over calling
the traits directly.

**Autoconnect / persistence:** on successful connect, store the wallet `name` in
`localStorage` (`web-sys`); on mount with `autoconnect: true`, re-discover and
silently reconnect if that wallet reappears. Standard wallet-adapter behavior.

The facade selects the platform impl by target
(`cfg(target_arch = "wasm32")` → web).

## Dependencies & Versioning

Modular solana crates with a major floor only (no strict pins); cargo resolves
compatible minors. `solana-sdk` / `solana-client` are avoided — they pull
non-WASM deps (tokio, blocking reqwest).

```toml
# workspace [workspace.dependencies]
solana-pubkey      = ">=3"
solana-signature   = ">=3"
solana-transaction = ">=3"   # VersionedTransaction, VersionedMessage
solana-message     = ">=3"
bincode            = "1"
```

Web-only deps (`dioxus-solana-web`): `wasm-bindgen`, `js-sys`, `web-sys`
(Window, Storage, CustomEvent), `wasm-bindgen-futures`, `async-trait`,
`gloo-events`, `thiserror`.

## Testing Strategy

WASM makes testing the hard part.

- **`core`**: plain `cargo test` for pure logic — error mapping,
  `Cluster` → chain-string, type round-trips. Fast, no browser.
- **`web`**: `wasm-bindgen-test` headless (`wasm-pack test --headless --firefox`)
  for the serialization boundary and event-handshake wiring, using a **mock
  wallet object** injected onto `window` — no real extension needed. Asserts:
  discovery finds the mock, connect returns the account, sign round-trips bytes,
  `4001` maps to `UserRejected`.
- **Manual / example**: `connect-demo` is the real-wallet smoke test
  (Phantom / Solflare on devnet).
- TDD per the superpowers workflow: write the failing `wasm-bindgen-test`
  against the mock, then implement the binding.

## Example App

`examples/connect-demo`: wallet picker → connect → show pubkey + cluster →
"Sign message" button → display signature. ~120 lines; doubles as living docs.

## Future Work

- `dioxus-solana-desktop`: keystore / hardware signer implementing the same
  core traits.
- Optional `spume`-backed convenience hooks (`use_balance`, `use_account`).
- Mobile Wallet Adapter connector.
