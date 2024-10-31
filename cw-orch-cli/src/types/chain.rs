use std::str::FromStr;

use cw_orch::{
    daemon::{
        networks::SUPPORTED_NETWORKS, senders::QueryOnlyDaemon, Daemon, DaemonBuilder, DaemonError,
    },
    environment::{ChainInfo, ChainInfoOwned},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct CliLockedChain(usize);

impl CliLockedChain {
    pub fn new(index: usize) -> Self {
        CliLockedChain(index)
    }

    pub fn chain_info(&self) -> &ChainInfo {
        &SUPPORTED_NETWORKS[self.0]
    }

    pub fn daemon(&self, seed: String) -> Result<Daemon, DaemonError> {
        DaemonBuilder::new(SUPPORTED_NETWORKS[self.0].clone())
            .mnemonic(seed)
            .build()
    }

    pub fn daemon_querier(&self) -> Result<QueryOnlyDaemon, DaemonError> {
        DaemonBuilder::new(SUPPORTED_NETWORKS[self.0].clone()).build_sender(())
    }
}

impl From<CliLockedChain> for ChainInfoOwned {
    fn from(value: CliLockedChain) -> Self {
        SUPPORTED_NETWORKS[value.0].clone().into()
    }
}

impl FromStr for CliLockedChain {
    type Err = String;

    // Just parse chain id
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SUPPORTED_NETWORKS
            .iter()
            .position(|c| c.chain_id == s)
            .map(CliLockedChain::new)
            .ok_or("Unknown network".to_owned())
    }
}

impl ::std::fmt::Display for CliLockedChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SUPPORTED_NETWORKS[self.0].chain_id)
    }
}

impl interactive_clap::ToCli for CliLockedChain {
    type CliVariant = CliLockedChain;
}
