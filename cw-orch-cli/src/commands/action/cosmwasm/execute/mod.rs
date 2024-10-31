use crate::{
    commands::action::CosmosContext,
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress, CliCoins},
};

use super::msg_type;

use color_eyre::eyre::Context;
use cw_orch::daemon::TxSender;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = ExecuteWasmOutput)]
/// Execute contract method
pub struct ExecuteContractCommands {
    /// Contract Address or alias from address-book
    contract_addr: CliAddress,
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    /// How do you want to pass the message arguments?
    msg_type: msg_type::MsgType,
    /// Enter message or filename
    msg: String,
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl ExecuteContractCommands {
    fn input_msg_type(
        _context: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<msg_type::MsgType>> {
        msg_type::input_msg_type()
    }

    fn input_coins(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }

    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}
pub struct ExecuteWasmOutput;

impl ExecuteWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<ExecuteContractCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;
        let coins = (&scope.coins).try_into()?;

        let contract_account_id = scope
            .contract_addr
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
            sender: daemon.sender().account_id(),
            contract: contract_account_id,
            msg,
            funds: coins,
        };
        let resp = daemon
            .rt_handle
            .block_on(daemon.sender().commit_tx(vec![exec_msg], None))?;
        resp.log(chain.chain_info());

        Ok(ExecuteWasmOutput)
    }
}
