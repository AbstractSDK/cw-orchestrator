mod cw20;
mod native;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct TransferCommands {
    #[interactive_clap(subcommand)]
    action: TransferAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select cosmwasm action
pub enum TransferAction {
    /// Transfer native coins
    #[strum_discriminants(strum(message = "Transfer native coins"))]
    Native(native::NativeTransferCommands),
    /// Transfer cw20 coin
    #[strum_discriminants(strum(message = "Transfer cw20 coin"))]
    Cw20(cw20::Cw20TransferCommands),
    // TODO: cw720?
}
