use cw_orch::{daemon::GrpcChannel, environment::ChainInfoOwned, tokio::runtime::Runtime};

use crate::types::{CliAddress, CliSkippable};

use super::CosmosContext;

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

        let chain_data: ChainInfoOwned = chain.into();
        let rt = Runtime::new()?;

        rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.grpc_urls, chain_data.chain_id.as_str()).await?;
            let bank = cw_orch::daemon::queriers::Bank::new_async(grpc_channel);
            if let Some(denom) = denom {
                let balance = bank
                    ._balance(account_id.to_string(), Some(denom))
                    .await?
                    .swap_remove(0);
                println!("balance: {balance}")
            } else {
                let balances = bank._balance(account_id.to_string(), None).await?;
                // `cosmwasm_std::Coins` have nice display
                let coins = cosmwasm_std::Coins::try_from(balances).unwrap();
                println!("balances: {coins}")
            }
            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(QueryNativeOutput)
    }
}
