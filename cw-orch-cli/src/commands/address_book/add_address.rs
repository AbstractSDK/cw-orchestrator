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
        let chain = previous_context.chain;
        let maybe_account_id =
            address_book::get_account_id(chain.chain_info().chain_id, &scope.alias)?;

        if let Some(account_id) = maybe_account_id {
            let confirmed =
                inquire::Confirm::new(&format!("Override {}({account_id})?", scope.alias))
                    .prompt()?;
            if !confirmed {
                return Ok(AddAddressOutput);
            }
        }

        let new_address = address_book::insert_account_id(chain.chain_info().chain_id, &scope.alias, &scope.address)?;
        println!("Wrote successfully:\n{}:{}", scope.alias, new_address);
        Ok(AddAddressOutput)
    }
}
