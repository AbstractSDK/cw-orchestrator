use crate::{commands::action::CosmosContext, types::CliAddress};

use super::ContractQueryMsg;

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = GetOwnershipOutput)]
pub struct GetOwnership {
    /// Contract Address or alias from address-book
    contract: CliAddress,
}

pub struct GetOwnershipOutput;

impl GetOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<GetOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let contract_addr = Addr::unchecked(contract_account_id);

        let daemon = chain.daemon_querier()?;

        let output: serde_json::Value = daemon
            .wasm_querier()
            .smart_query(&contract_addr, &ContractQueryMsg::Ownership {})?;
        println!("{}", serde_json::to_string_pretty(&output)?);

        Ok(GetOwnershipOutput)
    }
}
