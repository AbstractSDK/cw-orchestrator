use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use crate::commands::action::CosmosContext;

mod raw;
mod smart;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct QueryCommands {
    #[interactive_clap(subcommand)]
    action: QueryAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select cosmwasm action
pub enum QueryAction {
    /// Query wasm smart
    #[strum_discriminants(strum(message = "🤓Smart"))]
    Smart(smart::QuerySmartCommands),
    /// Query wasm raw state
    #[strum_discriminants(strum(message = "👉Raw"))]
    Raw(raw::QueryRawCommands),
}
