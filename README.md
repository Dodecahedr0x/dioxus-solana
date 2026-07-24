# dioxus-solana

Browser-wallet connection and signing (Wallet Standard) for Dioxus web apps, exposed via a
`WalletProvider` context and a `use_wallet()` hook. The core is platform-agnostic so a desktop
implementation can be dropped in later without changing the public API.

## Workspace layout

- `crates/dioxus-solana-core` — platform-agnostic async traits (`Wallet`, `WalletSigner`) and
  types (`Cluster`, `WalletInfo`, `ConnectedAccount`, `WalletError`). No wasm dependencies.
- `crates/dioxus-solana-web` — implements those traits against the browser Wallet Standard
  registry via `wasm-bindgen` (the `wallet-standard:register-wallet` /
  `wallet-standard:app-ready` handshake, `standard:connect`, `solana:signMessage`,
  `solana:signTransaction`, `solana:signAndSendTransaction`).
- `crates/dioxus-solana` — the facade crate apps depend on. Re-exports core types, selects the
  platform implementation by `cfg(target_arch)`, and layers Dioxus hooks/context on top.
- `examples/connect-demo` — a minimal Dioxus web app exercising the full connect / sign /
  disconnect flow.

## Quickstart

Add the facade crate to your Dioxus web app:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-solana = "0.1"
```

Wrap your app in `WalletProvider` once, near the root:

```rust
use dioxus::prelude::*;
use dioxus_solana::{Cluster, WalletProvider};

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        WalletProvider { cluster: Cluster::Devnet, autoconnect: true, App {} }
    }
}
```

Then use `use_wallet()` from any descendant component:

```rust
use dioxus::prelude::*;
use dioxus_solana::{use_wallet, WalletState};

#[component]
fn App() -> Element {
    let wallet = use_wallet();

    rsx! {
        match wallet.state() {
            WalletState::Connected(_) => rsx! {
                p { "Connected: {wallet.pubkey().unwrap()}" }
                button { onclick: move |_| wallet.disconnect(), "Disconnect" }
            },
            WalletState::Connecting => rsx! { p { "Connecting…" } },
            _ => rsx! {
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
            },
        }
    }
}
```

`WalletHandle` (returned by `use_wallet()`) exposes:

- `state()`, `pubkey()`, `wallets()`, `cluster()` — read the current connection state.
- `connect(name)`, `disconnect()` — manage the connection.
- `sign_message(bytes)`, `sign_transaction(tx)`, `sign_and_send(tx)` — async signing operations
  against the connected wallet.

See `examples/connect-demo` for a complete, buildable app.

## What this crate does *not* do

This crate is deliberately RPC-free: it only handles wallet discovery, connection, and signing.
Fetching account/balance data and building transactions is left to your app, typically via
[`spume`](https://crates.io/crates/spume) for RPC calls and the modular `solana-*` crates
(`solana-transaction`, `solana-message`, etc.) for transaction construction. `dioxus-solana` just
takes the `VersionedTransaction` you build and gets it signed (and optionally sent) by the wallet.

## Platform support

Web-only today (`dioxus-solana-web`, gated on `cfg(target_arch = "wasm32")`). The core crate has
no wasm dependencies by design, so a desktop wallet implementation (e.g. against local keypairs or
a hardware wallet) can be added later behind the same `Wallet` / `WalletSigner` traits without
changing the facade's public API.

## Building / testing

The example app only builds for `wasm32-unknown-unknown` (it depends on `dioxus`'s `web` feature,
which pulls in `dioxus-web`):

```bash
# native crates (core + facade)
cargo test --workspace --exclude connect-demo
cargo clippy --workspace --exclude connect-demo --all-targets -- -D warnings

# wasm target (facade, web impl, and the example)
cargo build --target wasm32-unknown-unknown -p dioxus-solana -p dioxus-solana-web -p connect-demo
cargo clippy -p connect-demo --target wasm32-unknown-unknown -- -D warnings

# run the example locally (requires the Dioxus CLI: cargo install dioxus-cli)
dx serve --package connect-demo
```
