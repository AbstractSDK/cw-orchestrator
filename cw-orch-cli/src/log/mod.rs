use cw_orch::{
    daemon::CosmTxResponse,
    environment::{ChainInfo, ChainKind},
    tokio::runtime::Runtime,
};
use ibc_chain_registry::fetchable::Fetchable;

use crate::fetch::explorers::Explorers;

pub trait LogOutput {
    fn log(&self, chain_info: &ChainInfo);
}

impl LogOutput for CosmTxResponse {
    fn log(&self, chain_info: &ChainInfo) {
        println!("Transaction hash: {}", self.txhash);
        if let ChainKind::Mainnet = chain_info.kind {
            let log_explorer_url = || -> cw_orch::anyhow::Result<()> {
                let rt = Runtime::new()?;
                let Explorers { explorers } = rt.block_on(Explorers::fetch(
                    chain_info.network_info.chain_name.to_owned(),
                    None,
                ))?;
                for explorer in explorers {
                    if let Some(tx_page) = explorer.tx_page {
                        let url = tx_page.replace("${txHash}", &self.txhash);
                        println!("Explorer: {url}");
                        break;
                    }
                }
                Ok(())
            };
            // Ignore any errors
            let _ = log_explorer_url();
        }
    }
}
