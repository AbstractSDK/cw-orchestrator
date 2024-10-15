use cw_orch::tokio::runtime::Runtime;

use crate::{
    common::show_addr_explorer,
    types::address_book::{self, select_alias},
};

use super::AddresBookContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = AddresBookContext)]
#[interactive_clap(output_context = ShowAddressOutput)]
pub struct ShowAddress {
    /// Address Book Alias for the Address
    #[interactive_clap(skip_default_input_arg)]
    alias: String,
}

impl ShowAddress {
    pub fn input_alias(context: &AddresBookContext) -> color_eyre::eyre::Result<Option<String>> {
        select_alias(context.chain.chain_info(), &context.global_config)
    }
}

pub struct ShowAddressOutput;

impl ShowAddressOutput {
    fn from_previous_context(
        previous_context: AddresBookContext,
        scope: &<ShowAddress as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let maybe_account_id = address_book::get_account_id(
            chain.chain_info(),
            &previous_context.global_config,
            &scope.alias,
        )?;

        match maybe_account_id {
            Some(account_id) => {
                println!("{account_id}");
                let runtime = Runtime::new()?;
                let _ = runtime.block_on(show_addr_explorer(
                    chain.chain_info().clone(),
                    account_id.as_ref(),
                ));
            }
            None => println!("Address not found"),
        }

        Ok(ShowAddressOutput)
    }
}
