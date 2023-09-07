use base64::{prelude::BASE64_STANDARD as B64, Engine};
use color_eyre::eyre::Context;
use cosmrs::bip32;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = AddKeyContext)]
pub struct AddKeyCommand {
    // TODO: add checker for repetition
    // #[interactive_clap(skip_default_input_arg)]
    /// Id of they key
    name: String,
    #[interactive_clap(subcommand)]
    key_actions: AddKeyActions,
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
                bip32::Mnemonic::random(&mut rand_core::OsRng, Default::default())
            }
            AddKeyActionsDiscriminants::FromSeed => {
                // TODO: do we want to hide the input?
                let mnemonic_seed =
                    rpassword::prompt_password("Mnemonic 🔑: ").context("unable to read")?;
                bip32::Mnemonic::new(mnemonic_seed, Default::default())?
            }
        };
        let entry = keyring::Entry::new("cw-cli", &name)?;

        let password = B64.encode(mnemonic.phrase().as_bytes());
        entry.set_password(&password)?;
        Ok(AddKeyOutput)
    }
}
