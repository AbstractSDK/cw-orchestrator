mod action;
mod address_book;
mod keys;

pub use action::CosmosContext;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use crate::GlobalConfig;

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(disable_back)]
#[interactive_clap(context = GlobalConfig)]
/// Select one of the options with up-down arrows and press enter to select action
pub enum Commands {
    /// Select action
    #[strum_discriminants(strum(message = "🎬 Action"))]
    Action(action::CosmosCommands),
    /// Add, View or Remove key
    #[strum_discriminants(strum(message = "🔑 Keys"))]
    Key(keys::KeyCommands),
    /// Handle Address Book
    #[strum_discriminants(strum(message = "📖 Address Book"))]
    AddressBook(address_book::AddressBookCommands),
    // TODO:
    // 1) Config management
}
