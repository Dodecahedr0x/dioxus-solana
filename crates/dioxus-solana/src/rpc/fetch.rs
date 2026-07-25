//! Fetch hooks — one HTTP request per JSON-RPC read method, returning a
//! [`Resource`]`<Option<T>>`.

use std::future::Future;

use dioxus::prelude::*;
use solana_pubkey::Pubkey;

use dioxus_solana_web::rpc::config::*;
use dioxus_solana_web::rpc::response::*;
use dioxus_solana_web::rpc::{
    EncodedConfirmedTransactionWithStatusMeta, EpochInfo, EpochSchedule, TransactionStatus,
    WasmClient,
};

use super::use_rpc;

/// Run one HTTP request against the client, mapping errors to `None`.
///
/// Returns a [`Resource`]: read the value with `res().flatten()` and call
/// `res.restart()` to refetch. This backs every `use_get_*` hook and covers any
/// method without a dedicated one.
///
/// ```ignore
/// let block = use_fetch(move |c| async move { c.get_block(slot, None).await.ok() });
/// ```
pub fn use_fetch<T, F, Fut>(fetch: F) -> Resource<Option<T>>
where
    T: 'static,
    F: Fn(WasmClient) -> Fut + 'static,
    Fut: Future<Output = Option<T>> + 'static,
{
    let rpc = use_rpc();
    use_resource(move || fetch(rpc.clone()))
}

// --- No / config-only argument ---

/// `getSlot`.
pub fn use_get_slot() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_slot(None).await.ok() })
}
/// `getBlockHeight`.
pub fn use_get_block_height() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_block_height(None).await.ok() })
}
/// `getTransactionCount`.
pub fn use_get_transaction_count() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_transaction_count(None).await.ok() })
}
/// `getSlotLeader`.
pub fn use_get_slot_leader() -> Resource<Option<String>> {
    use_fetch(|c| async move { c.get_slot_leader(None).await.ok() })
}
/// `getFirstAvailableBlock`.
pub fn use_get_first_available_block() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_first_available_block().await.ok() })
}
/// `minimumLedgerSlot`.
pub fn use_minimum_ledger_slot() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.minimum_ledger_slot().await.ok() })
}
/// `getMaxRetransmitSlot`.
pub fn use_get_max_retransmit_slot() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_max_retransmit_slot().await.ok() })
}
/// `getMaxShredInsertSlot`.
pub fn use_get_max_shred_insert_slot() -> Resource<Option<u64>> {
    use_fetch(|c| async move { c.get_max_shred_insert_slot().await.ok() })
}
/// `getHealth` — `"ok"` when healthy.
pub fn use_get_health() -> Resource<Option<String>> {
    use_fetch(|c| async move { c.get_health().await.ok() })
}
/// `getGenesisHash`.
pub fn use_get_genesis_hash() -> Resource<Option<String>> {
    use_fetch(|c| async move { c.get_genesis_hash().await.ok() })
}
/// `getVersion`.
pub fn use_get_version() -> Resource<Option<RpcVersionInfo>> {
    use_fetch(|c| async move { c.get_version().await.ok() })
}
/// `getIdentity`.
pub fn use_get_identity() -> Resource<Option<RpcIdentity>> {
    use_fetch(|c| async move { c.get_identity().await.ok() })
}
/// `getEpochInfo`.
pub fn use_get_epoch_info() -> Resource<Option<EpochInfo>> {
    use_fetch(|c| async move { c.get_epoch_info(None).await.ok() })
}
/// `getEpochSchedule`.
pub fn use_get_epoch_schedule() -> Resource<Option<EpochSchedule>> {
    use_fetch(|c| async move { c.get_epoch_schedule().await.ok() })
}
/// `getHighestSnapshotSlot`.
pub fn use_get_highest_snapshot_slot() -> Resource<Option<RpcSnapshotSlotInfo>> {
    use_fetch(|c| async move { c.get_highest_snapshot_slot().await.ok() })
}
/// `getClusterNodes`.
pub fn use_get_cluster_nodes() -> Resource<Option<Vec<RpcContactInfo>>> {
    use_fetch(|c| async move { c.get_cluster_nodes().await.ok() })
}
/// `getVoteAccounts`.
pub fn use_get_vote_accounts() -> Resource<Option<RpcVoteAccountStatus>> {
    use_fetch(|c| async move { c.get_vote_accounts(None).await.ok() })
}
/// `getInflationGovernor`.
pub fn use_get_inflation_governor() -> Resource<Option<RpcInflationGovernor>> {
    use_fetch(|c| async move { c.get_inflation_governor(None).await.ok() })
}
/// `getInflationRate`.
pub fn use_get_inflation_rate() -> Resource<Option<RpcInflationRate>> {
    use_fetch(|c| async move { c.get_inflation_rate().await.ok() })
}
/// `getLargestAccounts`.
pub fn use_get_largest_accounts() -> Resource<Option<Vec<RpcAccountBalance>>> {
    use_fetch(|c| async move { c.get_largest_accounts(None).await.ok().map(|r| r.value) })
}
/// `getLatestBlockhash`.
pub fn use_get_latest_blockhash() -> Resource<Option<RpcBlockhash>> {
    use_fetch(|c| async move { c.get_latest_blockhash(None).await.ok().map(|r| r.value) })
}
/// `getBlockProduction`.
pub fn use_get_block_production() -> Resource<Option<RpcBlockProduction>> {
    use_fetch(|c| async move { c.get_block_production(None).await.ok().map(|r| r.value) })
}
/// `getLeaderSchedule` for the current epoch.
pub fn use_get_leader_schedule() -> Resource<Option<RpcLeaderSchedule>> {
    use_fetch(|c| async move { c.get_leader_schedule(None, None).await.ok().flatten() })
}

// --- Address argument ---

/// `getBalance`.
pub fn use_get_balance(pubkey: Pubkey) -> Resource<Option<u64>> {
    use_fetch(move |c| async move {
        c.get_balance(pubkey.to_string(), None)
            .await
            .ok()
            .map(|r| r.value)
    })
}
/// `getAccountInfo`.
pub fn use_get_account(pubkey: Pubkey) -> Resource<Option<UiAccount>> {
    use_fetch(move |c| async move {
        c.get_account_info(pubkey.to_string(), None)
            .await
            .ok()
            .and_then(|r| r.value)
    })
}
/// `getTokenAccountBalance`.
pub fn use_get_token_account_balance(account: Pubkey) -> Resource<Option<UiTokenAmount>> {
    use_fetch(move |c| async move {
        c.get_token_account_balance(account.to_string(), None)
            .await
            .ok()
            .map(|r| r.value)
    })
}
/// `getTokenLargestAccounts`.
pub fn use_get_token_largest_accounts(
    mint: Pubkey,
) -> Resource<Option<Vec<RpcTokenAccountBalance>>> {
    use_fetch(move |c| async move {
        c.get_token_largest_accounts(mint.to_string(), None)
            .await
            .ok()
            .map(|r| r.value)
    })
}
/// `getTokenSupply`.
pub fn use_get_token_supply(mint: Pubkey) -> Resource<Option<UiTokenAmount>> {
    use_fetch(move |c| async move {
        c.get_token_supply(mint.to_string(), None)
            .await
            .ok()
            .map(|r| r.value)
    })
}
/// `getProgramAccounts`.
pub fn use_get_program_accounts(program_id: Pubkey) -> Resource<Option<Vec<RpcKeyedAccount>>> {
    use_fetch(move |c| async move {
        c.get_program_accounts(program_id.to_string(), None)
            .await
            .ok()
            .map(|oc| oc.parse_value())
    })
}
/// `getSignaturesForAddress`.
pub fn use_get_signatures_for_address(
    address: Pubkey,
) -> Resource<Option<Vec<RpcConfirmedTransactionStatusWithSignature>>> {
    use_fetch(move |c| async move {
        c.get_signatures_for_address(address.to_string(), None)
            .await
            .ok()
    })
}

// --- Slot / numeric argument ---

/// `getBlockTime` (unix seconds).
pub fn use_get_block_time(slot: u64) -> Resource<Option<i64>> {
    use_fetch(move |c| async move { c.get_block_time(slot).await.ok().flatten() })
}
/// `getBlock`.
pub fn use_get_block(slot: u64) -> Resource<Option<UiConfirmedBlock>> {
    use_fetch(move |c| async move { c.get_block(slot, None).await.ok() })
}
/// `getBlockCommitment`.
pub fn use_get_block_commitment(slot: u64) -> Resource<Option<RpcBlockCommitment<Vec<usize>>>> {
    use_fetch(move |c| async move { c.get_block_commitment(slot).await.ok() })
}
/// `getMinimumBalanceForRentExemption`.
pub fn use_get_minimum_balance_for_rent_exemption(data_size: u64) -> Resource<Option<u64>> {
    use_fetch(move |c| async move {
        c.get_minimum_balance_for_rent_exemption(data_size, None)
            .await
            .ok()
    })
}
/// `getSlotLeaders`.
pub fn use_get_slot_leaders(start_slot: u64, limit: u64) -> Resource<Option<Vec<String>>> {
    use_fetch(move |c| async move { c.get_slot_leaders(start_slot, limit).await.ok() })
}
/// `getBlocks` in `[start_slot, end_slot]` (end defaults to latest).
pub fn use_get_blocks(start_slot: u64, end_slot: Option<u64>) -> Resource<Option<Vec<u64>>> {
    use_fetch(move |c| async move { c.get_blocks(start_slot, end_slot, None).await.ok() })
}
/// `getBlocksWithLimit`.
pub fn use_get_blocks_with_limit(start_slot: u64, limit: u64) -> Resource<Option<Vec<u64>>> {
    use_fetch(move |c| async move { c.get_blocks_with_limit(start_slot, limit, None).await.ok() })
}
/// `getRecentPerformanceSamples`.
pub fn use_get_recent_performance_samples(
    limit: Option<u32>,
) -> Resource<Option<Vec<RpcPerfSample>>> {
    use_fetch(move |c| async move { c.get_recent_performance_samples(limit).await.ok() })
}

// --- String argument ---

/// `getTransaction`.
pub fn use_get_transaction(
    signature: String,
) -> Resource<Option<EncodedConfirmedTransactionWithStatusMeta>> {
    use_fetch(move |c| {
        let signature = signature.clone();
        async move { c.get_transaction(signature, None).await.ok().flatten() }
    })
}
/// `getSignatureStatuses`.
pub fn use_get_signature_statuses(
    signatures: Vec<String>,
) -> Resource<Option<Vec<Option<TransactionStatus>>>> {
    use_fetch(move |c| {
        let signatures = signatures.clone();
        async move {
            c.get_signature_statuses(signatures, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}
/// `isBlockhashValid`.
pub fn use_is_blockhash_valid(blockhash: String) -> Resource<Option<bool>> {
    use_fetch(move |c| {
        let blockhash = blockhash.clone();
        async move {
            c.is_blockhash_valid(blockhash, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}
/// `getFeeForMessage` (base64 message).
pub fn use_get_fee_for_message(message: String) -> Resource<Option<u64>> {
    use_fetch(move |c| {
        let message = message.clone();
        async move {
            c.get_fee_for_message(message, None)
                .await
                .ok()
                .and_then(|r| r.value)
        }
    })
}
/// `simulateTransaction` (base64 transaction). Read-only — no state change.
pub fn use_simulate_transaction(
    transaction: String,
) -> Resource<Option<RpcSimulateTransactionResult>> {
    use_fetch(move |c| {
        let transaction = transaction.clone();
        async move {
            c.simulate_transaction(transaction, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}

// --- Address-list / filter argument ---

/// `getMultipleAccounts`.
pub fn use_get_multiple_accounts(pubkeys: Vec<Pubkey>) -> Resource<Option<Vec<Option<UiAccount>>>> {
    use_fetch(move |c| {
        let addrs: Vec<String> = pubkeys.iter().map(|p| p.to_string()).collect();
        async move {
            c.get_multiple_accounts(&addrs, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}
/// `getInflationReward` for the given addresses.
pub fn use_get_inflation_reward(
    pubkeys: Vec<Pubkey>,
) -> Resource<Option<Vec<Option<RpcInflationReward>>>> {
    use_fetch(move |c| {
        let pubkeys = pubkeys.clone();
        async move { c.get_inflation_reward(&pubkeys, None).await.ok() }
    })
}
/// `getRecentPrioritizationFees`, optionally filtered to accounts.
pub fn use_get_recent_prioritization_fees(
    addresses: Option<Vec<Pubkey>>,
) -> Resource<Option<Vec<RpcPrioritizationFee>>> {
    use_fetch(move |c| {
        let addresses = addresses.clone();
        async move {
            c.get_recent_prioritization_fees(addresses.as_deref())
                .await
                .ok()
        }
    })
}
/// `getTokenAccountsByOwner`.
pub fn use_get_token_accounts_by_owner(
    owner: Pubkey,
    filter: RpcTokenAccountsFilter,
) -> Resource<Option<Vec<RpcKeyedAccount>>> {
    use_fetch(move |c| {
        let filter = filter.clone();
        async move {
            c.get_token_accounts_by_owner(owner.to_string(), filter, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}
/// `getTokenAccountsByDelegate`.
pub fn use_get_token_accounts_by_delegate(
    delegate: Pubkey,
    filter: RpcTokenAccountsFilter,
) -> Resource<Option<Vec<RpcKeyedAccount>>> {
    use_fetch(move |c| {
        let filter = filter.clone();
        async move {
            c.get_token_accounts_by_delegate(delegate.to_string(), filter, None)
                .await
                .ok()
                .map(|r| r.value)
        }
    })
}
