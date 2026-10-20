//! Live-value hooks — fetch immediately, then keep current via a subscription.

use std::cell::Cell;
use std::future::Future;

use dioxus::prelude::*;
use futures_util::future::join;
use serde::de::DeserializeOwned;
use solana_pubkey::Pubkey;

use super::base64_account_config;
use super::response::*;
use super::subscription::{clear, pump};
use super::{use_dep, PubsubClient, RpcClient, RpcContext, Subscription};

/// Fetch a value and subscribe to its updates concurrently, keeping whichever
/// observation is from the newest slot (so a slow fetch never overwrites a
/// fresher push). `fetch` and `map` return `(context slot, value)`.
///
/// Restarts — clearing the value — when `dep` or the cluster changes.
fn use_live<D, T, U, E, FF, FFut, SF, SFut, M>(
    dep: D,
    fetch: FF,
    subscribe: SF,
    map: M,
) -> ReadSignal<Option<U>>
where
    D: Clone + PartialEq + 'static,
    T: DeserializeOwned + 'static,
    U: 'static,
    E: 'static,
    FF: Fn(RpcClient, D) -> FFut + Clone + 'static,
    FFut: Future<Output = Option<(u64, U)>> + 'static,
    SF: Fn(PubsubClient, D) -> SFut + Clone + 'static,
    SFut: Future<Output = Result<Subscription<T>, E>> + 'static,
    M: Fn(T) -> Option<(u64, U)> + Clone + 'static,
{
    let ctx = use_context::<RpcContext>();
    let dep = use_dep(dep);
    let value = use_signal(|| Option::<U>::None);
    use_resource(move || {
        let client = ctx.client();
        let dep = dep();
        let (fetch, subscribe, map) = (fetch.clone(), subscribe.clone(), map.clone());
        async move {
            clear(value);
            let newest = Cell::new(0u64);
            let accept = |(slot, u): (u64, U)| {
                if slot >= newest.get() {
                    newest.set(slot);
                    let mut value = value;
                    value.set(Some(u));
                }
            };
            let fetched = async {
                if let Some(observation) = fetch(client, dep.clone()).await {
                    accept(observation);
                }
            };
            let pushed = pump(ctx, |c| subscribe(c, dep.clone()), map, accept);
            join(fetched, pushed).await;
        }
    });
    value.into()
}

/// Live account: fetched immediately (`getAccountInfo`), then kept current via
/// `accountSubscribe`. Follows `pubkey` and the cluster as they change.
pub fn use_account(pubkey: Pubkey) -> ReadSignal<Option<UiAccount>> {
    use_live(
        pubkey,
        |c, pubkey| async move {
            let resp = c.get_account_info(pubkey, None).await.ok()?;
            Some((resp.context.slot, resp.value?))
        },
        |c, pubkey| async move { c.account_subscribe(pubkey, None).await },
        |resp: Response<Option<UiAccount>>| Some((resp.context.slot, resp.value?)),
    )
}

/// Live, decoded account data: fetched immediately, kept current via
/// `accountSubscribe`, and decoded from the raw account bytes with `decode`.
///
/// The account is requested base64-encoded and passed to `decode` as `&[u8]`;
/// return `None` for data your type can't parse. Pair with borsh/bincode/bytemuck.
///
/// ```ignore
/// let counter = use_account_data(pda, |bytes| Counter::try_from_slice(bytes).ok());
/// ```
pub fn use_account_data<T, D>(pubkey: Pubkey, decode: D) -> ReadSignal<Option<T>>
where
    T: 'static,
    D: Fn(&[u8]) -> Option<T> + Clone + 'static,
{
    let decode_fetched = decode.clone();
    use_live(
        pubkey,
        move |c, pubkey| {
            let decode = decode_fetched.clone();
            async move {
                let resp = c
                    .get_account_info(pubkey, Some(base64_account_config()))
                    .await
                    .ok()?;
                let bytes = resp.value?.data.decode()?;
                Some((resp.context.slot, decode(&bytes)?))
            }
        },
        |c, pubkey| async move {
            c.account_subscribe(pubkey, Some(base64_account_config()))
                .await
        },
        move |resp: Response<Option<UiAccount>>| {
            let bytes = resp.value?.data.decode()?;
            Some((resp.context.slot, decode(&bytes)?))
        },
    )
}

/// Lamport balance of `pubkey`: fetched immediately (`getBalance`), then kept
/// current via `accountSubscribe`. Follows `pubkey` and the cluster as they
/// change.
pub fn use_balance(pubkey: Pubkey) -> ReadSignal<Option<u64>> {
    use_live(
        pubkey,
        |c, pubkey| async move {
            let resp = c.get_balance(pubkey, None).await.ok()?;
            Some((resp.context.slot, resp.value))
        },
        |c, pubkey| async move { c.account_subscribe(pubkey, None).await },
        |resp: Response<Option<UiAccount>>| {
            // A closed account pushes `null`: its balance is zero.
            Some((resp.context.slot, resp.value.map_or(0, |a| a.lamports)))
        },
    )
}
