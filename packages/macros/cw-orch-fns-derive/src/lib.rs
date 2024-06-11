#![recursion_limit = "128"]

mod execute_fns;
mod fns_derive;
mod helpers;
mod query_fns;

extern crate proc_macro;
use helpers::{MsgType, SyncType};
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemEnum};

/// Available attributes are :
/// payable - The Execute function can accept funds
/// fn_name - Modify the generated function name (useful for query or execute variants for instance)
/// disable_fields_sorting - By default the fields are sorted on named variants. Disabled this behavior
/// into - The field can be indicated in the generated function with a type that implements `Into` the field type
#[proc_macro_derive(ExecuteFns, attributes(cw_orch))]
pub fn cw_orch_execute(input: TokenStream) -> TokenStream {
    // We only parse and return the modified code if the flag is activated
    let ast = parse_macro_input!(input as ItemEnum);
    fns_derive::fns_derive(MsgType::Execute, SyncType::Sync, ast).into()
}

/// Available attributes are :
/// returns - The return type of the query
/// fn_name - Modify the generated function name (useful for query or execute variants for instance)
/// disable_fields_sorting - By default the fields are sorted on named variants. Disabled this behavior
/// into - The field can be indicated in the generated function with a type that implements `Into` the field type
#[proc_macro_derive(QueryFns, attributes(cw_orch))]
pub fn cw_orch_query(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let sync_gen = fns_derive::fns_derive(MsgType::Query, SyncType::Sync, ast.clone());
    let async_gen = fns_derive::fns_derive(MsgType::Query, SyncType::Async, ast);
    let tokens = quote::quote! {
        #sync_gen
        #async_gen
    };
    tokens.into()
}
