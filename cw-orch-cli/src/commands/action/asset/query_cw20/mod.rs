use crate::types::CliAddress;

use super::CosmosContext;

use cw20::BalanceResponse;
use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryCw20Output)]
pub struct QueryCw20Commands {
    /// Cw20 Address or alias from address-book
    cw20_address: CliAddress,
    /// Address or alias from address-book
    address: CliAddress,
}

pub struct QueryCw20Output;

impl QueryCw20Output {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<QueryCw20Commands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let cw20_account_id = scope
            .cw20_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let cw20_addr = Addr::unchecked(cw20_account_id);

        let account_id = scope
            .address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let daemon = chain.daemon_querier()?;

        let balance: BalanceResponse = daemon.query(
            &(cw20::Cw20QueryMsg::Balance {
                address: account_id.to_string(),
            }),
            &cw20_addr,
        )?;
        println!("{}", serde_json::to_string_pretty(&balance)?);

        Ok(QueryCw20Output)
    }
}
