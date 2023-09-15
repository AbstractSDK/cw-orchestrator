use color_eyre::eyre::Context;
use cw_orch::{
    prelude::{networks::parse_network, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::{
    commands::transaction::CosmosContext,
    types::{CliCoins, CliSkippable},
};

use super::super::msg_type;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = InstantiateWasmOutput)]
/// Execute contract method
pub struct InstantiateContractCommands {
    /// Contract code id
    code_id: u64,
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    /// How do you want to pass the message arguments?
    msg_type: msg_type::MsgType,
    /// Enter message
    msg: String,
    #[interactive_clap(skip_default_input_arg)]
    /// Admin address of the contract
    admin: CliSkippable<String>,
    #[interactive_clap(skip_default_input_arg)]
    /// Label for the contract
    label: CliSkippable<String>,
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Signer id
    // TODO: should be possible to sign it from the seed phrase
    signer: String,
}

impl InstantiateContractCommands {
    fn input_msg_type(
        _context: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<msg_type::MsgType>> {
        msg_type::input_msg_type()
    }

    fn input_admin(
        _context: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<CliSkippable<String>>> {
        let val: Option<String> =
            inquire::CustomType::new("Input admin address for the contract".to_string().as_str())
                .with_help_message("press Esc to skip admin")
                .prompt_skippable()?;
        Ok(Some(CliSkippable(val)))
    }

    fn input_label(
        _context: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<CliSkippable<String>>> {
        let val: Option<String> =
            inquire::CustomType::new("Input label for the contract".to_string().as_str())
                .with_help_message("press Esc to skip label")
                .prompt_skippable()?;
        Ok(Some(CliSkippable(val)))
    }

    fn input_coins(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }
}
pub struct InstantiateWasmOutput;

impl InstantiateWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<InstantiateContractCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
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

            let exec_msg = cosmrs::cosmwasm::MsgInstantiateContract {
                sender: daemon.sender.pub_addr()?,
                admin: scope.admin.clone().0.map(|a| a.parse()).transpose()?,
                code_id: scope.code_id,
                label: scope.label.0.clone(),
                msg,
                funds: coins,
            };
            let _res = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(InstantiateWasmOutput)
    }
}
