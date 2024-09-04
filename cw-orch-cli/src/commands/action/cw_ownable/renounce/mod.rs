use crate::{
    commands::action::CosmosContext,
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress},
};

use super::ContractExecuteMsg;

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = RenounceOwnershipOutput)]
pub struct RenounceOwnership {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl RenounceOwnership {
    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct RenounceOwnershipOutput;

impl RenounceOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<RenounceOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let contract_addr = Addr::unchecked(contract_account_id);

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let action = cw_ownable::Action::RenounceOwnership {};
        let resp = daemon.execute(
            &ContractExecuteMsg::UpdateOwnership(action),
            &[],
            &contract_addr,
        )?;
        resp.log(chain.chain_info());

        Ok(RenounceOwnershipOutput)
    }
}
