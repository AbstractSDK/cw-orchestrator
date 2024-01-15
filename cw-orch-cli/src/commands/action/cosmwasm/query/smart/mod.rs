use cosmrs::proto::cosmwasm::wasm::v1::{
    query_client::QueryClient, QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use cw_orch::{
    daemon::{ChainRegistryData, GrpcChannel},
    tokio::runtime::Runtime,
};

use crate::{commands::action::CosmosContext, types::CliAddress};

use super::super::msg_type;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryWasmOutput)]
pub struct QuerySmartCommands {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    /// How do you want to pass the message arguments?
    msg_type: msg_type::MsgType,
    /// Enter message
    msg: String,
}

impl QuerySmartCommands {
    fn input_msg_type(
        _context: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<msg_type::MsgType>> {
        msg_type::input_msg_type()
    }
}
pub struct QueryWasmOutput;

impl QueryWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<QuerySmartCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let contract_account_id = scope.contract.clone().account_id(chain.chain_info())?;

        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;

        let chain_data: ChainRegistryData = chain.into();

        let rt = Runtime::new()?;
        let resp = rt.block_on(async {
            let grpc_channel =
                GrpcChannel::connect(&chain_data.apis.grpc, &chain_data.chain_id).await?;
            let mut client = QueryClient::new(grpc_channel);

            let resp = client
                .smart_contract_state(QuerySmartContractStateRequest {
                    address: contract_account_id.to_string(),
                    query_data: msg,
                })
                .await?;
            color_eyre::Result::<QuerySmartContractStateResponse, color_eyre::Report>::Ok(
                resp.into_inner(),
            )
        })?;
        let parsed_output: serde_json::Value = serde_json::from_slice(&resp.data)?;
        println!("{}", serde_json::to_string_pretty(&parsed_output)?);

        Ok(QueryWasmOutput)
    }
}
