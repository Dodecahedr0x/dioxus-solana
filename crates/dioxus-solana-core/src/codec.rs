use crate::WalletError;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

/// Serialize a versioned transaction to bincode bytes (wire format wallets expect).
pub fn serialize_transaction(tx: &VersionedTransaction) -> Result<Vec<u8>, WalletError> {
    bincode::serialize(tx).map_err(|e| WalletError::Codec(format!("serialize tx: {e}")))
}

/// Deserialize bincode bytes back into a versioned transaction.
pub fn deserialize_transaction(bytes: &[u8]) -> Result<VersionedTransaction, WalletError> {
    bincode::deserialize(bytes).map_err(|e| WalletError::Codec(format!("deserialize tx: {e}")))
}

/// Build a `Signature` from exactly 64 bytes.
pub fn signature_from_slice(bytes: &[u8]) -> Result<Signature, WalletError> {
    let arr: [u8; 64] = bytes.try_into().map_err(|_| {
        WalletError::Codec(format!("expected 64-byte signature, got {}", bytes.len()))
    })?;
    Ok(Signature::from(arr))
}

/// Build a `Pubkey` from exactly 32 bytes.
pub fn pubkey_from_slice(bytes: &[u8]) -> Result<Pubkey, WalletError> {
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| WalletError::Codec(format!("expected 32-byte pubkey, got {}", bytes.len())))?;
    Ok(Pubkey::new_from_array(arr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_transaction::versioned::VersionedTransaction;

    #[test]
    fn transaction_round_trips_through_bytes() {
        let tx = VersionedTransaction::default();
        let bytes = serialize_transaction(&tx).unwrap();
        let back = deserialize_transaction(&bytes).unwrap();
        assert_eq!(tx, back);
    }

    #[test]
    fn signature_requires_64_bytes() {
        assert!(signature_from_slice(&[0u8; 64]).is_ok());
        assert!(signature_from_slice(&[0u8; 10]).is_err());
    }

    #[test]
    fn pubkey_from_32_bytes() {
        let pk = pubkey_from_slice(&[7u8; 32]).unwrap();
        assert_eq!(pk.to_bytes(), [7u8; 32]);
        assert!(pubkey_from_slice(&[0u8; 5]).is_err());
    }
}
