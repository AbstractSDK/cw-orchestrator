use std::str::FromStr;

use cw_orch::{
    daemon::networks::SUPPORTED_NETWORKS,
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

impl ToString for CliLockedChain {
    fn to_string(&self) -> String {
        SUPPORTED_NETWORKS[self.0].chain_id.to_owned()
    }
}

impl interactive_clap::ToCli for CliLockedChain {
    type CliVariant = CliLockedChain;
}
