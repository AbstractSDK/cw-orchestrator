use crate::{
    commands::action::CosmosContext,
    log::LogOutput,
    types::{address_book, keys::seed_phrase_for_id, CliCoins, CliSkippable},
};

use super::msg_type;

use color_eyre::eyre::Context;
use cw_orch::{daemon::TxSender, prelude::*};

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
    /// Enter message or filename
    msg: String,
    /// Label for the contract
    label: String,
    /// Admin address of the contract, leave empty to skip admin
    admin: CliSkippable<String>,
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl InstantiateContractCommands {
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
pub struct InstantiateWasmOutput;

impl InstantiateWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<InstantiateContractCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let coins = (&scope.coins).try_into()?;
        let msg = msg_type::msg_bytes(scope.msg.clone(), scope.msg_type.clone())?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let init_msg = cosmrs::cosmwasm::MsgInstantiateContract {
            sender: daemon.sender().account_id(),
            admin: scope.admin.clone().0.map(|a| a.parse()).transpose()?,
            code_id: scope.code_id,
            label: Some(scope.label.clone()),
            msg,
            funds: coins,
        };

        let resp = daemon
            .rt_handle
            .block_on(daemon.sender().commit_tx(vec![init_msg], None))?;
        resp.log(chain.chain_info());

        let address = resp.instantiated_contract_address()?;
        println!("Address of the instantiated contract: {address}");

        // Maybe save it in Address Book
        if inquire::Confirm::new("Would you like to save address in Address Book?").prompt()? {
            let alias = inquire::Text::new("Input new contract alias")
                // Use label as default value
                .with_initial_value(&scope.label)
                .prompt()?;
            address_book::try_insert_account_id(chain.chain_info(), &alias, address.as_str())?;
        };

        Ok(InstantiateWasmOutput)
    }
}
