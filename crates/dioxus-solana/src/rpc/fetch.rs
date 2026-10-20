//! Fetch hooks — one HTTP request per JSON-RPC read method, returning a
//! [`Resource`]`<Option<T>>`.

use std::future::Future;
use std::time::Duration;

use dioxus::prelude::*;
use solana_pubkey::Pubkey;

use super::config::*;
use super::response::*;
use super::{
    EncodedConfirmedTransactionWithStatusMeta, EpochInfo, EpochSchedule, RpcClient,
    TransactionStatus,
};

use super::{sleep, use_dep, FetchError, RpcContext};

/// Run one HTTP request against the client.
///
/// Returns a [`Resource`]`<Result<T, FetchError>>`: `None` while loading, then
/// the outcome. [`FetchResult::data`] / [`FetchResult::error`] read either side;
/// `res.restart()` refetches. Refetches automatically when the cluster changes
/// or a signal read inside `fetch` (before its first `.await`) changes. Values
/// captured by the closure are fixed at first render — to key a fetch on a
/// changing value, use [`use_fetch_with`].
///
/// ```ignore
/// let health = use_fetch(|c| async move { c.get_health().await });
/// ```
pub fn use_fetch<T, E, F, Fut>(fetch: F) -> Resource<Result<T, FetchError>>
where
    T: 'static,
    E: Into<FetchError>,
    F: Fn(RpcClient) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    use_fetch_with((), move |c, ()| fetch(c))
}

/// [`use_fetch`] keyed by `dep`: refetches whenever `dep` changes between
/// renders (e.g. a route parameter or a prop), on cluster changes, and on
/// [`RpcContext::invalidate`].
/// This backs every `use_get_*` hook that takes an argument.
///
/// ```ignore
/// let block = use_fetch_with(slot, |c, slot| async move { c.get_block(slot, None).await });
/// ```
pub fn use_fetch_with<D, T, E, F, Fut>(dep: D, fetch: F) -> Resource<Result<T, FetchError>>
where
    D: Clone + PartialEq + 'static,
    T: 'static,
    E: Into<FetchError>,
    F: Fn(RpcClient, D) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let ctx = use_context::<RpcContext>();
    let dep = use_dep(dep);
    use_resource(move || {
        ctx.track_invalidation();
        let fut = fetch(ctx.client(), dep());
        async move { fut.await.map_err(Into::into) }
    })
}

/// Refetch `resource` every `period` while the component is mounted — for
/// values without a subscription, like epoch info or performance samples.
/// The previous value stays readable while a refetch is in flight.
///
/// ```ignore
/// let epoch = use_get_epoch_info();
/// use_refresh_every(epoch, Duration::from_secs(10));
/// ```
pub fn use_refresh_every<T: 'static>(mut resource: Resource<T>, period: Duration) {
    use_future(move || async move {
        loop {
            sleep(period).await;
            resource.restart();
        }
    });
}

/// Newest transaction format the fetch hooks accept (v1 is live on devnet).
pub const MAX_SUPPORTED_TRANSACTION_VERSION: u8 = 1;

/// `getBlock` config accepting versioned transactions — without it the node
/// rejects any block containing a non-legacy transaction.
pub fn block_config() -> RpcBlockConfig {
    RpcBlockConfig {
        max_supported_transaction_version: Some(MAX_SUPPORTED_TRANSACTION_VERSION),
        ..Default::default()
    }
}

/// `getTransaction` config accepting versioned transactions — without it the
/// node rejects every non-legacy transaction.
pub fn transaction_config() -> RpcEncodingConfigWrapper<RpcTransactionConfig> {
    RpcEncodingConfigWrapper::Current(Some(RpcTransactionConfig {
        max_supported_transaction_version: Some(MAX_SUPPORTED_TRANSACTION_VERSION),
        ..Default::default()
    }))
}

// --- No / config-only argument ---

/// `getSlot`.
pub fn use_get_slot() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_slot(None).await })
}
/// `getBlockHeight`.
pub fn use_get_block_height() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_block_height(None).await })
}
/// `getTransactionCount`.
pub fn use_get_transaction_count() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_transaction_count(None).await })
}
/// `getSlotLeader`.
pub fn use_get_slot_leader() -> Resource<Result<String, FetchError>> {
    use_fetch(|c| async move { c.get_slot_leader(None).await })
}
/// `getFirstAvailableBlock`.
pub fn use_get_first_available_block() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_first_available_block().await })
}
/// `minimumLedgerSlot`.
pub fn use_minimum_ledger_slot() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.minimum_ledger_slot().await })
}
/// `getMaxRetransmitSlot`.
pub fn use_get_max_retransmit_slot() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_max_retransmit_slot().await })
}
/// `getMaxShredInsertSlot`.
pub fn use_get_max_shred_insert_slot() -> Resource<Result<u64, FetchError>> {
    use_fetch(|c| async move { c.get_max_shred_insert_slot().await })
}
/// `getHealth` — `"ok"` when healthy.
pub fn use_get_health() -> Resource<Result<String, FetchError>> {
    use_fetch(|c| async move { c.get_health().await })
}
/// `getGenesisHash`.
pub fn use_get_genesis_hash() -> Resource<Result<String, FetchError>> {
    use_fetch(|c| async move { c.get_genesis_hash().await })
}
/// `getVersion`.
pub fn use_get_version() -> Resource<Result<RpcVersionInfo, FetchError>> {
    use_fetch(|c| async move { c.get_version().await })
}
/// `getIdentity`.
pub fn use_get_identity() -> Resource<Result<RpcIdentity, FetchError>> {
    use_fetch(|c| async move { c.get_identity().await })
}
/// `getEpochInfo`.
pub fn use_get_epoch_info() -> Resource<Result<EpochInfo, FetchError>> {
    use_fetch(|c| async move { c.get_epoch_info(None).await })
}
/// `getEpochSchedule`.
pub fn use_get_epoch_schedule() -> Resource<Result<EpochSchedule, FetchError>> {
    use_fetch(|c| async move { c.get_epoch_schedule().await })
}
/// `getHighestSnapshotSlot`.
pub fn use_get_highest_snapshot_slot() -> Resource<Result<RpcSnapshotSlotInfo, FetchError>> {
    use_fetch(|c| async move { c.get_highest_snapshot_slot().await })
}
/// `getClusterNodes`.
pub fn use_get_cluster_nodes() -> Resource<Result<Vec<RpcContactInfo>, FetchError>> {
    use_fetch(|c| async move { c.get_cluster_nodes().await })
}
/// `getVoteAccounts`.
pub fn use_get_vote_accounts() -> Resource<Result<RpcVoteAccountStatus, FetchError>> {
    use_fetch(|c| async move { c.get_vote_accounts(None).await })
}
/// `getInflationGovernor`.
pub fn use_get_inflation_governor() -> Resource<Result<RpcInflationGovernor, FetchError>> {
    use_fetch(|c| async move { c.get_inflation_governor(None).await })
}
/// `getInflationRate`.
pub fn use_get_inflation_rate() -> Resource<Result<RpcInflationRate, FetchError>> {
    use_fetch(|c| async move { c.get_inflation_rate().await })
}
/// `getLargestAccounts`.
pub fn use_get_largest_accounts() -> Resource<Result<Vec<RpcAccountBalance>, FetchError>> {
    use_fetch(|c| async move { c.get_largest_accounts(None).await.map(|r| r.value) })
}
/// `getLatestBlockhash`.
pub fn use_get_latest_blockhash() -> Resource<Result<RpcBlockhash, FetchError>> {
    use_fetch(|c| async move { c.get_latest_blockhash(None).await.map(|r| r.value) })
}
/// `getBlockProduction`.
pub fn use_get_block_production() -> Resource<Result<RpcBlockProduction, FetchError>> {
    use_fetch(|c| async move { c.get_block_production(None).await.map(|r| r.value) })
}
/// `getLeaderSchedule` for the current epoch.
pub fn use_get_leader_schedule() -> Resource<Result<Option<RpcLeaderSchedule>, FetchError>> {
    use_fetch(|c| async move { c.get_leader_schedule(None, None).await })
}

// --- Address argument ---

/// `getBalance`.
pub fn use_get_balance(pubkey: Pubkey) -> Resource<Result<u64, FetchError>> {
    use_fetch_with(pubkey, |c, pubkey| async move {
        c.get_balance(pubkey, None).await.map(|r| r.value)
    })
}
/// `getAccountInfo`. `Ok(None)` means the account does not exist.
pub fn use_get_account(pubkey: Pubkey) -> Resource<Result<Option<UiAccount>, FetchError>> {
    use_fetch_with(pubkey, |c, pubkey| async move {
        c.get_account_info(pubkey, None).await.map(|r| r.value)
    })
}
/// `getTokenAccountBalance`.
pub fn use_get_token_account_balance(
    account: Pubkey,
) -> Resource<Result<UiTokenAmount, FetchError>> {
    use_fetch_with(account, |c, account| async move {
        c.get_token_account_balance(account, None)
            .await
            .map(|r| r.value)
    })
}
/// `getTokenLargestAccounts`.
pub fn use_get_token_largest_accounts(
    mint: Pubkey,
) -> Resource<Result<Vec<RpcTokenAccountBalance>, FetchError>> {
    use_fetch_with(mint, |c, mint| async move {
        c.get_token_largest_accounts(mint, None)
            .await
            .map(|r| r.value)
    })
}
/// `getTokenSupply`.
pub fn use_get_token_supply(mint: Pubkey) -> Resource<Result<UiTokenAmount, FetchError>> {
    use_fetch_with(mint, |c, mint| async move {
        c.get_token_supply(mint, None).await.map(|r| r.value)
    })
}
/// `getProgramAccounts`.
pub fn use_get_program_accounts(
    program_id: Pubkey,
) -> Resource<Result<Vec<RpcKeyedAccount>, FetchError>> {
    use_fetch_with(program_id, |c, program_id| async move {
        c.get_program_accounts(program_id, None)
            .await
            .map(|oc| oc.parse_value())
    })
}
/// `getSignaturesForAddress`.
pub fn use_get_signatures_for_address(
    address: Pubkey,
) -> Resource<Result<Vec<RpcConfirmedTransactionStatusWithSignature>, FetchError>> {
    use_fetch_with(address, |c, address| async move {
        c.get_signatures_for_address(address, None).await
    })
}

// --- Slot / numeric argument ---

/// `getBlockTime` (unix seconds).
pub fn use_get_block_time(slot: u64) -> Resource<Result<Option<i64>, FetchError>> {
    use_fetch_with(slot, |c, slot| async move { c.get_block_time(slot).await })
}
/// `getBlock`.
pub fn use_get_block(slot: u64) -> Resource<Result<UiConfirmedBlock, FetchError>> {
    use_fetch_with(slot, |c, slot| async move {
        c.get_block(
            slot,
            Some(RpcEncodingConfigWrapper::Current(Some(block_config()))),
        )
        .await
    })
}
/// `getBlockCommitment`.
pub fn use_get_block_commitment(
    slot: u64,
) -> Resource<Result<RpcBlockCommitment<Vec<usize>>, FetchError>> {
    use_fetch_with(
        slot,
        |c, slot| async move { c.get_block_commitment(slot).await },
    )
}
/// `getMinimumBalanceForRentExemption`.
pub fn use_get_minimum_balance_for_rent_exemption(
    data_size: u64,
) -> Resource<Result<u64, FetchError>> {
    use_fetch_with(data_size, |c, data_size| async move {
        c.get_minimum_balance_for_rent_exemption(data_size, None)
            .await
    })
}
/// `getSlotLeaders`.
pub fn use_get_slot_leaders(
    start_slot: u64,
    limit: u64,
) -> Resource<Result<Vec<String>, FetchError>> {
    use_fetch_with((start_slot, limit), |c, (start_slot, limit)| async move {
        c.get_slot_leaders(start_slot, limit).await
    })
}
/// `getBlocks` in `[start_slot, end_slot]` (end defaults to latest).
pub fn use_get_blocks(
    start_slot: u64,
    end_slot: Option<u64>,
) -> Resource<Result<Vec<u64>, FetchError>> {
    use_fetch_with(
        (start_slot, end_slot),
        |c, (start_slot, end_slot)| async move { c.get_blocks(start_slot, end_slot, None).await },
    )
}
/// `getBlocksWithLimit`.
pub fn use_get_blocks_with_limit(
    start_slot: u64,
    limit: u64,
) -> Resource<Result<Vec<u64>, FetchError>> {
    use_fetch_with((start_slot, limit), |c, (start_slot, limit)| async move {
        c.get_blocks_with_limit(start_slot, limit, None).await
    })
}
/// `getRecentPerformanceSamples`.
pub fn use_get_recent_performance_samples(
    limit: Option<u32>,
) -> Resource<Result<Vec<RpcPerfSample>, FetchError>> {
    use_fetch_with(limit, |c, limit| async move {
        c.get_recent_performance_samples(limit).await
    })
}

// --- String argument ---

/// `getTransaction`. `Ok(None)` means the transaction was not found.
pub fn use_get_transaction(
    signature: String,
) -> Resource<Result<Option<EncodedConfirmedTransactionWithStatusMeta>, FetchError>> {
    use_fetch_with(signature, |c, signature| async move {
        c.get_transaction(signature, Some(transaction_config()))
            .await
    })
}
/// `getSignatureStatuses`.
pub fn use_get_signature_statuses(
    signatures: Vec<String>,
) -> Resource<Result<Vec<Option<TransactionStatus>>, FetchError>> {
    use_fetch_with(signatures, |c, signatures| async move {
        c.get_signature_statuses(signatures, None)
            .await
            .map(|r| r.value)
    })
}
/// `isBlockhashValid`.
pub fn use_is_blockhash_valid(blockhash: String) -> Resource<Result<bool, FetchError>> {
    use_fetch_with(blockhash, |c, blockhash| async move {
        c.is_blockhash_valid(blockhash, None).await.map(|r| r.value)
    })
}
/// `getFeeForMessage` (base64 message).
pub fn use_get_fee_for_message(message: String) -> Resource<Result<Option<u64>, FetchError>> {
    use_fetch_with(message, |c, message| async move {
        c.get_fee_for_message(message, None).await.map(|r| r.value)
    })
}
/// `simulateTransaction` (base64 transaction). Read-only — no state change.
pub fn use_simulate_transaction(
    transaction: String,
) -> Resource<Result<RpcSimulateTransactionResult, FetchError>> {
    use_fetch_with(transaction, |c, transaction| async move {
        c.simulate_transaction(transaction, None)
            .await
            .map(|r| r.value)
    })
}

// --- Address-list / filter argument ---

/// `getMultipleAccounts`.
pub fn use_get_multiple_accounts(
    pubkeys: Vec<Pubkey>,
) -> Resource<Result<Vec<Option<UiAccount>>, FetchError>> {
    use_fetch_with(pubkeys, |c, pubkeys| {
        let addrs: Vec<String> = pubkeys.iter().map(|p| p.to_string()).collect();
        async move { c.get_multiple_accounts(&addrs, None).await.map(|r| r.value) }
    })
}
/// `getInflationReward` for the given addresses.
pub fn use_get_inflation_reward(
    pubkeys: Vec<Pubkey>,
) -> Resource<Result<Vec<Option<RpcInflationReward>>, FetchError>> {
    use_fetch_with(pubkeys, |c, pubkeys| async move {
        c.get_inflation_reward(&pubkeys, None).await
    })
}
/// `getRecentPrioritizationFees`, optionally filtered to accounts.
pub fn use_get_recent_prioritization_fees(
    addresses: Option<Vec<Pubkey>>,
) -> Resource<Result<Vec<RpcPrioritizationFee>, FetchError>> {
    use_fetch_with(addresses, |c, addresses| async move {
        c.get_recent_prioritization_fees(addresses.as_deref()).await
    })
}
/// `getTokenAccountsByOwner`.
pub fn use_get_token_accounts_by_owner(
    owner: Pubkey,
    filter: RpcTokenAccountsFilter,
) -> Resource<Result<Vec<RpcKeyedAccount>, FetchError>> {
    use_fetch_with((owner, filter), |c, (owner, filter)| async move {
        c.get_token_accounts_by_owner(owner, filter, None)
            .await
            .map(|r| r.value)
    })
}
/// `getTokenAccountsByDelegate`.
pub fn use_get_token_accounts_by_delegate(
    delegate: Pubkey,
    filter: RpcTokenAccountsFilter,
) -> Resource<Result<Vec<RpcKeyedAccount>, FetchError>> {
    use_fetch_with((delegate, filter), |c, (delegate, filter)| async move {
        c.get_token_accounts_by_delegate(delegate, filter, None)
            .await
            .map(|r| r.value)
    })
}
