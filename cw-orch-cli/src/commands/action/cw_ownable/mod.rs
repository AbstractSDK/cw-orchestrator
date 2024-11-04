use crate::commands::action::CosmosContext;

use serde::Serialize;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod accept;
mod get;
mod renounce;
mod transfer;

// Helper enum to serialize execute
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum ContractExecuteMsg {
    UpdateOwnership(cw_ownable::Action),
}

// Helper enum to serialize query
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ContractQueryMsg {
    Ownership {},
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct CwOwnableCommands {
    #[interactive_clap(subcommand)]
    action: CwOwnableAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select CW-Ownable action
pub enum CwOwnableAction {
    /// Propose to transfer contract ownership to another address
    #[strum_discriminants(strum(message = "💍 Propose ownership to another address."))]
    Transfer(transfer::TransferOwnership),
    /// Accept pending ownership
    #[strum_discriminants(strum(message = "✅ Accept pending ownership."))]
    Accept(accept::AcceptOwnership),
    /// Renounce pending ownership
    #[strum_discriminants(strum(message = "🚫 Renounce pending ownership"))]
    Renounce(renounce::RenounceOwnership),
    /// Get current ownership
    #[strum_discriminants(strum(message = "❓ Get current ownership"))]
    Get(get::GetOwnership),
}
