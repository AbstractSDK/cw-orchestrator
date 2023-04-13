#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(outside_contract)]
use boot_contract_derive::boot_contract as boot_contract_raw;

#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, mut input: TokenStream) -> TokenStream {

    // If the in_contract feature is enabled, we don't add the boot_contract macro
    #[cfg(outside_contract)]
    input.extend(boot_contract_raw(_attrs, input));
    input
}
