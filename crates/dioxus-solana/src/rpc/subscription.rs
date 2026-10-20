//! Subscription hooks — a live WebSocket subscription, exposed as a signal.

use std::future::Future;
use std::time::Duration;

use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use solana_pubkey::Pubkey;

use super::client::sleep;
use super::config::*;
use super::response::*;
use super::{use_dep, PubsubClient, RpcContext, Subscription};

const MIN_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Run `subscribe` on the provider's shared socket and feed every item `map`
/// keeps to `sink`. Never returns: when the socket drops it is reset and the
/// subscription re-established with exponential backoff.
pub(crate) async fn pump<T, U, E, Fut>(
    ctx: RpcContext,
    subscribe: impl Fn(PubsubClient) -> Fut,
    map: impl Fn(T) -> Option<U>,
    mut sink: impl FnMut(U),
) where
    T: DeserializeOwned,
    Fut: Future<Output = Result<Subscription<T>, E>>,
{
    let mut backoff = MIN_BACKOFF;
    loop {
        if let Ok((generation, client)) = ctx.pubsub() {
            if let Ok(mut sub) = subscribe(client).await {
                backoff = MIN_BACKOFF;
                // Items are only `Err` on disconnect, right before the stream ends.
                while let Some(item) = sub.next().await {
                    if let Some(u) = item.ok().and_then(&map) {
                        sink(u);
                    }
                }
                // The stream only ends when the socket does.
                ctx.reset_pubsub(generation);
            }
        }
        sleep(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }
}

/// Clear `value` without notifying readers if it is already empty.
pub(crate) fn clear<U: 'static>(mut value: Signal<Option<U>>) {
    if value.peek().is_some() {
        value.set(None);
    }
}

/// Drive a subscription keyed by `dep` into a signal, projecting each item with
/// `map` (`None` skips the update, e.g. a null account).
///
/// Re-subscribes — clearing the value — whenever `dep` or the cluster changes.
pub(crate) fn subscribe_mapped<D, T, U, E, F, Fut, M>(
    dep: D,
    subscribe: F,
    map: M,
) -> ReadSignal<Option<U>>
where
    D: Clone + PartialEq + 'static,
    T: DeserializeOwned + 'static,
    U: 'static,
    E: 'static,
    F: Fn(PubsubClient, D) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<T>, E>> + 'static,
    M: Fn(T) -> Option<U> + Clone + 'static,
{
    let ctx = use_context::<RpcContext>();
    let dep = use_dep(dep);
    let mut value = use_signal(|| Option::<U>::None);
    use_resource(move || {
        // Read reactively: an endpoint or dependency change restarts the task.
        ctx.track_endpoint();
        let dep = dep();
        let subscribe = subscribe.clone();
        let map = map.clone();
        async move {
            clear(value);
            pump(
                ctx,
                |client| subscribe(client, dep.clone()),
                map,
                |u| value.set(Some(u)),
            )
            .await
        }
    });
    value.into()
}

/// Drive a subscription whose items are `Response<T>`, storing each `value`.
fn subscribe_response<D, T, E, F, Fut>(dep: D, subscribe: F) -> ReadSignal<Option<T>>
where
    D: Clone + PartialEq + 'static,
    T: DeserializeOwned + 'static,
    E: 'static,
    F: Fn(PubsubClient, D) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<Response<T>>, E>> + 'static,
{
    subscribe_mapped(dep, subscribe, |r: Response<T>| Some(r.value))
}

/// Escape hatch: drive any subscription, storing each item verbatim.
///
/// The closure may capture values, but they are fixed at first render; for a
/// subscription keyed by a value that changes, use [`use_subscription_with`].
///
/// ```ignore
/// let votes = use_subscription(|c| async move { c.vote_subscribe().await });
/// ```
pub fn use_subscription<T, E, F, Fut>(subscribe: F) -> ReadSignal<Option<T>>
where
    T: DeserializeOwned + 'static,
    E: 'static,
    F: Fn(PubsubClient) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<T>, E>> + 'static,
{
    subscribe_mapped((), move |c, ()| subscribe(c), Some)
}

/// [`use_subscription`] keyed by `dep`: re-subscribes (clearing the value)
/// whenever `dep` changes between renders.
///
/// ```ignore
/// let logs = use_subscription_with(program_id, |c, id| async move {
///     c.logs_subscribe(RpcTransactionLogsFilter::Mentions(vec![id.to_string()]), None).await
/// });
/// ```
pub fn use_subscription_with<D, T, E, F, Fut>(dep: D, subscribe: F) -> ReadSignal<Option<T>>
where
    D: Clone + PartialEq + 'static,
    T: DeserializeOwned + 'static,
    E: 'static,
    F: Fn(PubsubClient, D) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<T>, E>> + 'static,
{
    subscribe_mapped(dep, subscribe, Some)
}

/// Live slot number (`slotSubscribe`).
pub fn use_slot_subscription() -> ReadSignal<Option<u64>> {
    subscribe_mapped(
        (),
        |c, ()| async move { c.slot_subscribe().await },
        |i: SlotInfo| Some(i.slot),
    )
}

/// Live account data for `pubkey` (`accountSubscribe`).
pub fn use_account_subscription(pubkey: Pubkey) -> ReadSignal<Option<UiAccount>> {
    subscribe_mapped(
        pubkey,
        |c, pubkey| async move { c.account_subscribe(pubkey, None).await },
        |resp: Response<Option<UiAccount>>| resp.value,
    )
}

/// Live root slot (`rootSubscribe`).
pub fn use_root_subscription() -> ReadSignal<Option<u64>> {
    use_subscription(|c| async move { c.root_subscribe().await })
}

/// Live slot-progress updates (`slotsUpdatesSubscribe`).
pub fn use_slot_updates_subscription() -> ReadSignal<Option<SlotUpdate>> {
    use_subscription(|c| async move { c.slots_updates_subscribe().await })
}

/// Live vote notifications (`voteSubscribe`; requires a node with votes enabled).
pub fn use_vote_subscription() -> ReadSignal<Option<RpcVote>> {
    use_subscription(|c| async move { c.vote_subscribe().await })
}

/// Live transaction logs matching `filter` (`logsSubscribe`).
pub fn use_logs_subscription(
    filter: RpcTransactionLogsFilter,
) -> ReadSignal<Option<RpcLogsResponse>> {
    subscribe_response(filter, |c, filter| async move {
        c.logs_subscribe(filter, None).await
    })
}

/// Live account changes owned by `program_id` (`programSubscribe`).
pub fn use_program_subscription(program_id: Pubkey) -> ReadSignal<Option<RpcKeyedAccount>> {
    subscribe_response(program_id, |c, program_id| async move {
        c.program_subscribe(program_id, None).await
    })
}

/// Live status of a transaction `signature` (`signatureSubscribe`).
pub fn use_signature_subscription(signature: String) -> ReadSignal<Option<RpcSignatureResult>> {
    subscribe_response(signature, |c, signature| async move {
        c.signature_subscribe(signature, None).await
    })
}

/// Live block updates matching `filter` (`blockSubscribe`).
pub fn use_block_subscription(
    filter: RpcBlockSubscribeFilter,
) -> ReadSignal<Option<RpcBlockUpdate>> {
    subscribe_response(filter, |c, filter| async move {
        c.block_subscribe(filter, None).await
    })
}
