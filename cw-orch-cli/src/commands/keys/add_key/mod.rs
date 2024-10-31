use base64::Engine;
use cosmrs::bip32;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use crate::common::B64;
use crate::types::keys::{entry_for_seed, read_entries, save_entry_if_required};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = AddKeyContext)]
pub struct AddKeyCommand {
    #[interactive_clap(skip_default_input_arg)]
    /// Id of they key
    name: String,
    #[interactive_clap(subcommand)]
    key_actions: AddKeyActions,
}

impl AddKeyCommand {
    fn input_name(_context: &()) -> color_eyre::eyre::Result<Option<String>> {
        let entries = read_entries()?;
        let name = inquire::Text::new("Id of they key")
            .with_validator(move |s: &str| {
                if s.is_empty() {
                    return Ok(inquire::validator::Validation::Invalid(
                        inquire::validator::ErrorMessage::Custom(
                            "Empty key not allowed".to_owned(),
                        ),
                    ));
                };
                if entries.entries.contains(s) {
                    return Ok(inquire::validator::Validation::Invalid(
                        inquire::validator::ErrorMessage::Custom("Key already exist".to_owned()),
                    ));
                };
                Ok(inquire::validator::Validation::Valid)
            })
            .prompt()?;
        Ok(Some(name))
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(input_context = AddKeyContext)]
#[interactive_clap(output_context = AddKeyOutput)]
/// How you want to create a new key?
pub enum AddKeyActions {
    /// Generate new random key
    #[strum_discriminants(strum(message = "Generate new random key"))]
    New,
    /// Recover key from the seed phrase
    #[strum_discriminants(strum(message = "Recover key from the seed phrase"))]
    FromSeed,
}

#[derive(Clone)]
pub struct AddKeyContext(String);

impl AddKeyContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<AddKeyCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(AddKeyContext(scope.name.clone()))
    }
}

pub struct AddKeyOutput;

impl AddKeyOutput {
    fn from_previous_context(
        previous_context: AddKeyContext,
        scope:&<AddKeyActions as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let name = previous_context.0;
        let mnemonic = match scope {
            AddKeyActionsDiscriminants::New => {
                bip32::Mnemonic::random(rand_core::OsRng, Default::default())
            }
            AddKeyActionsDiscriminants::FromSeed => {
                let mnemonic_seed = inquire::Password::new("Mnemonic 🔑: ")
                    .with_display_mode(inquire::PasswordDisplayMode::Masked)
                    .with_display_toggle_enabled()
                    .with_help_message("ctrl+R to unmask")
                    .without_confirmation()
                    .prompt()?;
                bip32::Mnemonic::new(mnemonic_seed, Default::default())?
            }
        };
        let entry = entry_for_seed(&name)?;
        let password = B64.encode(mnemonic.phrase().as_bytes());
        entry.set_password(&password)?;
        save_entry_if_required(&name)?;
        println!("New key \"{name}\" added");
        Ok(AddKeyOutput)
    }
}
