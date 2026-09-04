# dioxus-solana

Browser-wallet connection and signing (Wallet Standard, plus Mobile Wallet Adapter on Android
Chrome) for Dioxus web apps, exposed via a `WalletProvider` context and a `use_wallet()` hook. It also ships an optional `RpcProvider` plus
reactive RPC hooks (`use_slot`, `use_account`, `use_balance`, subscriptions) backed by
[`spume`](https://crates.io/crates/spume). The core is platform-agnostic so a desktop
implementation can be dropped in later without changing the public API. Native Android
(`dx serve --android`) discovers one **Mobile wallet** entry and talks to an on-device
MWA wallet through JNI.

## Workspace layout

- `crates/dioxus-solana-core` — platform-agnostic async traits (`Wallet`, `WalletSigner`) and
  types (`Cluster`, `WalletInfo`, `ConnectedAccount`, `WalletError`). No wasm dependencies.
- `crates/dioxus-solana-web` — implements those traits against the browser Wallet Standard
  registry via `wasm-bindgen` (the `wallet-standard:register-wallet` /
  `wallet-standard:app-ready` handshake, `standard:connect`, `solana:signMessage`,
  `solana:signTransaction`, `solana:signAndSendTransaction`). On Android Chrome it also
  registers [Mobile Wallet Adapter](https://docs.solanamobile.com/get-started/web/installation)
  as a Wallet Standard wallet. Its `rpc` module wraps
  [`spume`](https://crates.io/crates/spume) for wasm JSON-RPC and PubSub.
- `crates/dioxus-solana` — the facade crate apps depend on. Re-exports core types, selects the
  platform implementation by `cfg`, and layers Dioxus hooks/context on top. Native Android uses
  `crates/dioxus-solana/android` (Kotlin Mobile Wallet Adapter) through `manganis::ffi`.
- `examples/connect-demo` — a Dioxus web app exercising connect / sign-message / disconnect, a
  devnet airdrop, sending an on-chain **memo transaction** (`sign_and_send`), a live slot
  subscription, and a live balance.

## Quickstart

Add the facade crate to your Dioxus web app:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-solana = "0.1"
```

Wrap your app in `WalletProvider` once, near the root. To pick the cluster (the chain wallets sign
against), either pass `WalletProvider` a `cluster` prop, or — if you also want RPC — wrap it in an
`RpcProvider`, whose cluster takes precedence:

```rust
use dioxus::prelude::*;
use dioxus_solana::{AppIdentity, Cluster, WalletProvider};
use dioxus_solana::rpc::RpcProvider;

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        RpcProvider { cluster: Cluster::Devnet,
            WalletProvider {
                autoconnect: true,
                // Shown in the mobile wallet during authorization. `icon` must be a
                // relative path — MWA wallets reject absolute icon URLs.
                app_identity: Some(AppIdentity::named("My dapp")),
                App {}
            }
        }
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

## RPC providers and hooks

For live chain data, mount an `RpcProvider` and call the hooks from any descendant. It owns the
cluster — endpoints *and* the chain wallets sign against — so a nested `WalletProvider` picks the
same cluster up automatically. Endpoints default to the cluster's public URLs and can be overridden
with `rpc_url` / `ws_url` props.

The hooks come in three families:

- **Fetch** (`use_get_*`, plus the generic `use_fetch`) do one HTTP request and return a
  `Resource<Option<T>>` — read it with `res().flatten()`, refetch with `res.restart()`.
- **Subscription** (`use_*_subscription`, plus the generic `use_subscription`) hold a live
  WebSocket and return a `ReadSignal<Option<T>>` that updates on each push.
- **Live values** (`use_account`, `use_account_data`, `use_balance`) fetch immediately *and*
  subscribe, so a value is on screen at once and then kept current.

```rust
use dioxus::prelude::*;
use dioxus_solana::{Cluster, Pubkey};
use dioxus_solana::rpc::{RpcProvider, use_slot_subscription, use_balance, use_get_version};

#[component]
fn Root() -> Element {
    rsx! {
        RpcProvider { cluster: Cluster::Devnet, App {} }
    }
}

#[component]
fn Live(pubkey: Pubkey) -> Element {
    let slot = use_slot_subscription();  // ReadSignal<Option<u64>>  (live)
    let balance = use_balance(pubkey);   // fetched, then kept live
    let version = use_get_version();     // Resource<Option<RpcVersionInfo>>  (one-shot)
    rsx! {
        p { "slot: {slot():?}" }
        p { "balance: {balance():?}" }
        p { "node: {version().flatten().map(|v| v.solana_core):?}" }
    }
}

// Decode a program account into your own type, fetched then kept live:
#[component]
fn Counter(pda: Pubkey) -> Element {
    let counter = use_account_data(pda, |bytes| MyCounter::try_from_slice(bytes).ok());
    rsx! { p { "count: {counter():?}" } }
}
```

To point at your own RPC (a private validator or a paid provider) instead of the rate-limited
public endpoints, hand `RpcProvider` a custom cluster (a nested `WalletProvider` inherits it):

```rust
let cluster = Cluster::custom(Cluster::MainnetBeta, "https://my.rpc", "wss://my.rpc");
rsx! {
    RpcProvider { cluster,          // hooks talk to your endpoint; wallet signs on mainnet
        WalletProvider { autoconnect: true, App {} }
    }
}
```

(`RpcProvider` also accepts one-off `rpc_url` / `ws_url` prop overrides without a custom cluster.)

Available hooks:

- **Fetch:** one `use_get_*` per JSON-RPC read method — account/balance
  (`use_get_account`, `use_get_balance`, `use_get_multiple_accounts`,
  `use_get_program_accounts`), tokens (`use_get_token_account_balance`, `use_get_token_supply`,
  `use_get_token_accounts_by_owner`, …), blocks/slots (`use_get_slot`, `use_get_block`,
  `use_get_block_height`, `use_get_blocks`, …), transactions (`use_get_transaction`,
  `use_get_signature_statuses`, `use_get_latest_blockhash`, `use_get_fee_for_message`,
  `use_simulate_transaction`), and node/network info (`use_get_health`, `use_get_version`,
  `use_get_epoch_info`, `use_get_cluster_nodes`, `use_get_vote_accounts`, `use_get_inflation_*`, …).
  The generic `use_fetch` covers anything not wrapped. (Mutating calls — `sendTransaction`,
  `requestAirdrop` — are intentionally *not* auto-firing hooks; call them via `use_rpc()`.)
- **Subscription:** `use_slot_subscription`, `use_account_subscription`, `use_root_subscription`,
  `use_slot_updates_subscription`, `use_vote_subscription`, `use_logs_subscription`,
  `use_program_subscription`, `use_signature_subscription`, `use_block_subscription`, and the
  generic `use_subscription`.
- **Live values:** `use_account`, `use_account_data`, `use_balance`.
- **Client:** `use_rpc` returns the spume `WasmClient` for arbitrary calls.

The `rpc` module re-exports the spume surface and result types you need to name. spume is a browser
client, so the hooks only *do* anything on `wasm32` — but they compile on native too, so consumer
builds (and rust-analyzer) stay green everywhere.

See `examples/connect-demo` for a complete, buildable app.

## What this crate does *not* do

`dioxus-solana` handles wallet discovery/connection/signing and read-only RPC, but does not build
transactions — assemble those with the modular `solana-*` crates (`solana-transaction`,
`solana-message`, etc.), then hand the `VersionedTransaction` to the wallet to be signed (and
optionally sent).

## Platform support

Web-only today: wallet discovery and the RPC clients only *work* on `wasm32` (they use browser
APIs). The whole tree still *compiles* on native — so `cargo check --workspace` and rust-analyzer
work without a wasm toolchain — with the wasm-specific paths behind small `cfg(target_arch)` seams
(only `storage`, which needs `localStorage`, is fully gated). The core crate has no wasm
dependencies by design, so a desktop wallet implementation (local keypairs, a hardware wallet) can
be added later behind the same `Wallet` / `WalletSigner` traits without changing the public API.

On **Android Chrome**, `WalletProvider` registers [Mobile Wallet Adapter](https://docs.solanamobile.com/get-started/web/installation)
as a Wallet Standard wallet before discovery, so Phantom / Solflare / Seeker appear without a
browser extension. That registration is a no-op on desktop and iOS (use an extension or the
wallet's in-app browser there). MWA requires a [secure context](https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts)
(`https://` or `http://localhost`); a raw LAN IP over HTTP will not list the adapter. Pass
`app_identity` so the wallet shows your dapp's name; omit it and the page title / origin are used.
The identity `icon` must be a **relative** path (for example `favicon.ico`), resolved against
`uri` — absolute icon URLs are rejected by MWA 2.0 wallets. Autoconnect uses a silent
`standard:connect` so a cached mobile authorization is restored without reopening the wallet app.

## Building / testing

The example is a browser app (it does nothing useful off `wasm32`), but compiles on native so
`--workspace` stays green.

```bash
# native crates + tests
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# wasm target (facade, web impl, and the example)
cargo build --target wasm32-unknown-unknown -p dioxus-solana -p dioxus-solana-web -p connect-demo
cargo clippy -p connect-demo --target wasm32-unknown-unknown -- -D warnings

# run the example locally (requires the Dioxus CLI: cargo install dioxus-cli)
dx serve --package connect-demo
```
