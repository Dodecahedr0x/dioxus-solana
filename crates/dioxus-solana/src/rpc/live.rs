//! Live-value hooks — fetch immediately, then keep current via a subscription.

use dioxus::prelude::*;
use solana_pubkey::Pubkey;

use dioxus_solana_web::rpc::base64_account_config;
use dioxus_solana_web::rpc::response::*;

use super::subscription::{subscribe_mapped, use_account_subscription};
use super::use_rpc;

/// Live account: fetched immediately (`getAccountInfo`), then kept current via
/// `accountSubscribe`.
pub fn use_account(pubkey: Pubkey) -> ReadSignal<Option<UiAccount>> {
    let subscribed = use_account_subscription(pubkey);
    let rpc = use_rpc();
    let mut fetched = use_signal(|| None::<UiAccount>);

    use_future(move || {
        let rpc = rpc.clone();
        async move {
            if let Ok(resp) = rpc.get_account_info(pubkey.to_string(), None).await {
                if let Some(acc) = resp.value {
                    fetched.set(Some(acc));
                }
            }
        }
    });

    use_memo(move || subscribed().or(fetched())).into()
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
    T: Clone + PartialEq + 'static,
    D: Fn(&[u8]) -> Option<T> + Clone + 'static,
{
    let subscribed = {
        let decode = decode.clone();
        subscribe_mapped(
            move |c| async move {
                c.account_subscribe(pubkey.to_string(), Some(base64_account_config()))
                    .await
            },
            move |resp: Response<Option<UiAccount>>| {
                resp.value
                    .and_then(|ui| ui.data.decode())
                    .and_then(|b| decode(&b))
            },
        )
    };

    let rpc = use_rpc();
    let mut fetched = use_signal(|| None::<T>);
    use_future(move || {
        let rpc = rpc.clone();
        let decode = decode.clone();
        async move {
            if let Ok(resp) = rpc
                .get_account_info(pubkey.to_string(), Some(base64_account_config()))
                .await
            {
                if let Some(t) = resp
                    .value
                    .and_then(|ui| ui.data.decode())
                    .and_then(|b| decode(&b))
                {
                    fetched.set(Some(t));
                }
            }
        }
    });

    use_memo(move || subscribed().or(fetched())).into()
}

/// Lamport balance of `pubkey`: fetched immediately (`getBalance`), then kept
/// current via `accountSubscribe`.
pub fn use_balance(pubkey: Pubkey) -> ReadSignal<Option<u64>> {
    let account = use_account_subscription(pubkey);
    let rpc = use_rpc();
    let mut fetched = use_signal(|| Option::<u64>::None);

    use_future(move || {
        let rpc = rpc.clone();
        async move {
            if let Ok(resp) = rpc.get_balance(pubkey.to_string(), None).await {
                fetched.set(Some(resp.value));
            }
        }
    });

    use_memo(move || account().map(|a| a.lamports).or(fetched())).into()
}
