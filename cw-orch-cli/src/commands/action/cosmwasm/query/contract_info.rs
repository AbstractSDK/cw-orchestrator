use crate::{commands::action::CosmosContext, types::CliAddress};

use cw_orch::{daemon::DaemonBuilder, environment::ChainInfoOwned, prelude::*};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryContractInfoOutput)]
pub struct QueryContractInfoCommands {
    /// Contract Address or alias from address-book
    contract: CliAddress,
}

pub struct QueryContractInfoOutput;

impl QueryContractInfoOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<QueryContractInfoCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let chain_data: ChainInfoOwned = chain.into();
        let daemon = DaemonBuilder::new(chain_data.clone()).build_sender(())?;
        let contract_info = daemon
            .wasm_querier()
            .contract_info(account_id.to_string())?;
        println!("{}", serde_json::to_string_pretty(&contract_info)?);

        Ok(QueryContractInfoOutput)
    }
}
