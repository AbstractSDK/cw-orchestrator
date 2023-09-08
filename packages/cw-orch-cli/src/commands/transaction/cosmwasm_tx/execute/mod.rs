use color_eyre::eyre::Context;
use cw_orch::{
    prelude::{networks::parse_network, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::common::CliCoins;

use super::{msg_type, CwActionContext};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CwActionContext)]
#[interactive_clap(output_context = ExecuteWasmOutput)]
pub struct ExecuteCommands {
    contract_address: String,
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    /// How do you want to pass the message arguments?
    msg_type: msg_type::MsgType,
    /// Enter message
    msg: String,
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Signer id
    // TODO: sign it from seed phrase
    signer: String,
}

impl ExecuteCommands {
    fn input_msg_type(
        _context: &CwActionContext,
    ) -> color_eyre::eyre::Result<Option<msg_type::MsgType>> {
        msg_type::input_msg_type()
    }

    fn input_coins(_context: &CwActionContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }
}
pub struct ExecuteWasmOutput;

impl ExecuteWasmOutput {
    fn from_previous_context(
        previous_context: CwActionContext,
        scope:&<ExecuteCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        // TODO: non-panic parse_network
        let chain = parse_network(&previous_context.chain_id);
        let seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let coins = (&scope.coins).try_into()?;
        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;

        let rt = Runtime::new()?;
        rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract: previous_context.contract_addr.parse()?,
                msg,
                funds: coins,
            };
            daemon.sender.commit_tx(vec![exec_msg], None).await?;
            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(ExecuteWasmOutput)
    }
}
