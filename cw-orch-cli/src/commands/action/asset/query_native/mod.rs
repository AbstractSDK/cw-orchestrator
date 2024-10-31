use crate::types::{CliAddress, CliSkippable};

use super::CosmosContext;

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryNativeOutput)]
pub struct QueryNativeCommands {
    /// Input denom or leave empty to query all balances
    denom: CliSkippable<String>,
    /// Address or alias from address-book
    address: CliAddress,
}

pub struct QueryNativeOutput;

impl QueryNativeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<QueryNativeCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let denom = scope.denom.0.clone();

        let account_id = scope
            .address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let addr = Addr::unchecked(account_id);

        let daemon = chain.daemon_querier()?;

        if let Some(denom) = denom {
            let balance = daemon.balance(&addr, Some(denom))?.swap_remove(0);
            println!("balance: {balance}")
        } else {
            let balances = daemon.balance(&addr, None)?;
            // `cosmwasm_std::Coins` have nice display
            let coins = cosmwasm_std::Coins::try_from(balances).unwrap();
            println!("balances: {coins}")
        }

        Ok(QueryNativeOutput)
    }
}
