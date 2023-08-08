use cw_orch::{
    daemon::CosmTxResponse,
    prelude::{
        ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchUpload, Daemon,
        ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
    },
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

impl<
        Contract: ContractInstance<Daemon>
            + CwOrchUpload<Daemon>
            + InstantiableContract
            + ExecutableContract
            + QueryableContract
            + MigratableContract,
    > ContractCli<Contract>
where
    <Contract as InstantiableContract>::InstantiateMsg: ContractStructMsg,
    <Contract as ExecutableContract>::ExecuteMsg: ContractEnumMsg,
{
    pub fn select_action(contract: Contract) -> cw_orch::anyhow::Result<()> {
        let instance = ContractCli { contract };
        loop {
            let action =
                inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;
            let result = match action {
                ActionVariants::Execute => instance.execute()?,
                ActionVariants::Query => todo!(),
                ActionVariants::Deploy => instance.contract.upload()?,
                ActionVariants::Instantiate => instance.instantiate()?,
                ActionVariants::Migrate => todo!(),
                ActionVariants::Quit => return Ok(()),
            };
            log::debug!("tx result: {result:?}")
        }
    }

    fn instantiate(&self) -> cw_orch::anyhow::Result<CosmTxResponse> {
        let instantiate_msg: <Contract as InstantiableContract>::InstantiateMsg =
            Self::msg(&|input| serde_json::from_str(&input).map_err(|_| ()))?;
        let res = self.contract.instantiate(&instantiate_msg, None, None)?;
        Ok(res)
    }

    fn execute(&self) -> cw_orch::anyhow::Result<CosmTxResponse> {
        let variant = inquire::Select::new(
            "Select Execute Message",
            <Contract as ExecutableContract>::ExecuteMsg::VARIANTS.to_vec(),
        )
        .prompt()?
        .to_lowercase();
        let execute_msg: <Contract as ExecutableContract>::ExecuteMsg = Self::msg(&|input| {
            let s = format!("{{\"{variant}\": {input}}}");
            serde_json::from_str(&s).map_err(|_| ())
        })?;

        // TODO: support attaching coins
        let res = self.contract.execute(&execute_msg, None)?;

        Ok(res)
    }

    fn msg<'a, Msg: Serialize + Clone>(
        parser: CustomTypeParser<'a, Msg>,
    ) -> cw_orch::anyhow::Result<Msg> {
        let msg = CustomType {
            message: "Execute Msg",
            default: None,
            placeholder: Some("{\"key\": \"value\"}"),
            help_message: None,
            formatter: &|val: Msg| serde_json::to_string(&val).unwrap(),
            default_value_formatter: &|val| serde_json::to_string(&val).unwrap(),
            parser: parser,
            validators: CustomType::DEFAULT_VALIDATORS,
            error_message: "Serialization failed".to_owned(),
            render_config: RenderConfig::default_colored(),
        }
        .prompt()?;
        Ok(msg)
    }
}