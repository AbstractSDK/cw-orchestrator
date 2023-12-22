use cw_orch::{
    daemon::{ChainRegistryData, GrpcChannel},
    tokio::runtime::Runtime,
};

use cosmrs::proto::cosmwasm::wasm::v1::{
    query_client::QueryClient, QuerySmartContractStateRequest,
};

use crate::commands::action::CosmosContext;

use super::ContractQueryMsg;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = GetOwnershipOutput)]
pub struct GetOwnership {
    /// Contract address
    contract_addr: String,
}

pub struct GetOwnershipOutput;

impl GetOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<GetOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let msg = serde_json::to_vec(&ContractQueryMsg::Ownership {})?;
        let chain_data: ChainRegistryData = chain.into();

        let rt = Runtime::new()?;
        rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.apis.grpc, &chain_data.chain_id).await?;
            let mut client = QueryClient::new(grpc_channel);

            let resp = client
                .smart_contract_state(QuerySmartContractStateRequest {
                    address: scope.contract_addr.clone(),
                    query_data: msg,
                })
                .await?;
            let parsed_output: serde_json::Value = serde_json::from_slice(&resp.into_inner().data)?;
            println!("{}", serde_json::to_string_pretty(&parsed_output)?);
            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(GetOwnershipOutput)
    }
}
