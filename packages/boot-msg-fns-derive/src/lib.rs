#![recursion_limit = "128"]

mod helpers;
mod contract_execute_fns;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ContractExecuteFns, attributes(payable, impl_into))]
pub fn contract_execute(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    contract_execute_fns::contract_execute_fns_derive(ast)
}
