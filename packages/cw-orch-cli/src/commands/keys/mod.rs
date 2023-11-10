mod add_key;
mod remove_key;
mod show_address;
mod show_key;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct KeyCommands {
    #[interactive_clap(subcommand)]
    key_actions: KeyAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// Select key action
pub enum KeyAction {
    /// Add key to the keyring
    #[strum_discriminants(strum(message = "Add key to the keyring"))]
    Add(add_key::AddKeyCommand),
    /// Show seed from keyring
    #[strum_discriminants(strum(message = "Show key of given id from the keyring"))]
    Show(show_key::ShowKeyCommand),
    /// Remove key from the keyring
    #[strum_discriminants(strum(message = "Remove key from the keyring"))]
    Remove(remove_key::RemoveKeyCommand),
    /// Show address of the key
    #[strum_discriminants(strum(message = "Show address of the key"))]
    ShowAddress(show_address::ShowAddressCommand),
}
