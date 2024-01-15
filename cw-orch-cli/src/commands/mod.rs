mod action;
mod address_book;
mod keys;

// TODO: get it upper
pub use action::CosmosContext;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Select one of the options with up-down arrows and press enter to select action
pub enum Commands {
    /// Select action
    #[strum_discriminants(strum(message = "🎬 Action"))]
    Action(action::CosmosCommands),
    /// Add, View or Remove key
    #[strum_discriminants(strum(message = "🔑 Manage Keys"))]
    Key(keys::KeyCommands),
    /// Handle Address Book
    #[strum_discriminants(strum(message = "📖 Manage Address Book"))]
    AddressBook(address_book::AddressBookCommands),
    // TODO:
    // 1) Config management
}
