#![recursion_limit = "128"]

mod execute_fns;
mod helpers;
mod query_fns;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, ItemEnum};

#[proc_macro_derive(ExecuteFns, attributes(payable, impl_into, fn_name))]
pub fn cw_orch_execute(input: TokenStream) -> TokenStream {
    // We only parse and return the modified code if the flag is activated
    let ast = parse_macro_input!(input as DeriveInput);
    execute_fns::execute_fns_derive(ast)
}

#[proc_macro_derive(QueryFns, attributes(returns, impl_into, fn_name))]
pub fn cw_orch_query(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    query_fns::query_fns_derive(ast)
}
