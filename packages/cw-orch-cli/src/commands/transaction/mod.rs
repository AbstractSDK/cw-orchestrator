mod cosmwasm_tx;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TxCommands {
    /// Chain id
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: TxAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Select type of transaction
pub enum TxAction {
    /// Cosmwasm Action
    #[strum_discriminants(strum(message = "Execute cosmwasm action"))]
    Cw(cosmwasm_tx::CwCommands),
}
