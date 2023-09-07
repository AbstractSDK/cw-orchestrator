#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = RemoveKeyOutput)]
pub struct RemoveKeyCommand {
    /// Id of the key
    name: String,
}

pub struct RemoveKeyOutput;

impl RemoveKeyOutput {
    fn from_previous_context(
        _previous_context: (),
        scope:&<RemoveKeyCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let entry = keyring::Entry::new("cw-cli", &scope.name)?;
        entry.delete_password()?;
        Ok(RemoveKeyOutput)
    }
}
