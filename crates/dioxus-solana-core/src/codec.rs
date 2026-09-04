use crate::WalletError;
use solana_transaction::versioned::VersionedTransaction;

/// Serialize a versioned transaction to bincode bytes (wire format wallets expect).
pub fn serialize_transaction(tx: &VersionedTransaction) -> Result<Vec<u8>, WalletError> {
    bincode::serialize(tx).map_err(|e| WalletError::Codec(format!("serialize tx: {e}")))
}

/// Deserialize bincode bytes back into a versioned transaction.
pub fn deserialize_transaction(bytes: &[u8]) -> Result<VersionedTransaction, WalletError> {
    bincode::deserialize(bytes).map_err(|e| WalletError::Codec(format!("deserialize tx: {e}")))
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
}
