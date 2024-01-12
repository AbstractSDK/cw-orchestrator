mod action;
mod keys;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Select one of the options with up-down arrows and press enter to select action
pub enum Commands {
    /// Select action
    #[strum_discriminants(strum(message = "🎬 Action"))]
    Action(action::CosmosCommands),
    /// Add, View or Remove key
    #[strum_discriminants(strum(message = "🔑 Manage keys"))]
    Key(keys::KeyCommands),
    // TODO:
    // 1) AddressBook
    // 2) Config management
}
