use crate::types::keys::entry_for_seed;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = RemoveKeyOutput)]
pub struct RemoveKeyCommand {
    #[interactive_clap(skip_default_input_arg)]
    name: String,
}

impl RemoveKeyCommand {
    fn input_name(_: &()) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct RemoveKeyOutput;

impl RemoveKeyOutput {
    fn from_previous_context(
        _previous_context: (),
        scope:&<RemoveKeyCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let entry = entry_for_seed(&scope.name)?;
        entry.delete_password()?;
        crate::types::keys::remove_entry(&scope.name)?;
        println!("Key \"{}\" got removed", scope.name);
        Ok(RemoveKeyOutput)
    }
}
