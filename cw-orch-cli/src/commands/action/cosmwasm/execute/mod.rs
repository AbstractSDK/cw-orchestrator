use color_eyre::eyre::Context;
use cw_orch::{daemon::CosmTxResponse, prelude::DaemonAsync, tokio::runtime::Runtime};

use crate::log::LogOutput;
use crate::types::keys::seed_phrase_for_id;
use crate::types::CliAddress;
use crate::{commands::action::CosmosContext, types::CliCoins};

use super::msg_type;

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
    #[interactive_clap(skip_default_input_arg)]
    /// Enter message
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

    fn input_msg(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        msg_type::input_msg()
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
        let contract_account_id = scope.contract_addr.clone().account_id(chain.chain_info())?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let coins = (&scope.coins).try_into()?;
        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;

        let rt = Runtime::new()?;
        let resp = rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract: contract_account_id,
                msg,
                funds: coins,
            };
            let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            color_eyre::Result::<CosmTxResponse, color_eyre::Report>::Ok(resp)
        })?;

        resp.log();

        Ok(ExecuteWasmOutput)
    }
}
