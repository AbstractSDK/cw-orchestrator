use crate::types::address_book;

use super::AddresBookContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = AddresBookContext)]
#[interactive_clap(output_context = RemoveAddressOutput)]
pub struct RemoveAddress {
    /// Address Book Alias for the Address
    #[interactive_clap(skip_default_input_arg)]
    alias: String,
}

impl RemoveAddress {
    pub fn input_alias(context: &AddresBookContext) -> color_eyre::eyre::Result<Option<String>> {
        // Disable state merging, CLI do not remove items from cw-orch state
        let mut config = context.global_config.clone();
        config.source_state_file = false;

        address_book::select_alias(context.chain.chain_info(), &config)
    }
}

pub struct RemoveAddressOutput;

impl RemoveAddressOutput {
    fn from_previous_context(
        previous_context: AddresBookContext,
        scope: &<RemoveAddress as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let removed = address_book::remove_account_id(chain.chain_info().chain_id, &scope.alias)?;

        match removed {
            Some(val) => println!("removed: {val}"),
            None => println!("No updates!"),
        }
        Ok(RemoveAddressOutput)
    }
}
