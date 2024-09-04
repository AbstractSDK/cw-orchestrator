use crate::{commands::action::CosmosContext, types::CliAddress};

use cw_orch::prelude::*;

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
        let addr = Addr::unchecked(account_id);

        let daemon = chain.daemon_querier()?;

        let contract_info = daemon.wasm_querier().contract_info(&addr)?;
        println!("{}", serde_json::to_string_pretty(&contract_info)?);

        Ok(QueryContractInfoOutput)
    }
}
