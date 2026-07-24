use dioxus_solana_core::{ConnectedAccount, WalletError};
use solana_pubkey::Pubkey;

/// Reactive connection state held in a Dioxus `Signal`.
#[derive(Clone, Debug, Default)]
pub enum WalletState {
    #[default]
    Disconnected,
    Connecting,
    Connected(ConnectedAccount),
    Error(WalletError),
}

impl WalletState {
    pub fn is_connected(&self) -> bool {
        matches!(self, WalletState::Connected(_))
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        match self {
            WalletState::Connected(a) => Some(a.pubkey()),
            _ => None,
        }
    }

    pub fn account(&self) -> Option<&ConnectedAccount> {
        match self {
            WalletState::Connected(a) => Some(a),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_helpers() {
        assert!(WalletState::Disconnected.pubkey().is_none());
        assert!(!WalletState::Connecting.is_connected());
    }
}
