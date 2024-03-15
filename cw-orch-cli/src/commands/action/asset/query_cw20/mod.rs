use cw20::BalanceResponse;
use cw_orch::{
    daemon::{ChainRegistryData, GrpcChannel},
    tokio::runtime::Runtime,
};

use cosmrs::proto::cosmwasm::wasm::v1::{
    query_client::QueryClient, QuerySmartContractStateRequest,
};

use crate::types::CliAddress;

use super::CosmosContext;

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
        let account_id = scope
            .address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let chain_data: ChainRegistryData = chain.into();
        let msg = serde_json::to_vec(&cw20::Cw20QueryMsg::Balance {
            address: account_id.to_string(),
        })?;

        let rt = Runtime::new()?;

        rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.apis.grpc, chain_data.chain_id.as_str()).await?;
            let mut client = QueryClient::new(grpc_channel);

            let resp = client
                .smart_contract_state(QuerySmartContractStateRequest {
                    address: cw20_account_id.to_string(),
                    query_data: msg,
                })
                .await?;
            let parsed_output: BalanceResponse = serde_json::from_slice(&resp.into_inner().data)?;
            println!("{}", serde_json::to_string_pretty(&parsed_output)?);

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(QueryCw20Output)
    }
}
