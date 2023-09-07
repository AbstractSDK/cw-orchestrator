use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TxCommands {
    /// Chain id
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: CwAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Select cosmwasm action
pub enum CwAction {
    /// Execute
    #[strum_discriminants(strum(message = "Execute cosmwasm message"))]
    Execute,
    /// Query
    #[strum_discriminants(strum(message = "Query cosmwasm message"))]
    Query,
}
