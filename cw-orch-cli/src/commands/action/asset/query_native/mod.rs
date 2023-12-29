use cosmrs::proto::cosmos::{
    bank::v1beta1::{
        query_client::QueryClient, QueryAllBalancesRequest, QueryAllBalancesResponse,
        QueryBalanceRequest, QueryBalanceResponse,
    },
    base::v1beta1::Coin,
};
use cw_orch::{
    daemon::{ChainRegistryData, GrpcChannel},
    tokio::runtime::Runtime,
};

use crate::types::CliSkippable;

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryNativeOutput)]
pub struct QueryNativeCommands {
    /// Input denom or leave empty to query all balances
    denom: CliSkippable<String>,
    /// Address
    // TODO: Make it Address-bookable
    address: String,
}

pub struct QueryNativeOutput;

impl QueryNativeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<QueryNativeCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let denom = scope.denom.0.clone();

        let chain_data: ChainRegistryData = chain.into();

        let rt = Runtime::new()?;

        rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.apis.grpc, &chain_data.chain_id).await?;
            let mut client = QueryClient::new(grpc_channel);
            if let Some(denom) = denom {
                let response: QueryBalanceResponse = client
                    .balance(QueryBalanceRequest {
                        address: scope.address.clone(),
                        denom: denom.clone(),
                    })
                    .await?
                    .into_inner();
                let balance =
                    response
                        .balance
                        .map(proto_coin_to_std)
                        .unwrap_or(cosmwasm_std::Coin {
                            denom,
                            amount: Default::default(),
                        });
                println!("balance: {balance}")
            } else {
                let response: QueryAllBalancesResponse = client
                    .all_balances(QueryAllBalancesRequest {
                        address: scope.address.clone(),
                        pagination: None,
                    })
                    .await?
                    .into_inner();
                let balances: Vec<cosmwasm_std::Coin> = response
                    .balances
                    .into_iter()
                    .map(proto_coin_to_std)
                    .collect();
                // `cosmwasm_std::Coins` have nice display
                let coins = cosmwasm_std::Coins::try_from(balances).unwrap();
                println!("balances: {coins}")
            }

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(QueryNativeOutput)
    }
}

fn proto_coin_to_std(proto: Coin) -> cosmwasm_std::Coin {
    cosmwasm_std::Coin {
        denom: proto.denom,
        amount: proto.amount.parse().unwrap(),
    }
}
