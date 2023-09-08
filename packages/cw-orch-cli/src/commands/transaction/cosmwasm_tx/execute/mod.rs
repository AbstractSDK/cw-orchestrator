use color_eyre::eyre::Context;
use cw_orch::prelude::{networks::parse_network, DaemonBuilder, TxHandler};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

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
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Signer
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

// #[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
// #[strum_discriminants(derive(EnumMessage, EnumIter))]
// #[interactive_clap(input_context = ())]
// #[interactive_clap(output_context = ExecuteWasmOutput)]
// /// Select cosmwasm action
// pub enum ExecuteAction {
//     /// Query
//     #[strum_discriminants(strum(message = "Query cosmwasm message"))]
//     Query,
// }

pub struct ExecuteWasmOutput;

impl ExecuteWasmOutput {
    fn from_previous_context(
        previous_context: CwActionContext,
        scope:&<ExecuteCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        /// TODO: non-panic parse_network
        let chain = parse_network(&previous_context.0);
        let seed = crate::common::seed_phrase_for_id("aloha")?;
        let coins = crate::common::parse_coins()?;

        let d = DaemonBuilder::default().chain(chain).build()?;
        Ok(ExecuteWasmOutput)
    }
}
