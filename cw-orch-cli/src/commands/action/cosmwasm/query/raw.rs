use crate::{
    commands::action::{
        cosmwasm::msg_type::{self, key_bytes, KeyType},
        CosmosContext,
    },
    types::CliAddress,
};

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryWasmOutput)]
pub struct QueryRawCommands {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    /// Enter key type
    #[interactive_clap(skip_default_input_arg)]
    key_type: KeyType,
    /// Enter key
    key: String,
}

impl QueryRawCommands {
    fn input_key_type(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<KeyType>> {
        msg_type::input_key_type()
    }
}

pub struct QueryWasmOutput;

impl QueryWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<QueryRawCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let contract_addr = Addr::unchecked(contract_account_id);

        let query_data = key_bytes(scope.key.clone(), scope.key_type)?;

        let daemon = chain.daemon_querier()?;

        let resp_data = daemon
            .wasm_querier()
            .raw_query(&contract_addr, query_data)?;
        let parsed_output: Option<serde_json::Value> = serde_json::from_slice(&resp_data)?;
        let output = parsed_output.unwrap_or_default();
        println!("{}", serde_json::to_string_pretty(&output)?);

        Ok(QueryWasmOutput)
    }
}
