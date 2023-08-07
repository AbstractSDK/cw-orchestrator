pub use strum;

use std::marker::PhantomData;

use inquire::{ui::RenderConfig, CustomType};
use serde::{de::DeserializeOwned, Serialize};
use strum::{EnumIter, IntoEnumIterator, VariantNames};

pub trait ContractError: From<cosmwasm_std::StdError> + 'static {}

impl<T> ContractError for T where T: From<cosmwasm_std::StdError> + 'static {}

#[derive(EnumIter)]
pub enum ActionVariants {
    Execute,
    Query,
    Instantiate,
    Migrate,
}

impl std::fmt::Display for ActionVariants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionVariants::Execute => write!(f, "execute"),
            ActionVariants::Query => write!(f, "query"),
            ActionVariants::Instantiate => write!(f, "instantiate"),
            ActionVariants::Migrate => write!(f, "migrate"),
        }
    }
}

pub struct ContractCli<
    Error: ContractError,
    CustomInitMsg: Clone + Serialize + DeserializeOwned + 'static,
    CustomExecMsg: Clone + Serialize + DeserializeOwned + VariantNames + 'static,
    CustomQueryMsg: Clone + Serialize + DeserializeOwned + 'static,
    CustomMigrateMsg: Clone + Serialize + DeserializeOwned + 'static,
> {
    pub(crate) init: PhantomData<CustomInitMsg>,
    pub(crate) exec: PhantomData<CustomExecMsg>,
    pub(crate) query: PhantomData<CustomQueryMsg>,
    pub(crate) migrate: PhantomData<CustomMigrateMsg>,
    pub(crate) error: PhantomData<Error>,
}

impl<
        Error: ContractError,
        CustomInitMsg: Clone + Serialize + DeserializeOwned + 'static,
        CustomExecMsg: Clone + Serialize + DeserializeOwned + VariantNames + 'static,
        CustomQueryMsg: Clone + Serialize + DeserializeOwned + 'static,
        CustomMigrateMsg: Clone + Serialize + DeserializeOwned + 'static,
    > ContractCli<Error, CustomInitMsg, CustomExecMsg, CustomQueryMsg, CustomMigrateMsg>
{
    pub fn select_action() -> cw_orch::anyhow::Result<()> {
        let action =
            inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;

        match action {
            ActionVariants::Execute => Self::execute(),
            ActionVariants::Query => todo!(),
            ActionVariants::Instantiate => todo!(),
            ActionVariants::Migrate => todo!(),
        }
    }
    fn execute() -> cw_orch::anyhow::Result<()> {
        let variant =
            inquire::Select::new("Select Execute Message", CustomExecMsg::VARIANTS.to_vec())
                .prompt()?
                .to_lowercase();
        let execute_msg: CustomExecMsg = CustomType {
            message: "Execute Msg",
            default: None,
            placeholder: Some("{\"key\": \"value\"}"),
            help_message: None,
            formatter: &|val: CustomExecMsg| serde_json::to_string(&val).unwrap(),
            default_value_formatter: &|val| serde_json::to_string(&val).unwrap(),
            parser: &|input| {
                let s = format!("{{\"{variant}\": {input}}}");
                serde_json::from_str(&s).map_err(|_| ())
            },
            validators: vec![],
            error_message: "Serialization failed".to_owned(),
            render_config: RenderConfig::empty(),
        }
        .prompt()?;
        println!(
            "execute_msg: {}",
            serde_json::to_string(&execute_msg).unwrap()
        );

        Ok(())
    }
}
