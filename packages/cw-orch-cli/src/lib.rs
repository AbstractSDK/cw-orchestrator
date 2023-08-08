use cw_orch::prelude::{
    ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchMigrate, CwOrchQuery, CwOrchUpload,
    Daemon, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
};
pub use strum;

use inquire::{parser::CustomTypeParser, ui::RenderConfig, CustomType};
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
> where
    <Contract as InstantiableContract>::InstantiateMsg: ContractStructMsg,
    <Contract as ExecutableContract>::ExecuteMsg: ContractEnumMsg,
{
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
    <Contract as InstantiableContract>::InstantiateMsg: ContractStructMsg,
    <Contract as ExecutableContract>::ExecuteMsg: ContractEnumMsg,
    <Contract as QueryableContract>::QueryMsg: ContractEnumMsg,
    <Contract as MigratableContract>::MigrateMsg: ContractStructMsg,
{
    pub fn select_action(contract: Contract) -> cw_orch::anyhow::Result<()> {
        let instance = ContractCli { contract };
        loop {
            let action =
                inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;
            match action {
                ActionVariants::Execute => instance.execute()?,
                ActionVariants::Query => instance.query()?,
                ActionVariants::Deploy => instance.contract.upload().map(|_| ())?,
                ActionVariants::Instantiate => instance.instantiate()?,
                ActionVariants::Migrate => instance.migrate()?,
                ActionVariants::Quit => return Ok(()),
            }
        }
    }

    fn instantiate(&self) -> cw_orch::anyhow::Result<()> {
        let instantiate_msg: <Contract as InstantiableContract>::InstantiateMsg =
            Self::msg("Instantiate Message", &|input| {
                serde_json::from_str(&input).map_err(|_| ())
            })?;
        self.contract.instantiate(&instantiate_msg, None, None)?;
        Ok(())
    }

    fn execute(&self) -> cw_orch::anyhow::Result<()> {
        let variant = inquire::Select::new(
            "Select Execute Message",
            <Contract as ExecutableContract>::ExecuteMsg::VARIANTS.to_vec(),
        )
        .prompt()?
        .to_lowercase();
        let execute_msg: <Contract as ExecutableContract>::ExecuteMsg =
            Self::msg("Execute Message", &|input| {
                let s = format!("{{\"{variant}\": {input}}}");
                serde_json::from_str(&s).map_err(|_| ())
            })?;

        // TODO: support attaching coins
        self.contract.execute(&execute_msg, None)?;

        Ok(())
    }

    fn query(&self) -> cw_orch::anyhow::Result<()> {
        let variant = inquire::Select::new(
            "Select Query Message",
            <Contract as QueryableContract>::QueryMsg::VARIANTS.to_vec(),
        )
        .prompt()?;
        let query_msg: <Contract as QueryableContract>::QueryMsg =
            Self::msg("Query Message", &|input| {
                let s = format!("{{\"{variant}\": {input}}}");
                serde_json::from_str(&s).map_err(|_| ())
            })?;
        let resp: serde_json::Value = self.contract.query(&query_msg)?;
        println!("{}", serde_json::to_string_pretty(&resp)?);
        Ok(())
    }

    fn migrate(&self) -> cw_orch::anyhow::Result<()> {
        let new_code_id = inquire::CustomType::<u64>::new("New code_id").prompt()?;
        let migrate_msg: <Contract as MigratableContract>::MigrateMsg =
            Self::msg("Migrate Message", &|input| {
                serde_json::from_str(&input).map_err(|_| ())
            })?;
        self.contract.migrate(&migrate_msg, new_code_id)?;
        Ok(())
    }

    fn msg<'a, Msg: Serialize + Clone>(
        message: &str,
        parser: CustomTypeParser<'a, Msg>,
    ) -> cw_orch::anyhow::Result<Msg> {
        let msg = CustomType {
            message,
            default: None,
            placeholder: Some("{\"key\": \"value\"}"),
            help_message: None,
            formatter: &|val: Msg| serde_json::to_string(&val).unwrap(),
            default_value_formatter: &|val| serde_json::to_string(&val).unwrap(),
            parser,
            validators: CustomType::DEFAULT_VALIDATORS,
            error_message: "Serialization failed".to_owned(),
            render_config: RenderConfig::default_colored(),
        }
        .prompt()?;
        Ok(msg)
    }
}
