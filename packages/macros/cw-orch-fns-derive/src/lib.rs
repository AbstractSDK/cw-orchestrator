#![recursion_limit = "128"]

mod execute_fns;
mod fns_derive;
mod helpers;
mod query_fns;

extern crate proc_macro;
use helpers::MsgType;
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(
    ExecuteFns,
    attributes(payable, impl_into, fn_name, disable_fields_sorting)
)]
pub fn cw_orch_execute(input: TokenStream) -> TokenStream {
    // We only parse and return the modified code if the flag is activated
    let ast = parse_macro_input!(input as ItemEnum);
    fns_derive::fns_derive(MsgType::Execute, ast)
}

#[proc_macro_derive(
    QueryFns,
    attributes(returns, impl_into, fn_name, disable_fields_sorting)
)]
pub fn cw_orch_query(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    fns_derive::fns_derive(MsgType::Query, ast)
}
