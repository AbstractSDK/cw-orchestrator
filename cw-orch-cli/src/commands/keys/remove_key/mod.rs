use crate::common::entry_for_seed;

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
        let entry = entry_for_seed(&scope.name)?;
        entry.delete_password()?;
        Ok(RemoveKeyOutput)
    }
}
