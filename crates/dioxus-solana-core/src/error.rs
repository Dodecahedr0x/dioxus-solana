use thiserror::Error;

/// Errors surfaced by wallet operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum WalletError {
    #[error("wallet not installed")]
    NotInstalled,
    #[error("user rejected the request")]
    UserRejected,
    #[error("wallet is disconnected")]
    Disconnected,
    #[error("wallet is missing required feature: {0}")]
    Feature(String),
    #[error("javascript error: {0}")]
    Js(String),
    #[error("codec error: {0}")]
    Codec(String),
}

impl WalletError {
    /// Map a JS error `code` (e.g. `4001`) and message onto a typed error.
    pub fn from_js(code: Option<f64>, message: &str) -> Self {
        if code == Some(4001.0) {
            return WalletError::UserRejected;
        }
        let lower = message.to_lowercase();
        const REJECTION_PHRASES: &[&str] = &[
            "user rejected",
            "rejected by user",
            "rejected the request",
            "user declined",
            "user denied",
        ];
        if REJECTION_PHRASES.iter().any(|p| lower.contains(p)) {
            return WalletError::UserRejected;
        }
        const NOT_INSTALLED_PHRASES: &[&str] = &["no installed wallet", "wallet not found"];
        if NOT_INSTALLED_PHRASES.iter().any(|p| lower.contains(p)) {
            return WalletError::NotInstalled;
        }
        WalletError::Js(message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_user_rejection_code() {
        assert_eq!(
            WalletError::from_js(Some(4001.0), "User rejected"),
            WalletError::UserRejected
        );
    }

    #[test]
    fn maps_rejected_by_message() {
        assert_eq!(
            WalletError::from_js(None, "Request was rejected by user"),
            WalletError::UserRejected
        );
    }

    #[test]
    fn falls_back_to_js_variant() {
        assert_eq!(
            WalletError::from_js(None, "boom"),
            WalletError::Js("boom".into())
        );
    }

    #[test]
    fn does_not_treat_unrelated_rejection_mentions_as_user_rejection() {
        let message = "transaction rejected by validator: blockhash not found";
        assert_eq!(
            WalletError::from_js(None, message),
            WalletError::Js(message.into())
        );
    }

    #[test]
    fn maps_mwa_wallet_not_found() {
        assert_eq!(
            WalletError::from_js(None, "Found no installed wallet that supports MWA"),
            WalletError::NotInstalled
        );
    }
}
