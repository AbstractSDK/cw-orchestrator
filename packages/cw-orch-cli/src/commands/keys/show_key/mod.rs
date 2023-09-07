use crate::utils::seed_phrase_for_id;

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
        let phrase = seed_phrase_for_id(&scope.name)?;
        println!("your seed phrase: {phrase}");
        Ok(ShowKeyOutput)
    }
}
