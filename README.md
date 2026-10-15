# dioxus-solana

Solana wallet connection, signing, and reactive RPC for Dioxus apps on **web, Android and iOS**,
exposed via a `WalletProvider` context and a `use_wallet()` hook, plus an `RpcProvider` with
reactive RPC hooks (`use_balance`, `use_account`, subscriptions, …). The same app code compiles
and runs on every supported platform:

| Platform | Wallets | RPC transport |
| --- | --- | --- |
| Web (`wasm32`) | Wallet Standard extensions, plus Mobile Wallet Adapter on Android Chrome | browser `fetch` / `WebSocket` ([`spume`](https://crates.io/crates/spume)) |
| Android (`dx serve --android`) | one **Mobile wallet** entry: on-device MWA wallet through JNI | Agave `solana-rpc-client` + tokio WebSocket |
| iOS (`dx serve --ios`) | one **Mobile wallet** entry: Phantom through encrypted universal links | Agave `solana-rpc-client` + tokio WebSocket |
| Desktop | **not supported** — building for a desktop target is a compile error | — |

The wallet signs. This crate does not build transactions.

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
  `crates/dioxus-solana/android` (Kotlin Mobile Wallet Adapter) through `manganis::ffi`. Native
  iOS uses `crates/dioxus-solana/ios` (Phantom universal links) through `manganis::ffi`.
- `examples/connect-demo` — a Dioxus web app exercising connect / sign-message / disconnect, a
  devnet airdrop, sending an on-chain **memo transaction** (`sign_and_send`), a live slot
  subscription, and a live balance.
- `examples/mobile-demo` — one cross-platform Dioxus app for Android, iOS **and** web: connect a
  wallet (MWA / Phantom / browser extension), a live slot and live balance, sign a message, and
  send an on-chain memo (`signAndSend` where supported, otherwise sign + RPC submit). Run with
  `dx serve --package mobile-demo --android`, `--ios`, or `--web`.

## Quickstart

Add the facade crate to your Dioxus app (web, or `mobile` for Android/iOS):

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-solana = "0.3"
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
- **Client:** `use_rpc()` returns a cloneable `RpcClient` with one typed method per JSON-RPC
  method (`get_balance`, `get_latest_blockhash`, `send_transaction`, `request_airdrop`, …) and a
  raw `send`. `PubsubClient` is its WebSocket counterpart. Address parameters accept a `Pubkey`
  or its base58 string.

Every hook and both clients behave identically on web, Android and iOS — only the transport
underneath differs (see the table at the top).

See `examples/connect-demo` (web) and `examples/mobile-demo` (Android/iOS) for complete apps.

## What this crate does *not* do

`dioxus-solana` handles wallet discovery/connection/signing and read-only RPC, but does not build
transactions — assemble those with the modular `solana-*` crates (`solana-transaction`,
`solana-message`, etc.), then hand the `VersionedTransaction` to the wallet to be signed (and
optionally sent).

## Platform support

See the table at the top. **Desktop is deliberately unsupported**: there is no desktop wallet to
connect to, so instead of a provider that silently lists zero wallets, compiling `dioxus-solana`
for a desktop target fails with a `compile_error!`. If you build a fullstack web app, the server
half compiles for the host; enable the `server` feature there (wallets are unavailable on the
server, RPC works):

```toml
[features]
server = ["dioxus/server", "dioxus-solana/server"]
```

rust-analyzer analyzes the host target by default, so in a web/mobile-only project it will show
that compile error. Point it at a supported target (`"rust-analyzer.cargo.target":
"wasm32-unknown-unknown"`) or enable the feature for analysis (`"rust-analyzer.cargo.features":
["dioxus-solana/server"]`).

On Android/iOS the RPC clients run on the tokio runtime dioxus mobile provides;
`PubsubClient::connect` returns an error if called outside one.

**iOS simulator note (dx 0.7.10):** `dx build --platform ios` ad-hoc signs the bundled
`DioxusSwiftPlugins.framework` and then modifies it, so the simulator refuses to load it ("Code
Signature Invalid") the first time a wallet is opened. Until that is fixed upstream, re-sign it
after building:

```bash
codesign -f -s - target/dx/<app>/debug/ios/<App>.app/Frameworks/DioxusSwiftPlugins.framework
```

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

```bash
# library on the host (unit tests; `server` lifts the desktop compile guard)
cargo test --workspace --exclude connect-demo --exclude mobile-demo --features dioxus-solana/server
# live devnet checks of the native RPC transport
cargo test -p dioxus-solana --features server --test native_rpc -- --ignored

# every supported target (Android needs the NDK's clang on PATH / CC_aarch64_linux_android)
cargo clippy -p dioxus-solana -p mobile-demo -p connect-demo --all-targets --target wasm32-unknown-unknown -- -D warnings
cargo clippy -p dioxus-solana -p mobile-demo --all-targets --target aarch64-apple-ios -- -D warnings
cargo clippy -p dioxus-solana -p mobile-demo --all-targets --target aarch64-linux-android -- -D warnings

# run the examples (requires the Dioxus CLI: cargo install dioxus-cli)
dx serve --package connect-demo
dx serve --package mobile-demo --web
dx serve --package mobile-demo --android
dx serve --package mobile-demo --ios
```

The mobile demo registers the `dioxussolana://` URL scheme so Phantom can redirect back after
signing on iOS.
