use crate::{
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress},
};

use super::CosmosContext;

use cosmwasm_std::Uint128;
use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = SendCw20Output)]
pub struct Cw20TransferCommands {
    /// Cw20 Address or alias from address-book
    cw20_address: CliAddress,
    /// Cw20 Amount
    amount: u128,
    /// Recipient address or alias from address-book
    to_address: CliAddress,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl Cw20TransferCommands {
    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct SendCw20Output;

impl SendCw20Output {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<Cw20TransferCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let to_address_account_id = scope
            .to_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let cw20_account_id = scope
            .cw20_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let cw20_addr = Addr::unchecked(cw20_account_id);

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let resp = daemon.execute(
            &cw20::Cw20ExecuteMsg::Transfer {
                recipient: to_address_account_id.to_string(),
                amount: Uint128::new(scope.amount),
            },
            &[],
            &cw20_addr,
        )?;
        resp.log(chain.chain_info());

        Ok(SendCw20Output)
    }
}
