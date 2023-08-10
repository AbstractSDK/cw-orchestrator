use std::fmt::Display;

use cw_orch::prelude::{
    ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchMigrate, CwOrchQuery, CwOrchUpload,
    Daemon, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
};
pub use cw_orch_cli_derive::ParseCwMsg;
pub use strum;

use inquire::{ui::RenderConfig, CustomType};
use serde::{de::DeserializeOwned, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator, VariantNames};

pub trait ContractError: From<cosmwasm_std::StdError> + 'static {}
impl<T> ContractError for T where T: From<cosmwasm_std::StdError> + 'static {}

pub trait ContractEnumMsg: Clone + Serialize + DeserializeOwned + VariantNames + 'static {}
impl<T> ContractEnumMsg for T where T: Clone + Serialize + DeserializeOwned + VariantNames + 'static {}

pub trait ContractStructMsg: Clone + Serialize + DeserializeOwned + 'static {}
impl<T> ContractStructMsg for T where T: Clone + Serialize + DeserializeOwned + 'static {}

#[derive(EnumIter, Display)]
pub enum ActionVariants {
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
        let instantiate_msg = <Contract as InstantiableContract>::InstantiateMsg::parse()?;
        self.contract.instantiate(&instantiate_msg, None, None)?;
        println!("Instantiation succesfull");
        Ok(())
    }

    fn execute(&self) -> cw_orch::anyhow::Result<()> {
        let execute_msg = <Contract as ExecutableContract>::ExecuteMsg::parse()?;

        // TODO: support attaching coins
        self.contract.execute(&execute_msg, None)?;

        Ok(())
    }

    fn query(&self) -> cw_orch::anyhow::Result<()> {
        let query_msg = <Contract as QueryableContract>::QueryMsg::parse()?;

        let resp: serde_json::Value = self.contract.query(&query_msg)?;
        println!("{}", serde_json::to_string_pretty(&resp)?);
        Ok(())
    }

    fn migrate(&self) -> cw_orch::anyhow::Result<()> {
        let new_code_id = inquire::CustomType::<u64>::new("New code_id").prompt()?;
        let migrate_msg = <Contract as MigratableContract>::MigrateMsg::parse()?;
        self.contract.migrate(&migrate_msg, new_code_id)?;
        Ok(())
    }
}

pub fn custom_type_serialize<'a, Msg: Serialize + DeserializeOwned + Clone>(
    message: &str,
) -> cw_orch::anyhow::Result<Msg> {
    let msg = CustomType {
        message,
        default: None,
        placeholder: None,
        help_message: None,
        formatter: &|val: Msg| serde_json::to_string(&val).unwrap(),
        default_value_formatter: &|val| serde_json::to_string(&val).unwrap(),
        parser: &|input| serde_json::from_str(&input).map_err(|_| ()),
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

pub trait ParseCwMsg
where
    Self: Sized,
{
    fn parse() -> cw_orch::anyhow::Result<Self>;
}
