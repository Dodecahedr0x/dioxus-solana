# dioxus-solana 0.2 — wallet-adapter parity

**Status:** design approved, not implemented
**Target version:** 0.2.0 (not published; `dioxus-solana` does not yet exist on crates.io)

## Goal

Reach feature parity with `@solana/wallet-adapter-react` while fixing the
correctness bugs in the current tree. Four concerns drive the work: correctness,
missing features, API ergonomics, and testability.

Nothing is published yet, so breaking changes are free. This is the last point at
which the hook API can be reshaped without cost.

## Defects being fixed

| # | Defect | Location |
|---|--------|----------|
| 1 | Hooks never refetch when their arguments change | `rpc/fetch.rs`, `rpc/subscription.rs` |
| 2 | Every RPC error collapses to `None` | all of `rpc/fetch.rs` |
| 3 | No `standard:events` listener — account switches go unnoticed | `web/wallet.rs` |
| 4 | Wallet discovery is a one-shot mount snapshot | `provider.rs` |
| 5 | Subscriptions die silently when the socket closes | `rpc/subscription.rs` |
| 6 | Cluster is `.peek()`ed — runtime network switching rebuilds nothing | `provider.rs`, `rpc/mod.rs` |
| 7 | No send-and-confirm path | — |
| 8 | Desktop returns no wallets and there is no mock, so nothing is testable off-wasm | `platform.rs` |

## Crate topology

| Crate | Role | Target |
|---|---|---|
| `dioxus-solana-core` | Traits, types, `Cluster`, `RpcError`, mock wallet | all |
| `dioxus-solana-web` | Wallet Standard + spume RPC | wasm32 |
| `dioxus-solana-desktop` | **new** — native connector | native |
| `dioxus-solana` | Facade: providers, hooks, transaction helpers | all |
| `dioxus-solana-ui` | **new** — `WalletMultiButton`, `WalletModal` | all |

Three decisions worth recording:

**`RpcError` is ours, defined in core.** Today the hooks would surface
`Box<solana_rpc_client_types::request::RpcError>` from spume. Leaking a
transitively-versioned Solana type through the public signature of 40+ hooks
couples every consumer to whatever spume resolves — the same class of problem as
the existing wincode pin. A local enum costs one `From` impl:

```rust
pub enum RpcError {
    Transport(String),
    Rpc { code: i64, message: String },
    Parse(String),
    Disconnected,
}
```

It derives `Clone + PartialEq + Error`. `Clone` matters: `Resource<Result<T, E>>`
is far more ergonomic to read when the error can be cloned out.

**UI is a separate crate, not a feature flag.** Components carry markup and
styling concerns that consumers of the hooks alone should not pay for.

**The mock wallet lives in core, not behind `#[cfg(test)]`.** It is how users
test their own dapps, and it gives the desktop target something real to return.
This is what resolves defect 8.

## RPC layer

### Reactivity (defect 1)

`use_resource` already re-runs when a signal is read inside its closure, and
`use_reactive!` lifts a plain value into a signal that updates on change. The
hooks are simply missing the wrapper. Every body becomes:

```rust
use_resource(use_reactive!(|arg| async move { ... }))
```

Signatures are unchanged — `use_get_balance(pubkey: Pubkey)` stays. Callers
holding a signal pass `pk()`; the read re-renders the component, the plain value
changes, and `use_reactive` fires.

`subscribe_mapped` gets the same treatment and must additionally unsubscribe and
resubscribe when its argument changes. `use_future` cancels the previous task on
re-run and `Subscription`'s drop closes the subscription, so this mostly falls
out of the existing code — but it needs an explicit test.

### Errors (defect 2)

All `.ok()` calls are removed. `use_fetch` becomes:

```rust
pub fn use_fetch<T, F, Fut>(fetch: F) -> Resource<Result<T, RpcError>>
where Fut: Future<Output = Result<T, RpcError>>
```

This restores a distinction the current code destroys: `use_get_account` returns
`Result<Option<UiAccount>, RpcError>`, so "the RPC says this account does not
exist" and "the RPC did not answer" stop being the same value. Account-existence
checks are silently wrong today.

Loading state comes from `Resource::state()`; no new concepts are introduced.

### Cluster switching (defect 6)

`use_rpc()` currently does `use_hook(|| WasmClient::new(cluster.peek()...))` —
built once, never rebuilt. It becomes a `Memo<WasmClient>` keyed on the cluster
signal, so switching networks rebuilds the client and re-runs every dependent
resource. `RpcProvider` must also accept a reactive cluster prop instead of
folding props into a `use_signal(|| resolved)` that ignores later prop changes.

### Reconnect (defect 5)

`subscribe_mapped`'s `while let Some(item) = sub.next().await` exits silently on
close. It gets a retry loop with capped exponential backoff (250ms → 8s,
jittered). Status is published through:

```rust
pub fn use_rpc_status() -> ReadSignal<ConnectionStatus>
```

`PubsubProvider::is_connected()` already exists in spume 0.3; nothing uses it.

## Wallet layer

### Account and chain changes (defect 3)

`StandardWallet::connect` registers a `"change"` listener via the
`standard:events` feature, which currently appears nowhere in `src/`.

Two constraints:

- The listener must outlive `connect()`. Its `Closure` is owned by the returned
  `ConnectedAccount`, mirroring `_register_cb` in `WalletRegistry`. Calling
  `.forget()` instead would leak it on every disconnect.
- A change event carrying `accounts: []` means the wallet disconnected on its
  own. It must drive `WalletState::Disconnected`, or the app renders a stale
  pubkey after the user locks their wallet.

Because the pubkey can now change during a live session, `StandardSigner.pubkey`
moves from a plain field to interior-mutable state, and the core trait grows a
defaulted method so existing implementors are unaffected:

```rust
trait WalletSigner {
    /// Subscribe to account/chain changes. Returns a guard; drop to unsubscribe.
    fn on_change(&self, f: Box<dyn Fn(WalletChange)>) -> Option<ChangeGuard> { None }
}
```

### Live discovery (defect 4)

The limitation is self-documented in `provider.rs`. `WalletRegistry` pushes into
a `RefCell<Vec<JsValue>>` that the provider snapshots exactly once. The registry
instead takes a callback at construction; `register` pushes *and* notifies, so a
late-loading wallet appears immediately. The `EventListener` is already retained
for the provider's lifetime, so late `register-wallet` events do arrive — only
the plumbing to the signal is missing.

**Autoconnect race.** Autoconnect reads the same mount-time snapshot, so a
remembered wallet that registers late never reconnects. With live discovery it
becomes "attempt once, when the remembered wallet appears."

### Desktop

`dioxus-solana-desktop` initially ships the mock/keypair signer (local keypair
file, explicit opt-in, dev-only) so `use_wallet()` is not a dead API off-wasm.
Real desktop wallet connectivity needs a deeplink/relay design and is scoped
separately rather than guessed at here.

## Transactions (defect 7)

Today `sign_and_send(tx)` requires the caller to build a `VersionedTransaction`
with a fresh blockhash and fee payer, then poll for confirmation. Every dapp
rewrites this.

```rust
let sig = wallet.tx()
    .add(memo_instruction("gm"))
    .send_and_confirm()
    .await?;
```

`send_and_confirm` prefers `signAndSendTransaction` when the wallet advertises it
— the wallet then picks its own RPC, which is what users expect from Phantom —
and falls back to `signTransaction` plus our own `sendTransaction` when it does
not. That branch is the main justification for the abstraction; it is what
consumers would otherwise get subtly wrong.

Confirmation uses `signatureSubscribe` with a slot-based timeout, falling back to
`getSignatureStatuses` polling when the socket is down. It returns a typed
`TxError` separating user-rejected, blockhash-expired, simulation-failed, and
timeout — the four cases a UI renders differently.

`use_send_transaction()` wraps this with reactive status
(`Idle → Signing → Sending → Confirming → Done/Failed`) for button states.

**Excluded:** priority-fee estimation and compute-budget auto-tuning. It is
provider-specific, ages badly, and guessing wrong costs users money. Callers get
`.with_compute_budget(..)` and decide for themselves.

## UI crate

Modeled on `@solana/wallet-adapter-react-ui`:

- `WalletMultiButton` — the entire flow in one component: "Select Wallet" →
  modal → connected pubkey with a dropdown (copy address, change wallet,
  disconnect).
- `WalletModal` — picker with icons, *Installed* and *Not installed* sections
  with install links, and an empty state.

Unstyled-first. Every element carries a stable `dxsol-*` class name; an opt-in
`default-theme` feature ships baseline CSS. Tailwind users are left alone,
five-minute users get something that works.

## Testing

- **Core** — the mock wallet drives trait-level tests off-wasm under native
  `cargo test`.
- **Web** — the existing `wasm-bindgen-test` suite extends to cover change
  events and late registration.
- **Hooks** — the current blind spot: no hook is tested at all. A headless-Chrome
  harness renders a `VirtualDom` against a mock RPC and asserts reactivity (arg
  change refetches), error propagation, and reconnect. This is what makes the
  rest of the refactor safe to perform.
- **CI** — the existing workflow gains a wasm test job.

## Sequencing

Phases are independently reviewable; all land under 0.2.0 before any release.

1. **Foundation** — `RpcError` in core, mock wallet in core, hook test harness.
2. **RPC correctness** — reactivity, error surfacing, cluster switching, reconnect.
3. **Wallet correctness** — change events, live discovery, autoconnect race.
4. **Transactions** — builder, confirmation, `use_send_transaction`.
5. **UI** — `dioxus-solana-ui`.
6. **Desktop** — `dioxus-solana-desktop` with the keypair signer.
7. **Docs and demo** — README rewrite, demo exercises the new API.

Phase 1 comes first because phases 2 and 3 are large refactors of untested code.
