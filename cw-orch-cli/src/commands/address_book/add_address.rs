use crate::types::address_book;

use super::AddresBookContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = AddresBookContext)]
#[interactive_clap(output_context = AddAddressOutput)]
pub struct AddAddress {
    /// Alias on AddressBook
    alias: String,
    /// New Address for the alias
    address: String,
}

pub struct AddAddressOutput;

impl AddAddressOutput {
    fn from_previous_context(
        previous_context: AddresBookContext,
        scope: &<AddAddress as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain_info = previous_context.chain.chain_info();
        address_book::try_insert_account_id(chain_info, &scope.alias, &scope.address)?;
        Ok(AddAddressOutput)
    }
}
