use crate::{types::CliLockedChain, GlobalConfig};

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod add_address;
mod fetch_cw_orch;
mod remove_address;
mod show_address;

#[derive(Clone, Debug)]
pub struct AddresBookContext {
    pub global_config: GlobalConfig,
    pub chain: CliLockedChain,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = GlobalConfig)]
#[interactive_clap(output_context = AddresBookContext)]
pub struct AddressBookCommands {
    #[interactive_clap(skip_default_input_arg)]
    chain_id: CliLockedChain,
    #[interactive_clap(subcommand)]
    key_actions: KeyAction,
}

impl AddressBookCommands {
    fn input_chain_id(_context: &GlobalConfig) -> color_eyre::eyre::Result<Option<CliLockedChain>> {
        crate::common::select_chain()
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = AddresBookContext)]
/// Select key action
pub enum KeyAction {
    /// Add or override an Address to your Address Book
    #[strum_discriminants(strum(message = "📝 Add or override an Address to your Address Book"))]
    Add(add_address::AddAddress),
    /// Show address from address book
    #[strum_discriminants(strum(message = "📌 Show Address from Address Book"))]
    Show(show_address::ShowAddress),
    /// Remove an Address from your Address Book
    #[strum_discriminants(strum(message = "❌ Remove an address from your address book"))]
    Remove(remove_address::RemoveAddress),
    /// Fetch addresses from cw-orchestrator state file
    #[strum_discriminants(strum(message = "🧷 Fetch addresses from cw-orchestrator state file"))]
    Fetch(fetch_cw_orch::FetchAddresses),
}

impl AddresBookContext {
    fn from_previous_context(
        previous_context: GlobalConfig,
        scope:&<AddressBookCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(AddresBookContext {
            global_config: previous_context,
            chain: scope.chain_id,
        })
    }
}
