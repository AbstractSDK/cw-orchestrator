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

// TODO: remove if unused
// #[derive(Clone)]
// pub struct CwActionContext {
//     chain_id: String,
// }

// impl CwActionContext {
//     fn from_previous_context(
//         previous_context: TxContext,
//         scope:&<CwCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
//     ) -> color_eyre::eyre::Result<Self> {
//         Ok(CwActionContext {
//             chain_id: previous_context.chain_id.clone(),
//         })
//     }
// }
