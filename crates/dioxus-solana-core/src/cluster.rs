/// A Solana cluster, used to build Wallet Standard chain identifier strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cluster {
    #[default]
    MainnetBeta,
    Devnet,
    Testnet,
    Localnet,
}

impl Cluster {
    /// The Wallet Standard chain identifier, e.g. `"solana:devnet"`.
    pub fn chain_id(&self) -> &'static str {
        match self {
            Cluster::MainnetBeta => "solana:mainnet",
            Cluster::Devnet => "solana:devnet",
            Cluster::Testnet => "solana:testnet",
            Cluster::Localnet => "solana:localnet",
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
}
