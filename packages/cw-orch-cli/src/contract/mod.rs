use std::fmt::{Display, Write};

use cosmwasm_std::{Addr, Coin};
use cw_orch::prelude::{
    ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchMigrate, CwOrchQuery, CwOrchUpload,
    Daemon, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
};

use inquire::{ui::RenderConfig, Confirm, CustomType, Text};
use serde::{de::DeserializeOwned, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(EnumIter, Display)]
enum ActionVariants {
    Execute,
    Query,
    Deploy,
    Instantiate,
    Migrate,
    Quit,
}

pub struct ContractCli<
    Contract: ContractInstance<Daemon>
        + InstantiableContract
        + ExecutableContract
        + QueryableContract
        + MigratableContract,
> {
    pub(crate) contract: Contract,
}

pub trait ParseCwMsg
where
    Self: Sized,
{
    fn cw_parse() -> cw_orch::anyhow::Result<Self>;
}

impl<Contract> ContractCli<Contract>
where
    Contract: ContractInstance<Daemon>
        + CwOrchUpload<Daemon>
        + InstantiableContract
        + ExecutableContract
        + QueryableContract
        + MigratableContract,
    <Contract as InstantiableContract>::InstantiateMsg: ParseCwMsg,
    <Contract as ExecutableContract>::ExecuteMsg: ParseCwMsg,
    <Contract as QueryableContract>::QueryMsg: ParseCwMsg,
    <Contract as MigratableContract>::MigrateMsg: ParseCwMsg,
{
    pub fn select_action(contract: Contract) -> cw_orch::anyhow::Result<()> {
        let instance = ContractCli { contract };
        loop {
            let action =
                inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;
            match action {
                ActionVariants::Execute => instance.execute()?,
                ActionVariants::Query => instance.query()?,
                ActionVariants::Deploy => {
                    instance.contract.upload()?;
                    println!("Code_id: {}", instance.contract.addr_str()?);
                }
                ActionVariants::Instantiate => instance.instantiate()?,
                ActionVariants::Migrate => instance.migrate()?,
                ActionVariants::Quit => return Ok(()),
            }
        }
    }

    fn instantiate(&self) -> cw_orch::anyhow::Result<()> {
        let instantiate_msg = <Contract as InstantiableContract>::InstantiateMsg::cw_parse()?;
        let coins = helpers::parse_coins()?;

        let admin = Text::new("Admin addr")
            .with_help_message("Press ESC to not set admin")
            .prompt_skippable()?
            .map(Addr::unchecked);

        if Self::confirm_action("Execute", &instantiate_msg, Some(coins.as_slice()))? {
            let res = self.contract.instantiate(
                &instantiate_msg,
                admin.as_ref(),
                Some(coins.as_slice()),
            )?;
            println!("Instantiation succesfull, hash: {}", res.txhash);
        }
        Ok(())
    }

    fn execute(&self) -> cw_orch::anyhow::Result<()> {
        let execute_msg = <Contract as ExecutableContract>::ExecuteMsg::cw_parse()?;
        let coins = helpers::parse_coins()?;

        if Self::confirm_action("Execute", &execute_msg, Some(coins.as_slice()))? {
            let res = self
                .contract
                .execute(&execute_msg, Some(coins.as_slice()))?;
            println!("Execution succesfull, hash: {}", res.txhash);
        }
        Ok(())
    }

    fn query(&self) -> cw_orch::anyhow::Result<()> {
        let query_msg = <Contract as QueryableContract>::QueryMsg::cw_parse()?;

        let resp: serde_json::Value = self.contract.query(&query_msg)?;
        println!("{}", serde_json::to_string_pretty(&resp)?);
        Ok(())
    }

    fn migrate(&self) -> cw_orch::anyhow::Result<()> {
        let new_code_id = inquire::CustomType::<u64>::new("New code_id").prompt()?;
        let migrate_msg = <Contract as MigratableContract>::MigrateMsg::cw_parse()?;

        if Self::confirm_action("Migrate", &migrate_msg, None)? {
            let res = self.contract.migrate(&migrate_msg, new_code_id)?;
            println!("Migrate succesfull, hash: {}", res.txhash);
        }
        Ok(())
    }

    fn confirm_action<T: Serialize>(
        action: &str,
        message: T,
        coins: Option<&[Coin]>,
    ) -> cw_orch::anyhow::Result<bool> {
        let mut message = format!(
            "Confirm {action}, with message: {}",
            serde_json::to_string(&message).unwrap()
        );
        if let Some(c) = coins {
            let coins_str = c.iter().map(|c| c.to_string()).collect::<Vec<String>>();
            write!(message, ", and attached coins: {coins_str:?}",)?;
        }
        Ok(Confirm::new(&message).prompt_skippable()? == Some(true))
    }
}

pub mod helpers {
    use super::*;

    pub fn parse_coins() -> cw_orch::anyhow::Result<Vec<Coin>> {
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
    ) -> cw_orch::anyhow::Result<Msg> {
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

    pub fn select_msg<T: Display>(options: Vec<T>) -> cw_orch::anyhow::Result<T> {
        let variant = inquire::Select::new("Select Message", options).prompt()?;
        Ok(variant)
    }
}
