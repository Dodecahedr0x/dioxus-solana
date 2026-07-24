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
}

impl WalletError {
    /// Map a JS error `code` (e.g. `4001`) and message onto a typed error.
    pub fn from_js(code: Option<f64>, message: &str) -> Self {
        if code == Some(4001.0) || message.to_lowercase().contains("reject") {
            return WalletError::UserRejected;
        }
        WalletError::Js(message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_user_rejection_code() {
        assert_eq!(WalletError::from_js(Some(4001.0), "User rejected"), WalletError::UserRejected);
    }

    #[test]
    fn maps_rejected_by_message() {
        assert_eq!(WalletError::from_js(None, "Request was rejected by user"), WalletError::UserRejected);
    }

    #[test]
    fn falls_back_to_js_variant() {
        assert_eq!(WalletError::from_js(None, "boom"), WalletError::Js("boom".into()));
    }
}
