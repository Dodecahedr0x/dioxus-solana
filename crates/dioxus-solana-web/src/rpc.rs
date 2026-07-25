//! Wasm Solana RPC surface, re-exported from [`spume`].
//!
//! Exposes spume's HTTP ([`WasmClient`]) and PubSub ([`WasmPubsubClient`])
//! clients plus the `solana_rpc_client_types` result/config types the facade's
//! hooks return, so consumers can name them without a direct spume dependency.
//! The bulk of the types live in the re-exported [`config`] and [`response`]
//! modules.

use solana_account_decoder_client_types::UiAccountEncoding;
use solana_rpc_client_types::config::RpcAccountInfoConfig;

pub use spume::pubsub_provider::Subscription;
pub use spume::{self, WasmClient, WasmPubsubClient};

pub use solana_rpc_client_types::{config, response};

pub use solana_account_decoder_client_types::UiAccountData;
pub use solana_epoch_info::EpochInfo;
pub use solana_epoch_schedule::EpochSchedule;
pub use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, TransactionStatus,
};

/// Account-info config requesting base64-encoded data, so [`UiAccountData::decode`]
/// yields raw bytes for the caller to deserialize.
pub fn base64_account_config() -> RpcAccountInfoConfig {
    RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        ..Default::default()
    }
}
