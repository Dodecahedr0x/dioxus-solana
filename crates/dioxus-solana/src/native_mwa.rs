//! Host-testable Mobile Wallet Adapter helpers shared with the Android JNI
//! wallet. Discovery still returns nothing on desktop and iOS.

use base64::Engine;
use dioxus_solana_core::{WalletError, WalletInfo};
use solana_pubkey::Pubkey;
use solana_signature::Signature;

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

/// Picker name for the one native Android MWA entry.
pub const MOBILE_WALLET_NAME: &str = "Mobile wallet";

/// MWA `RpcCluster` name for a Wallet Standard chain id.
pub fn rpc_cluster_from_chain_id(chain_id: &str) -> &'static str {
    match chain_id {
        "solana:mainnet" | "mainnet-beta" | "mainnet" => "mainnet-beta",
        "solana:testnet" | "testnet" => "testnet",
        _ => "devnet",
    }
}

pub fn wallet_info() -> WalletInfo {
    WalletInfo {
        name: MOBILE_WALLET_NAME.into(),
        icon: None,
        features: vec![
            "standard:connect".into(),
            "solana:signMessage".into(),
            "solana:signTransaction".into(),
        ],
    }
}

pub fn pubkey_from_b64(b64: &str) -> Result<Pubkey, WalletError> {
    Ok(Pubkey::try_from(decode_b64(b64)?.as_slice())?)
}

pub fn signature_from_b64(b64: &str) -> Result<Signature, WalletError> {
    Ok(Signature::try_from(decode_b64(b64)?.as_slice())?)
}

pub fn decode_b64(b64: &str) -> Result<Vec<u8>, WalletError> {
    B64.decode(b64.trim())
        .map_err(|e| WalletError::Codec(e.to_string()))
}

pub fn encode_b64(bytes: &[u8]) -> String {
    B64.encode(bytes)
}

pub fn error_from_status(status: &str, message: &str) -> WalletError {
    match status {
        "ok" => WalletError::Js(message.to_string()),
        "not_installed" => WalletError::NotInstalled,
        "rejected" => WalletError::UserRejected,
        "disconnected" => WalletError::Disconnected,
        _ => {
            if message.is_empty() {
                WalletError::Js(status.to_string())
            } else {
                WalletError::from_js(None, message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_solana_core::WalletError;

    #[test]
    fn picker_name_is_mobile_wallet() {
        assert_eq!(wallet_info().name, "Mobile wallet");
        assert!(wallet_info()
            .features
            .iter()
            .any(|f| f == "solana:signTransaction"));
    }

    #[test]
    fn rpc_cluster_maps_wallet_standard_chain() {
        assert_eq!(rpc_cluster_from_chain_id("solana:mainnet"), "mainnet-beta");
        assert_eq!(rpc_cluster_from_chain_id("solana:testnet"), "testnet");
        assert_eq!(rpc_cluster_from_chain_id("solana:devnet"), "devnet");
        assert_eq!(rpc_cluster_from_chain_id("solana:localnet"), "devnet");
    }

    #[test]
    fn pubkey_from_standard_b64() {
        let pk = pubkey_from_b64("BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc=").unwrap();
        assert_eq!(pk.to_bytes(), [7u8; 32]);
    }

    #[test]
    fn signature_from_standard_b64() {
        let sig = signature_from_b64(
            "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQ==",
        )
        .unwrap();
        assert_eq!(sig.as_ref(), &[1u8; 64]);
    }

    #[test]
    fn status_maps_wallet_errors() {
        assert_eq!(
            error_from_status("not_installed", ""),
            WalletError::NotInstalled
        );
        assert_eq!(error_from_status("rejected", ""), WalletError::UserRejected);
        assert_eq!(
            error_from_status("disconnected", ""),
            WalletError::Disconnected
        );
        assert_eq!(
            error_from_status("err", "Found no installed wallet that supports MWA"),
            WalletError::NotInstalled
        );
    }

    #[test]
    fn b64_round_trips_bytes() {
        let raw = [9u8, 8, 7, 6, 5];
        assert_eq!(decode_b64(&encode_b64(&raw)).unwrap(), raw);
    }

    #[test]
    fn bad_b64_is_codec_error() {
        assert!(matches!(pubkey_from_b64("%%%"), Err(WalletError::Codec(_))));
    }
}
