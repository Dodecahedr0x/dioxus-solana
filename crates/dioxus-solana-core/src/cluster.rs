/// A Solana cluster: one of the well-known networks, or [`Cluster::custom`]
/// endpoints (e.g. a private validator or a paid RPC provider).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Cluster {
    #[default]
    MainnetBeta,
    Devnet,
    Testnet,
    Localnet,
    /// Custom RPC/WS endpoints on a chosen Wallet Standard chain.
    Custom(CustomCluster),
}

/// Endpoints for a [`Cluster::Custom`] network. Build one via [`Cluster::custom`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomCluster {
    pub chain_id: String,
    pub rpc_url: String,
    pub ws_url: String,
}

impl Cluster {
    /// Custom RPC/WS endpoints served over `chain`'s Wallet Standard chain.
    ///
    /// `chain` picks the chain identifier the wallet signs against (typically
    /// [`Cluster::MainnetBeta`] or [`Cluster::Devnet`]); the URLs are yours.
    ///
    /// ```
    /// # use dioxus_solana_core::Cluster;
    /// let c = Cluster::custom(Cluster::MainnetBeta, "https://my.rpc", "wss://my.rpc");
    /// assert_eq!(c.chain_id(), "solana:mainnet");
    /// assert_eq!(c.rpc_url(), "https://my.rpc");
    /// ```
    pub fn custom(chain: Cluster, rpc_url: impl Into<String>, ws_url: impl Into<String>) -> Self {
        Cluster::Custom(CustomCluster {
            chain_id: chain.chain_id().to_string(),
            rpc_url: rpc_url.into(),
            ws_url: ws_url.into(),
        })
    }

    /// The Wallet Standard chain identifier, e.g. `"solana:devnet"`.
    pub fn chain_id(&self) -> &str {
        match self {
            Cluster::MainnetBeta => "solana:mainnet",
            Cluster::Devnet => "solana:devnet",
            Cluster::Testnet => "solana:testnet",
            Cluster::Localnet => "solana:localnet",
            Cluster::Custom(c) => &c.chain_id,
        }
    }

    /// JSON-RPC (HTTP) endpoint for this cluster.
    ///
    /// The built-in clusters return rate-limited public endpoints; real apps
    /// should point at their own via [`Cluster::custom`].
    pub fn rpc_url(&self) -> &str {
        match self {
            Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
            Cluster::Devnet => "https://api.devnet.solana.com",
            Cluster::Testnet => "https://api.testnet.solana.com",
            Cluster::Localnet => "http://localhost:8899",
            Cluster::Custom(c) => &c.rpc_url,
        }
    }

    /// PubSub (WebSocket) endpoint for this cluster.
    pub fn ws_url(&self) -> &str {
        match self {
            Cluster::MainnetBeta => "wss://api.mainnet-beta.solana.com",
            Cluster::Devnet => "wss://api.devnet.solana.com",
            Cluster::Testnet => "wss://api.testnet.solana.com",
            Cluster::Localnet => "ws://localhost:8900",
            Cluster::Custom(c) => &c.ws_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_id_maps_each_cluster() {
        assert_eq!(Cluster::MainnetBeta.chain_id(), "solana:mainnet");
        assert_eq!(Cluster::Devnet.chain_id(), "solana:devnet");
        assert_eq!(Cluster::Testnet.chain_id(), "solana:testnet");
        assert_eq!(Cluster::Localnet.chain_id(), "solana:localnet");
    }

    #[test]
    fn endpoints_are_scheme_correct() {
        for c in [
            Cluster::MainnetBeta,
            Cluster::Devnet,
            Cluster::Testnet,
            Cluster::Localnet,
        ] {
            assert!(c.rpc_url().starts_with("http"));
            assert!(c.ws_url().starts_with("ws"));
        }
        assert_eq!(Cluster::Devnet.rpc_url(), "https://api.devnet.solana.com");
        assert_eq!(Cluster::Devnet.ws_url(), "wss://api.devnet.solana.com");
    }

    #[test]
    fn custom_carries_urls_and_inherits_chain() {
        let c = Cluster::custom(Cluster::Devnet, "https://my.rpc", "wss://my.rpc");
        assert_eq!(c.chain_id(), "solana:devnet");
        assert_eq!(c.rpc_url(), "https://my.rpc");
        assert_eq!(c.ws_url(), "wss://my.rpc");
    }
}
