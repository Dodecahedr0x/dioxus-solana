//! Sending transactions: build from instructions, sign with the wallet, submit
//! and confirm through the app's own RPC.
//!
//! Submitting through [`RpcClient`] (instead of the wallet's sign-and-send)
//! guarantees the transaction lands on the same cluster its blockhash came
//! from, whatever network the wallet extension is pointed at.

use std::time::Duration;

use dioxus_solana_core::{serialize_transaction, WalletError};
use solana_hash::Hash;
use solana_instruction::Instruction;
use solana_message::{Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

use crate::rpc::config::{CommitmentConfig, RpcSendTransactionConfig};
use crate::rpc::{RpcClient, RpcError};
use crate::WalletHandle;

/// How long [`RpcClient::confirm_transaction`] waits before giving up — a bit
/// longer than a blockhash stays valid (~150 slots).
const CONFIRM_TIMEOUT: Duration = Duration::from_secs(90);
const CONFIRM_POLL: Duration = Duration::from_millis(500);

/// Why sending (or confirming) a transaction failed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SendError {
    /// The wallet refused or failed to sign.
    #[error(transparent)]
    Wallet(#[from] WalletError),
    /// The RPC node rejected the request (including preflight failures).
    #[error("rpc error: {0}")]
    Rpc(String),
    /// The transaction landed but failed on-chain.
    #[error("transaction failed: {0}")]
    Failed(String),
    /// The transaction was not confirmed in time (it likely expired).
    #[error("transaction was not confirmed in time")]
    Timeout,
}

impl From<Box<RpcError>> for SendError {
    fn from(err: Box<RpcError>) -> Self {
        SendError::Rpc(err.to_string())
    }
}

/// An unsigned transaction paying from `payer` with the given instructions.
/// Signature slots are zeroed for the wallet to fill.
pub fn build_transaction(
    payer: &Pubkey,
    instructions: &[Instruction],
    blockhash: Hash,
) -> VersionedTransaction {
    let message = Message::new_with_blockhash(instructions, Some(payer), &blockhash);
    VersionedTransaction {
        signatures: vec![Signature::default(); message.header.num_required_signatures as usize],
        message: VersionedMessage::Legacy(message),
    }
}

impl RpcClient {
    /// The latest blockhash, parsed.
    pub async fn latest_blockhash(&self) -> Result<Hash, SendError> {
        let resp = self.get_latest_blockhash(None).await?;
        resp.value
            .blockhash
            .parse()
            .map_err(|_| SendError::Rpc(format!("bad blockhash {}", resp.value.blockhash)))
    }

    /// Submit a signed transaction (base64-encoded on the wire).
    pub async fn send_versioned_transaction(
        &self,
        tx: &VersionedTransaction,
    ) -> Result<Signature, SendError> {
        use base64::Engine;
        let bytes = serialize_transaction(tx)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        let config = RpcSendTransactionConfig {
            encoding: Some(solana_transaction_status_client_types::UiTransactionEncoding::Base64),
            ..Default::default()
        };
        let sig = self.send_transaction(encoded, Some(config)).await?;
        sig.parse()
            .map_err(|_| SendError::Rpc(format!("bad signature {sig}")))
    }

    /// Poll until `signature` (a [`Signature`] or its base58 string, as
    /// `request_airdrop` returns) is confirmed. Fails with
    /// [`SendError::Failed`] if it landed with an error, or
    /// [`SendError::Timeout`] if it never landed.
    pub async fn confirm_transaction(&self, signature: &impl ToString) -> Result<(), SendError> {
        let polls = CONFIRM_TIMEOUT.as_millis() / CONFIRM_POLL.as_millis();
        for _ in 0..polls {
            let statuses = self
                .get_signature_statuses(vec![signature.to_string()], None)
                .await?;
            if let Some(status) = statuses.value.into_iter().next().flatten() {
                if let Some(err) = status.err {
                    return Err(SendError::Failed(err.to_string()));
                }
                if status.satisfies_commitment(CommitmentConfig::confirmed()) {
                    return Ok(());
                }
            }
            crate::rpc::sleep(CONFIRM_POLL).await;
        }
        Err(SendError::Timeout)
    }
}

impl WalletHandle {
    /// Sign `tx` with the connected wallet and submit it through `rpc`.
    ///
    /// Wallets that cannot sign without sending (e.g. some mobile wallets) fall
    /// back to the wallet's own sign-and-send. Does not wait for confirmation —
    /// chain [`RpcClient::confirm_transaction`] for that.
    pub async fn send_transaction(
        &self,
        rpc: &RpcClient,
        tx: VersionedTransaction,
    ) -> Result<Signature, SendError> {
        match self.sign_transaction(tx.clone()).await {
            Ok(signed) => rpc.send_versioned_transaction(&signed).await,
            Err(WalletError::Feature(_)) => Ok(self.sign_and_send(tx).await?),
            Err(err) => Err(err.into()),
        }
    }

    /// Build a transaction from `instructions` (paid by the connected account,
    /// with a fresh blockhash), sign it with the wallet and submit it through
    /// `rpc`. Returns once submitted; see [`send_transaction`](Self::send_transaction).
    ///
    /// ```ignore
    /// let sig = wallet.send_instructions(&rpc, &[transfer_ix]).await?;
    /// rpc.confirm_transaction(&sig).await?;
    /// ```
    pub async fn send_instructions(
        &self,
        rpc: &RpcClient,
        instructions: &[Instruction],
    ) -> Result<Signature, SendError> {
        let payer = self.pubkey().ok_or(WalletError::Disconnected)?;
        let blockhash = rpc.latest_blockhash().await?;
        let tx = build_transaction(&payer, instructions, blockhash);
        self.send_transaction(rpc, tx).await
    }
}
