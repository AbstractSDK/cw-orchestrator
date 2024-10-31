use crate::{
    commands::action::CosmosContext,
    common::parse_expiration,
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress, CliExpiration, CliSkippable},
};

use super::ContractExecuteMsg;

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = TransferOwnershipOutput)]
pub struct TransferOwnership {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    /// New owner Address or alias from address-book
    new_owner: CliAddress,
    /// Expiration
    #[interactive_clap(skip_default_input_arg)]
    expiration: CliExpiration,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
    /// New owner signer id, leave empty to skip auto-claim
    new_signer: CliSkippable<String>,
}

impl TransferOwnership {
    fn input_expiration(_: &CosmosContext) -> color_eyre::eyre::Result<Option<CliExpiration>> {
        let expiration = parse_expiration()?;
        Ok(Some(CliExpiration(expiration)))
    }

    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct TransferOwnershipOutput;

impl TransferOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<TransferOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let contract_addr = Addr::unchecked(contract_account_id);

        let new_owner = scope
            .new_owner
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let action = cw_ownable::Action::TransferOwnership {
            new_owner: new_owner.to_string(),
            expiry: Some(scope.expiration.0),
        };
        let resp = daemon.execute(
            &ContractExecuteMsg::UpdateOwnership(action),
            &[],
            &contract_addr,
        )?;
        resp.log(chain.chain_info());
        println!("Successfully transferred ownership, waiting for approval by {new_owner}",);

        let maybe_receiver_seed = scope
            .new_signer
            .0
            .as_deref()
            .map(seed_phrase_for_id)
            .transpose()?;
        if let Some(receiver_seed) = maybe_receiver_seed {
            let daemon = daemon.rebuild().mnemonic(receiver_seed).build()?;

            let action = cw_ownable::Action::AcceptOwnership {};
            let resp = daemon.execute(
                &ContractExecuteMsg::UpdateOwnership(action),
                &[],
                &contract_addr,
            )?;
            resp.log(chain.chain_info());
            println!("{new_owner} successfully accepted ownership");
        }

        Ok(TransferOwnershipOutput)
    }
}
