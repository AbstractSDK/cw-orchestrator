#![recursion_limit = "128"]

extern crate proc_macro;
use syn::ItemFn;
use quote::quote;
use proc_macro::TokenStream;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, input: TokenStream) -> TokenStream {

    let input_fn = parse_macro_input!(input as ItemFn);

    quote!{
        #[cfg_attr(not(feature = "library"), ::cosmwasm_std::entry_point)]
        #[cfg_attr(feature="outside_contract", ::boot_contract_derive::boot_contract_raw)]
        #input_fn
    }.into()
}
