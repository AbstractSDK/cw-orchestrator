mod add_key;
mod remove_key;
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
    AddKey(add_key::AddKeyCommand),
    /// Show key from keyring
    #[strum_discriminants(strum(message = "Show key of given id from the keyring"))]
    ShowKey(show_key::ShowKeyCommand),
    /// Remove key from the keyring
    #[strum_discriminants(strum(message = "Remove key from the keyring"))]
    RemoveKey(remove_key::RemoveKeyCommand),
}
