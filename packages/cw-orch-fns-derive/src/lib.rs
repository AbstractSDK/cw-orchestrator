#![recursion_limit = "128"]

mod execute_fns;
mod helpers;
mod query_fns;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, ItemEnum};

#[proc_macro_derive(
    ExecuteFns,
    attributes(payable, impl_into, fn_name, disable_fields_sorting)
)]
pub fn cw_orch_execute(input: TokenStream) -> TokenStream {
    // We only parse and return the modified code if the flag is activated
    let ast = parse_macro_input!(input as DeriveInput);
    execute_fns::execute_fns_derive(ast)
}

#[proc_macro_derive(
    QueryFns,
    attributes(returns, impl_into, fn_name, disable_fields_sorting)
)]
pub fn cw_orch_query(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let sync_tokens = query_fns::query_fns_derive(ast.clone());
    let async_tokens = query_fns::async_query_fns_derive(ast);

    let tokens = quote::quote! {
        #sync_tokens
        #async_tokens
    };
    tokens.into()
}
