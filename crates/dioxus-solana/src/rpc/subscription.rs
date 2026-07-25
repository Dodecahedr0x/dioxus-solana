//! Subscription hooks — a live WebSocket subscription, exposed as a signal.

use std::future::Future;

use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use solana_pubkey::Pubkey;

use dioxus_solana_web::rpc::config::*;
use dioxus_solana_web::rpc::response::*;
use dioxus_solana_web::rpc::{Subscription, WasmPubsubClient};

use super::RpcContext;

/// Drive any spume subscription into a signal, projecting each item with `map`.
/// `map` returning `None` skips the update (e.g. a null account).
pub(crate) fn subscribe_mapped<T, U, E, F, Fut, M>(subscribe: F, map: M) -> ReadSignal<Option<U>>
where
    T: DeserializeOwned + 'static,
    U: 'static,
    E: 'static,
    F: Fn(WasmPubsubClient) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<T>, E>> + 'static,
    M: Fn(T) -> Option<U> + Clone + 'static,
{
    let ctx = use_context::<RpcContext>();
    let mut value = use_signal(|| Option::<U>::None);
    use_future(move || {
        let ws_url = ctx.cluster.peek().ws_url().to_string();
        let subscribe = subscribe.clone();
        let map = map.clone();
        async move {
            let Ok(client) = WasmPubsubClient::connect(ws_url) else {
                return;
            };
            let Ok(mut sub) = subscribe(client).await else {
                return;
            };
            // `sub` owns an `Rc` to the connection, so it stays live after
            // `client` drops.
            while let Some(item) = sub.next().await {
                let Ok(v) = item else { continue };
                if let Some(u) = map(v) {
                    value.set(Some(u));
                }
            }
        }
    });
    value.into()
}

/// Drive a subscription whose items are `Response<T>`, storing each `value`.
fn subscribe_response<T, E, F, Fut>(subscribe: F) -> ReadSignal<Option<T>>
where
    T: DeserializeOwned + 'static,
    E: 'static,
    F: Fn(WasmPubsubClient) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<Response<T>>, E>> + 'static,
{
    subscribe_mapped(subscribe, |r: Response<T>| Some(r.value))
}

/// Escape hatch: drive any spume subscription, storing each item verbatim.
///
/// ```ignore
/// let votes = use_subscription(|c| async move { c.vote_subscribe().await });
/// ```
pub fn use_subscription<T, E, F, Fut>(subscribe: F) -> ReadSignal<Option<T>>
where
    T: DeserializeOwned + 'static,
    E: 'static,
    F: Fn(WasmPubsubClient) -> Fut + Clone + 'static,
    Fut: Future<Output = Result<Subscription<T>, E>> + 'static,
{
    subscribe_mapped(subscribe, Some)
}

/// Live slot number (`slotSubscribe`).
pub fn use_slot_subscription() -> ReadSignal<Option<u64>> {
    subscribe_mapped(
        |c| async move { c.slot_subscribe().await },
        |i: SlotInfo| Some(i.slot),
    )
}

/// Live account data for `pubkey` (`accountSubscribe`).
pub fn use_account_subscription(pubkey: Pubkey) -> ReadSignal<Option<UiAccount>> {
    subscribe_mapped(
        move |c| async move { c.account_subscribe(pubkey.to_string(), None).await },
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
    subscribe_response(move |c| {
        let filter = filter.clone();
        async move { c.logs_subscribe(filter, None).await }
    })
}

/// Live account changes owned by `program_id` (`programSubscribe`).
pub fn use_program_subscription(program_id: Pubkey) -> ReadSignal<Option<RpcKeyedAccount>> {
    subscribe_response(
        move |c| async move { c.program_subscribe(program_id.to_string(), None).await },
    )
}

/// Live status of a transaction `signature` (`signatureSubscribe`).
pub fn use_signature_subscription(signature: String) -> ReadSignal<Option<RpcSignatureResult>> {
    subscribe_response(move |c| {
        let signature = signature.clone();
        async move { c.signature_subscribe(signature, None).await }
    })
}

/// Live block updates matching `filter` (`blockSubscribe`).
pub fn use_block_subscription(
    filter: RpcBlockSubscribeFilter,
) -> ReadSignal<Option<RpcBlockUpdate>> {
    subscribe_response(move |c| {
        let filter = filter.clone();
        async move { c.block_subscribe(filter, None).await }
    })
}
