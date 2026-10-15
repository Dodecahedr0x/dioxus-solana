//! HTTP JSON-RPC methods, ported from spume's `WasmClient` (Apache-2.0) onto the
//! transport-agnostic [`RpcClient`].

use {
    super::RpcClient,
    serde_json::json,
    solana_epoch_info::EpochInfo,
    solana_epoch_schedule::EpochSchedule,
    solana_pubkey::Pubkey,
    solana_rpc_client_types::{
        config::{
            CommitmentConfig, RpcAccountInfoConfig, RpcBlockConfig, RpcBlockProductionConfig,
            RpcContextConfig, RpcEncodingConfigWrapper, RpcEpochConfig, RpcGetVoteAccountsConfig,
            RpcLargestAccountsConfig, RpcLeaderScheduleConfig, RpcProgramAccountsConfig,
            RpcRequestAirdropConfig, RpcSendTransactionConfig, RpcSignatureStatusConfig,
            RpcSignaturesForAddressConfig, RpcSimulateTransactionConfig, RpcSupplyConfig,
            RpcTokenAccountsFilter, RpcTransactionConfig,
        },
        request::{RpcError, RpcRequest},
        response::{
            OptionalContext, Response, RpcAccountBalance, RpcBlockCommitment, RpcBlockProduction,
            RpcBlockhash, RpcConfirmedTransactionStatusWithSignature, RpcContactInfo, RpcIdentity,
            RpcInflationGovernor, RpcInflationRate, RpcInflationReward, RpcKeyedAccount,
            RpcLeaderSchedule, RpcPerfSample, RpcPrioritizationFee, RpcSimulateTransactionResult,
            RpcSnapshotSlotInfo, RpcSupply, RpcTokenAccountBalance, RpcVersionInfo,
            RpcVoteAccountStatus, UiAccount, UiConfirmedBlock, UiTokenAmount,
        },
    },
    solana_transaction_status_client_types::{
        EncodedConfirmedTransactionWithStatusMeta, TransactionStatus,
    },
    std::borrow::Cow,
};

type RpcResult<T> = Result<T, Box<RpcError>>;

impl RpcClient {
    /// Fetch all information associated with the account of the given address.
    pub async fn get_account_info(
        &self,
        address: impl CheckAddress,
        config: Option<RpcAccountInfoConfig>,
    ) -> RpcResult<Response<Option<UiAccount>>> {
        self.send(
            RpcRequest::GetAccountInfo,
            json!([address.parse()?, config]),
        )
        .await
    }

    /// Fetch the lamport balance of an account.
    pub async fn get_balance(
        &self,
        address: impl CheckAddress,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Response<u64>> {
        self.send(RpcRequest::GetBalance, json!([&address.parse()?, config]))
            .await
    }

    /// Return the 20 largest accounts by lamport balance.
    pub async fn get_largest_accounts(
        &self,
        config: Option<RpcLargestAccountsConfig>,
    ) -> RpcResult<Response<Vec<RpcAccountBalance>>> {
        self.send(RpcRequest::GetLargestAccounts, json!([config]))
            .await
    }

    /// Return the minimum balance required to make an account of `data_size` rent-exempt.
    pub async fn get_minimum_balance_for_rent_exemption(
        &self,
        data_size: u64,
        config: Option<CommitmentConfig>,
    ) -> RpcResult<u64> {
        self.send(
            RpcRequest::GetMinimumBalanceForRentExemption,
            json!([data_size, config]),
        )
        .await
    }

    /// Fetch info for several accounts in a single request.
    pub async fn get_multiple_accounts<T: CheckAddress>(
        &self,
        addresses: &[T],
        config: Option<RpcAccountInfoConfig>,
    ) -> RpcResult<Response<Vec<Option<UiAccount>>>> {
        let addresses: Vec<Cow<'_, str>> = addresses
            .iter()
            .map(CheckAddress::parse)
            .collect::<RpcResult<_>>()?;
        self.send(RpcRequest::GetMultipleAccounts, json!([addresses, config]))
            .await
    }

    /// Fetch all accounts owned by the given program, optionally filtered.
    pub async fn get_program_accounts(
        &self,
        program_id: impl CheckAddress,
        config: Option<RpcProgramAccountsConfig>,
    ) -> RpcResult<OptionalContext<Vec<RpcKeyedAccount>>> {
        self.send(
            RpcRequest::GetProgramAccounts,
            json!([program_id.parse()?, config]),
        )
        .await
    }

    /// Return the SPL token balance held by a token account.
    pub async fn get_token_account_balance(
        &self,
        account: impl CheckAddress,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Response<UiTokenAmount>> {
        self.send(
            RpcRequest::GetTokenAccountBalance,
            json!([account.parse()?, config]),
        )
        .await
    }

    /// Fetch SPL token accounts delegated to the given address.
    pub async fn get_token_accounts_by_delegate(
        &self,
        delegate: impl CheckAddress,
        filter: RpcTokenAccountsFilter,
        config: Option<RpcAccountInfoConfig>,
    ) -> RpcResult<Response<Vec<RpcKeyedAccount>>> {
        self.send(
            RpcRequest::GetTokenAccountsByDelegate,
            json!([delegate.parse()?, filter, config]),
        )
        .await
    }

    /// Fetch SPL token accounts owned by the given address.
    pub async fn get_token_accounts_by_owner(
        &self,
        owner: impl CheckAddress,
        filter: RpcTokenAccountsFilter,
        config: Option<RpcAccountInfoConfig>,
    ) -> RpcResult<Response<Vec<RpcKeyedAccount>>> {
        self.send(
            RpcRequest::GetTokenAccountsByOwner,
            json!([owner.parse()?, filter, config]),
        )
        .await
    }

    /// Return the 20 largest token accounts for a given mint.
    pub async fn get_token_largest_accounts(
        &self,
        mint: impl CheckAddress,
        config: Option<CommitmentConfig>,
    ) -> RpcResult<Response<Vec<RpcTokenAccountBalance>>> {
        self.send(
            RpcRequest::GetTokenLargestAccounts,
            json!([mint.parse()?, config]),
        )
        .await
    }

    /// Return the total supply of an SPL token mint.
    pub async fn get_token_supply(
        &self,
        mint: impl CheckAddress,
        config: Option<CommitmentConfig>,
    ) -> RpcResult<Response<UiTokenAmount>> {
        self.send(RpcRequest::GetTokenSupply, json!([mint.parse()?, config]))
            .await
    }

    /// Return the fee the cluster would charge to process a base64-encoded transaction message.
    pub async fn get_fee_for_message(
        &self,
        message: String,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Response<Option<u64>>> {
        self.send(RpcRequest::GetFeeForMessage, json!([message, config]))
            .await
    }

    /// Fetch the latest blockhash and the last block height at which it is still valid.
    pub async fn get_latest_blockhash(
        &self,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Response<RpcBlockhash>> {
        self.send(RpcRequest::GetLatestBlockhash, json!([config]))
            .await
    }

    /// Return recent per-slot prioritization fees, optionally restricted to accounts that must be
    /// writable.
    pub async fn get_recent_prioritization_fees(
        &self,
        addresses: Option<&[Pubkey]>,
    ) -> RpcResult<Vec<RpcPrioritizationFee>> {
        // Omit the positional entirely when `None` — the RPC rejects `[null]`.
        let params = match addresses {
            Some(addresses) => {
                let addresses: Vec<String> = addresses.iter().map(Pubkey::to_string).collect();
                json!([addresses])
            }
            None => json!([]),
        };
        self.send(RpcRequest::GetRecentPrioritizationFees, params)
            .await
    }

    /// Return confirmed transaction signatures involving the given address, most recent first.
    pub async fn get_signatures_for_address(
        &self,
        address: impl CheckAddress,
        config: Option<RpcSignaturesForAddressConfig>,
    ) -> RpcResult<Vec<RpcConfirmedTransactionStatusWithSignature>> {
        self.send(
            RpcRequest::GetSignaturesForAddress,
            json!([address.parse()?, config]),
        )
        .await
    }

    /// Return the processing status of one or more transaction signatures.
    pub async fn get_signature_statuses(
        &self,
        signatures: Vec<String>,
        config: Option<RpcSignatureStatusConfig>,
    ) -> RpcResult<Response<Vec<Option<TransactionStatus>>>> {
        self.send(
            RpcRequest::GetSignatureStatuses,
            json!([signatures, config]),
        )
        .await
    }

    /// Fetch a confirmed transaction by its signature.
    pub async fn get_transaction(
        &self,
        signature: String,
        config: Option<RpcEncodingConfigWrapper<RpcTransactionConfig>>,
    ) -> RpcResult<Option<EncodedConfirmedTransactionWithStatusMeta>> {
        self.send(RpcRequest::GetTransaction, json!([signature, config]))
            .await
    }

    /// Return the cumulative number of transactions processed by the cluster.
    pub async fn get_transaction_count(&self, config: Option<RpcContextConfig>) -> RpcResult<u64> {
        self.send(RpcRequest::GetTransactionCount, json!([config]))
            .await
    }

    /// Report whether the given blockhash is still valid.
    pub async fn is_blockhash_valid(
        &self,
        blockhash: String,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Response<bool>> {
        self.send(RpcRequest::IsBlockhashValid, json!([blockhash, config]))
            .await
    }

    /// Request an airdrop of lamports to an address (devnet/testnet only). Returns the signature.
    pub async fn request_airdrop(
        &self,
        address: impl CheckAddress,
        lamports: u64,
        config: Option<RpcRequestAirdropConfig>,
    ) -> RpcResult<String> {
        self.send(
            RpcRequest::RequestAirdrop,
            json!([address.parse()?, lamports, config]),
        )
        .await
    }

    /// Submit a signed transaction. Does not wait for confirmation; returns the signature.
    pub async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        self.send(RpcRequest::SendTransaction, json!([transaction, config]))
            .await
    }

    /// Simulate a transaction without submitting it. Returns logs, accounts, and any error.
    pub async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<Response<RpcSimulateTransactionResult>> {
        self.send(
            RpcRequest::SimulateTransaction,
            json!([transaction, config]),
        )
        .await
    }

    /// Fetch a confirmed block by slot.
    pub async fn get_block(
        &self,
        slot: u64,
        config: Option<RpcEncodingConfigWrapper<RpcBlockConfig>>,
    ) -> RpcResult<UiConfirmedBlock> {
        self.send(RpcRequest::GetBlock, json!([slot, config])).await
    }

    /// Return per-stake vote commitment for a block at the given slot.
    pub async fn get_block_commitment(
        &self,
        slot: u64,
    ) -> RpcResult<RpcBlockCommitment<Vec<usize>>> {
        self.send(
            RpcRequest::Custom {
                method: "getBlockCommitment",
            },
            json!([slot]),
        )
        .await
    }

    /// Return the current block height of the node.
    pub async fn get_block_height(&self, config: Option<RpcContextConfig>) -> RpcResult<u64> {
        self.send(RpcRequest::GetBlockHeight, json!([config])).await
    }

    /// Return recent block production information, broken down by validator identity.
    pub async fn get_block_production(
        &self,
        config: Option<RpcBlockProductionConfig>,
    ) -> RpcResult<Response<RpcBlockProduction>> {
        self.send(RpcRequest::GetBlockProduction, json!([config]))
            .await
    }

    /// List confirmed blocks in the inclusive slot range `[start_slot, end_slot]`.
    pub async fn get_blocks(
        &self,
        start_slot: u64,
        end_slot: Option<u64>,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Vec<u64>> {
        self.send(RpcRequest::GetBlocks, json!([start_slot, end_slot, config]))
            .await
    }

    /// List up to `limit` confirmed blocks starting at `start_slot`.
    pub async fn get_blocks_with_limit(
        &self,
        start_slot: u64,
        limit: u64,
        config: Option<RpcContextConfig>,
    ) -> RpcResult<Vec<u64>> {
        self.send(
            RpcRequest::GetBlocksWithLimit,
            json!([start_slot, limit, config]),
        )
        .await
    }

    /// Return the estimated UNIX production timestamp of a block, if available.
    pub async fn get_block_time(&self, slot: u64) -> RpcResult<Option<i64>> {
        self.send(RpcRequest::GetBlockTime, json!([slot])).await
    }

    /// Return the slot of the lowest confirmed block still retained by the node.
    pub async fn get_first_available_block(&self) -> RpcResult<u64> {
        self.send(RpcRequest::GetFirstAvailableBlock, json!([]))
            .await
    }

    /// Return recent slot-time performance samples, up to `limit` (default 720).
    pub async fn get_recent_performance_samples(
        &self,
        limit: Option<u32>,
    ) -> RpcResult<Vec<RpcPerfSample>> {
        // Omit the positional entirely when `None` — the RPC rejects `[null]`.
        let params = match limit {
            Some(n) => json!([n]),
            None => json!([]),
        };
        self.send(RpcRequest::GetRecentPerformanceSamples, params)
            .await
    }

    /// Return the lowest slot the node has information about.
    pub async fn minimum_ledger_slot(&self) -> RpcResult<u64> {
        self.send(RpcRequest::MinimumLedgerSlot, json!([])).await
    }

    /// Return information about all known cluster nodes.
    pub async fn get_cluster_nodes(&self) -> RpcResult<Vec<RpcContactInfo>> {
        self.send(RpcRequest::GetClusterNodes, json!([])).await
    }

    /// Return information about the current epoch.
    pub async fn get_epoch_info(&self, config: Option<RpcEpochConfig>) -> RpcResult<EpochInfo> {
        self.send(RpcRequest::GetEpochInfo, json!([config])).await
    }

    /// Return the cluster's epoch schedule parameters from the genesis config.
    pub async fn get_epoch_schedule(&self) -> RpcResult<EpochSchedule> {
        self.send(RpcRequest::GetEpochSchedule, json!([])).await
    }

    /// Return the cluster's genesis hash.
    pub async fn get_genesis_hash(&self) -> RpcResult<String> {
        self.send(RpcRequest::GetGenesisHash, json!([])).await
    }

    /// Return `"ok"` when the node is caught up with its peers.
    pub async fn get_health(&self) -> RpcResult<String> {
        self.send(RpcRequest::GetHealth, json!([])).await
    }

    /// Return the highest full (and optional incremental) snapshot slot the node has stored.
    pub async fn get_highest_snapshot_slot(&self) -> RpcResult<RpcSnapshotSlotInfo> {
        self.send(RpcRequest::GetHighestSnapshotSlot, json!([]))
            .await
    }

    /// Return the node's identity pubkey.
    pub async fn get_identity(&self) -> RpcResult<RpcIdentity> {
        self.send(RpcRequest::GetIdentity, json!([])).await
    }

    /// Return the leader schedule for the epoch containing `slot` (or the current epoch).
    pub async fn get_leader_schedule(
        &self,
        slot: Option<u64>,
        config: Option<RpcLeaderScheduleConfig>,
    ) -> RpcResult<Option<RpcLeaderSchedule>> {
        self.send(RpcRequest::GetLeaderSchedule, json!([slot, config]))
            .await
    }

    /// Return the highest slot seen via the retransmit stage.
    pub async fn get_max_retransmit_slot(&self) -> RpcResult<u64> {
        self.send(RpcRequest::GetMaxRetransmitSlot, json!([])).await
    }

    /// Return the highest slot for which shreds have been inserted.
    pub async fn get_max_shred_insert_slot(&self) -> RpcResult<u64> {
        self.send(RpcRequest::GetMaxShredInsertSlot, json!([]))
            .await
    }

    /// Return the slot the node is currently processing.
    pub async fn get_slot(&self, config: Option<RpcContextConfig>) -> RpcResult<u64> {
        self.send(RpcRequest::GetSlot, json!([config])).await
    }

    /// Return the identity of the current slot leader.
    pub async fn get_slot_leader(&self, config: Option<RpcContextConfig>) -> RpcResult<String> {
        self.send(RpcRequest::GetSlotLeader, json!([config])).await
    }

    /// Return the slot leaders for the half-open range `[start_slot, start_slot + limit)`.
    pub async fn get_slot_leaders(&self, start_slot: u64, limit: u64) -> RpcResult<Vec<String>> {
        self.send(RpcRequest::GetSlotLeaders, json!([start_slot, limit]))
            .await
    }

    /// Return the node's software version.
    pub async fn get_version(&self) -> RpcResult<RpcVersionInfo> {
        self.send(RpcRequest::GetVersion, json!([])).await
    }

    /// Return the current and delinquent vote accounts.
    pub async fn get_vote_accounts(
        &self,
        config: Option<RpcGetVoteAccountsConfig>,
    ) -> RpcResult<RpcVoteAccountStatus> {
        self.send(RpcRequest::GetVoteAccounts, json!([config]))
            .await
    }

    /// Return the cluster's current inflation governor.
    pub async fn get_inflation_governor(
        &self,
        config: Option<CommitmentConfig>,
    ) -> RpcResult<RpcInflationGovernor> {
        self.send(RpcRequest::GetInflationGovernor, json!([config]))
            .await
    }

    /// Return the specific inflation values for the current epoch.
    pub async fn get_inflation_rate(&self) -> RpcResult<RpcInflationRate> {
        self.send(RpcRequest::GetInflationRate, json!([])).await
    }

    /// Return inflation rewards earned by a list of addresses during an epoch.
    pub async fn get_inflation_reward(
        &self,
        addresses: &[Pubkey],
        config: Option<RpcEpochConfig>,
    ) -> RpcResult<Vec<Option<RpcInflationReward>>> {
        let addresses: Vec<String> = addresses.iter().map(Pubkey::to_string).collect();
        self.send(RpcRequest::GetInflationReward, json!([addresses, config]))
            .await
    }

    /// Return the stake-program minimum delegation in lamports.
    pub async fn get_stake_minimum_delegation(
        &self,
        config: Option<CommitmentConfig>,
    ) -> RpcResult<Response<u64>> {
        self.send(RpcRequest::GetStakeMinimumDelegation, json!([config]))
            .await
    }

    /// Return information about the cluster's circulating and non-circulating supply.
    pub async fn get_supply(
        &self,
        config: Option<RpcSupplyConfig>,
    ) -> RpcResult<Response<RpcSupply>> {
        self.send(RpcRequest::GetSupply, json!([config])).await
    }
}

/// An account address parameter: a [`Pubkey`] or its base58 string form
/// (`&str`, `&String`, `String`, `Cow<str>`).
pub trait CheckAddress {
    fn parse(&self) -> RpcResult<Cow<'_, str>>;
}

macro_rules! impl_check_address_str {
    ($($t:ty),*) => {$(
        impl CheckAddress for $t {
            fn parse(&self) -> RpcResult<Cow<'_, str>> {
                let s: &str = self.as_ref();
                Ok(s.into())
            }
        }
    )*};
}

impl_check_address_str!(&str, &String, String, Cow<'_, str>);

impl CheckAddress for Pubkey {
    fn parse(&self) -> RpcResult<Cow<'_, str>> {
        Ok(self.to_string().into())
    }
}

impl CheckAddress for &Pubkey {
    fn parse(&self) -> RpcResult<Cow<'_, str>> {
        Ok(self.to_string().into())
    }
}
