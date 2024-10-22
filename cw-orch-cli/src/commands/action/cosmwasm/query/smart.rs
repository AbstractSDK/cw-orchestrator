use crate::{commands::action::CosmosContext, types::CliAddress};

use super::super::msg_type;

use cw_orch::prelude::*;

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
    /// Enter message or filename
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

        let contract_account_id = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let contract_addr = Addr::unchecked(contract_account_id);

        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;

        let daemon = chain.daemon_querier()?;

        let resp_data = daemon
            .rt_handle
            .block_on(daemon.wasm_querier()._contract_state(&contract_addr, msg))?;
        let parsed_output: Option<serde_json::Value> = serde_json::from_slice(&resp_data)?;
        let output = parsed_output.unwrap_or_default();
        println!("{}", serde_json::to_string_pretty(&output)?);

        Ok(QueryWasmOutput)
    }
}
