use cw_orch::{
    prelude::{networks::parse_network, Daemon, DaemonBuilder, TxHandler},
    tokio::runtime::Runtime,
};
pub use strum;

use std::marker::PhantomData;

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
    Instantiate,
    Migrate,
    Quit,
}

pub struct ContractCli<
    Error: ContractError,
    CustomInitMsg: ContractStructMsg,
    CustomExecMsg: ContractEnumMsg,
    CustomQueryMsg: ContractEnumMsg,
    CustomMigrateMsg: ContractStructMsg,
> {
    pub(crate) init: PhantomData<CustomInitMsg>,
    pub(crate) exec: PhantomData<CustomExecMsg>,
    pub(crate) query: PhantomData<CustomQueryMsg>,
    pub(crate) migrate: PhantomData<CustomMigrateMsg>,
    pub(crate) error: PhantomData<Error>,
}

impl<
        Error: ContractError,
        CustomInitMsg: ContractStructMsg,
        CustomExecMsg: ContractEnumMsg,
        CustomQueryMsg: ContractEnumMsg,
        CustomMigrateMsg: ContractStructMsg,
    > ContractCli<Error, CustomInitMsg, CustomExecMsg, CustomQueryMsg, CustomMigrateMsg>
{
    pub fn select_action() -> cw_orch::anyhow::Result<()> {
        let network = inquire::Text::new("Chain id").prompt()?;
        let chain = parse_network(&network);
        let rt = Runtime::new()?;
        // TODO: option to change wallet
        let daemon = DaemonBuilder::default()
            .chain(chain)
            .handle(rt.handle())
            .build()?;
        loop {
            let action =
                inquire::Select::new("Select action", ActionVariants::iter().collect()).prompt()?;
            match action {
                ActionVariants::Execute => Self::execute()?,
                ActionVariants::Query => todo!(),
                ActionVariants::Instantiate => todo!(),
                ActionVariants::Migrate => todo!(),
                ActionVariants::Quit => return Ok(()),
            }
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
            validators: CustomType::DEFAULT_VALIDATORS,
            error_message: "Serialization failed".to_owned(),
            render_config: RenderConfig::default_colored(),
        }
        .prompt()?;
        println!(
            "execute_msg: {}",
            serde_json::to_string(&execute_msg).unwrap()
        );

        Ok(())
    }
}
