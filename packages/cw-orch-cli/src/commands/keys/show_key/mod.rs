use base64::{prelude::BASE64_STANDARD as B64, Engine};

use crate::utils::entry_for_seed;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ShowKeyOutput)]
pub struct ShowKeyCommand {
    /// Id of the key
    name: String,
}

pub struct ShowKeyOutput;

impl ShowKeyOutput {
    fn from_previous_context(
        _previous_context: (),
        scope:&<ShowKeyCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let entry = entry_for_seed(&scope.name)?;

        let password = entry.get_password()?;
        let phrase = String::from_utf8(B64.decode(password)?)?;
        println!("your seed phrase: {phrase}");
        Ok(ShowKeyOutput)
    }
}
