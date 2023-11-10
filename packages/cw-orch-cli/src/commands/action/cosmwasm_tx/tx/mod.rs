use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use crate::commands::action::CosmosContext;

mod execute;
mod instantiate;
mod store;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct TxCommands {
    #[interactive_clap(subcommand)]
    action: TxAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select cosmwasm action
pub enum TxAction {
    /// Store contract
    #[strum_discriminants(strum(message = "Store contract"))]
    Store(store::StoreContractCommands),
    /// Instantiate contract
    #[strum_discriminants(strum(message = "Instantiate contract"))]
    Instantiate(instantiate::InstantiateContractCommands),
    /// Execute contract method
    #[strum_discriminants(strum(message = "Execute contract method"))]
    Execute(execute::ExecuteContractCommands),
}
