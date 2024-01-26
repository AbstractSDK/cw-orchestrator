use cosmrs::proto::cosmwasm::wasm::v1::{
    query_client::QueryClient, QueryRawContractStateRequest, QueryRawContractStateResponse,
};
use cw_orch::{
    daemon::{ChainRegistryData, GrpcChannel},
    tokio::runtime::Runtime,
};

use crate::{commands::action::CosmosContext, types::CliAddress};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryWasmOutput)]
pub struct QueryRawCommands {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    // TODO: add base-64 option for binary keys
    /// Enter key
    key: String,
}

pub struct QueryWasmOutput;

impl QueryWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<QueryRawCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let contract_account_id = scope.contract.clone().account_id(chain.chain_info())?;

        let chain_data: ChainRegistryData = chain.into();

        let rt = Runtime::new()?;
        let resp = rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.apis.grpc, chain_data.chain_id.as_str()).await?;
            let mut client = QueryClient::new(grpc_channel);

            let resp = client
                .raw_contract_state(QueryRawContractStateRequest {
                    address: contract_account_id.to_string(),
                    query_data: scope.key.clone().into_bytes(),
                })
                .await?;

            color_eyre::Result::<QueryRawContractStateResponse, color_eyre::Report>::Ok(
                resp.into_inner(),
            )
        })?;

        let parsed_output: serde_json::Value = serde_json::from_slice(&resp.data)?;
        println!("{}", serde_json::to_string_pretty(&parsed_output)?);

        Ok(QueryWasmOutput)
    }
}
