pub mod msg_type;
mod query;
mod tx;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct CwCommands {
    #[interactive_clap(subcommand)]
    action: CwAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select cosmwasm action
pub enum CwAction {
    /// Transaction
    #[strum_discriminants(strum(message = "Transaction"))]
    Tx(tx::TxCommands),
    /// Query
    #[strum_discriminants(strum(message = "Query"))]
    Query(query::QueryCommands),
}
