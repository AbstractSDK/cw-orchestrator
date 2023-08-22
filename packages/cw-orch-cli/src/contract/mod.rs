mod error;

use std::{
    fmt::{Display, Write},
    rc::Rc,
};

pub type OrchCliResult<T> = Result<T, OrchCliError>;

use cosmwasm_std::{Addr, Coin, Empty};
use cw_orch::{
    daemon::DaemonState,
    prelude::{
        ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchMigrate, CwOrchQuery,
        CwOrchUpload, Daemon, ExecutableContract, InstantiableContract, MigratableContract,
        QueryableContract,
    },
    state::ChainState,
};

use inquire::{error::InquireResult, ui::RenderConfig, Confirm, CustomType, InquireError, Text};
use serde::{de::DeserializeOwned, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

pub use self::error::OrchCliError;

#[derive(EnumIter, Display)]
enum ActionVariants {
    Execute,
    Query,
    Upload,
    Instantiate,
    Migrate,
    ContractInfo,
    Addons,
    Quit,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct ContractInfo {
    addr: Option<Addr>,
    code_id: Option<u64>,
}

pub trait ContractCli<C>
where
    C: AddonsContext,
    Self: ContractInstance<Daemon>
        + CwOrchUpload<Daemon>
        + InstantiableContract
        + ExecutableContract
        + QueryableContract
        + MigratableContract
        + CwCliAddons<C>,
    <Self as InstantiableContract>::InstantiateMsg: ParseCwMsg,
    <Self as ExecutableContract>::ExecuteMsg: ParseCwMsg,
    <Self as QueryableContract>::QueryMsg: ParseCwMsg,
    <Self as MigratableContract>::MigrateMsg: ParseCwMsg,
{
    fn select_action(mut self, context: C) -> OrchCliResult<()> {
        let state_interface = self.get_chain().state();
        loop {
            let action =
                inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;
            let res = match action {
                ActionVariants::Execute => self.execute_cli(&state_interface),
                ActionVariants::Query => self.query_cli(&state_interface),
                ActionVariants::Upload => {
                    self.upload()?;
                    println!("Code_id: {}", self.code_id().unwrap());
                    Ok(())
                }
                ActionVariants::Instantiate => self.instantiate_cli(&state_interface),
                ActionVariants::Migrate => self.migrate_cli(&state_interface),
                ActionVariants::ContractInfo => {
                    let contract_info = ContractInfo {
                        addr: self.address().ok(),
                        code_id: self.code_id().ok(),
                    };
                    println!("{}", serde_json::to_string_pretty(&contract_info).unwrap());
                    Ok(())
                }
                ActionVariants::Addons => self.addons(context.clone()),
                ActionVariants::Quit => return Ok(()),
            };
            match res {
                Err(OrchCliError::InquireError(InquireError::OperationCanceled)) | Ok(_) => {}
                Err(err) => {
                    // Unrecoverable error?
                    return Err(err);
                }
            }
        }
    }

    fn instantiate_cli(&self, state_interface: &Rc<DaemonState>) -> OrchCliResult<()> {
        let instantiate_msg =
            <Self as InstantiableContract>::InstantiateMsg::cw_parse(state_interface)?;
        let coins = helpers::parse_coins()?;

        let admin = Text::new("Admin addr")
            .with_help_message("Press ESC to not set admin")
            .prompt_skippable()?
            .map(Addr::unchecked);

        if helpers::confirm_action("Execute", &instantiate_msg, Some(coins.as_slice()))? {
            let res = self.instantiate(&instantiate_msg, admin.as_ref(), Some(coins.as_slice()))?;
            println!(
                "Instantiation succesfull\naddr: {}\nhash: {}",
                self.addr_str()?,
                res.txhash
            );
        }
        Ok(())
    }

    fn execute_cli(&self, state_interface: &Rc<DaemonState>) -> OrchCliResult<()> {
        let execute_msg = <Self as ExecutableContract>::ExecuteMsg::cw_parse(state_interface)?;
        // TODO: figure out a way to make this only with `payable` attribute
        let coins = helpers::parse_coins()?;

        if helpers::confirm_action("Execute", &execute_msg, Some(coins.as_slice()))? {
            let res = self.execute(&execute_msg, Some(coins.as_slice()))?;
            println!("Execution succesfull, hash: {}", res.txhash);
        }
        Ok(())
    }

    fn query_cli(&self, state_interface: &Rc<DaemonState>) -> OrchCliResult<()> {
        let query_msg = <Self as QueryableContract>::QueryMsg::cw_parse(state_interface)?;

        let resp: serde_json::Value = self.query(&query_msg)?;
        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        Ok(())
    }

    fn migrate_cli(&self, state_interface: &Rc<DaemonState>) -> OrchCliResult<()> {
        let new_code_id = inquire::CustomType::<u64>::new("New code_id").prompt()?;
        let migrate_msg = <Self as MigratableContract>::MigrateMsg::cw_parse(state_interface)?;

        if helpers::confirm_action("Migrate", &migrate_msg, None)? {
            let res = self.migrate(&migrate_msg, new_code_id)?;
            println!("Migrate succesfull, hash: {}", res.txhash);
        }
        Ok(())
    }
}

pub trait ParseCwMsg
where
    Self: Sized,
{
    fn cw_parse(state: &impl cw_orch::state::StateInterface) -> OrchCliResult<Self>;
}

impl ParseCwMsg for Empty {
    fn cw_parse(_state: &impl cw_orch::state::StateInterface) -> OrchCliResult<Self> {
        Ok(Empty {})
    }
}

pub trait AddonsContext: Clone {}

impl<T: Clone> AddonsContext for T {}

pub trait CwCliAddons<AddonsContext> {
    fn addons(&mut self, context: AddonsContext) -> OrchCliResult<()>
    where
        Self: ContractInstance<Daemon>;
}

impl<T, C> ContractCli<C> for T
where
    C: AddonsContext,
    T: ContractInstance<Daemon>
        + CwOrchUpload<Daemon>
        + InstantiableContract
        + ExecutableContract
        + QueryableContract
        + MigratableContract
        + CwCliAddons<C>,
    <T as InstantiableContract>::InstantiateMsg: ParseCwMsg,
    <T as ExecutableContract>::ExecuteMsg: ParseCwMsg,
    <T as QueryableContract>::QueryMsg: ParseCwMsg,
    <T as MigratableContract>::MigrateMsg: ParseCwMsg,
{
}

pub mod helpers {
    use super::*;

    pub fn parse_coins() -> InquireResult<Vec<Coin>> {
        let mut coins = Vec::new();
        loop {
            let coin = CustomType::<Coin>::new("Add coin to transaction")
                .with_placeholder("5ucosm")
                .with_help_message("Press ESC to finish adding coins")
                .prompt_skippable()?;
            if let Some(c) = coin {
                coins.push(c)
            } else {
                break;
            }
        }
        Ok(coins)
    }

    pub fn custom_type_serialize<Msg: Serialize + DeserializeOwned + Clone>(
        message: &str,
    ) -> InquireResult<Msg> {
        let msg = CustomType {
            message,
            default: None,
            placeholder: None,
            help_message: None,
            formatter: &|val: Msg| serde_json::to_string(&val).unwrap(),
            default_value_formatter: &|val| serde_json::to_string(&val).unwrap(),
            parser: &|input| serde_json::from_str(input).map_err(|_| ()),
            validators: CustomType::DEFAULT_VALIDATORS,
            error_message: "Serialization failed".to_owned(),
            render_config: RenderConfig::default_colored(),
        }
        .prompt()?;

        Ok(msg)
    }

    pub fn select_msg<T: Display>(options: Vec<T>) -> InquireResult<T> {
        let variant = inquire::Select::new("Select Message", options).prompt()?;
        Ok(variant)
    }

    pub fn confirm_action<T: Serialize>(
        action: &str,
        message: T,
        coins: Option<&[Coin]>,
    ) -> InquireResult<bool> {
        let mut message = format!(
            "Confirm {action}, with message: {}",
            serde_json::to_string(&message).unwrap()
        );
        if let Some(c) = coins {
            let coins_str = c.iter().map(|c| c.to_string()).collect::<Vec<String>>();
            write!(message, ", and attached coins: {coins_str:?} y/n?",).unwrap();
        }
        Ok(Confirm::new(&message)
            .with_default(true)
            .prompt_skippable()?
            == Some(true))
    }
}
