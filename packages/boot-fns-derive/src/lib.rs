#![recursion_limit = "128"]

mod execute_fns;
mod helpers;
mod query_fns;

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(feature="outside_contract")]
use syn::{parse_macro_input, DeriveInput, ItemEnum};

#[proc_macro_derive(ExecuteFns, attributes(payable, impl_into))]
pub fn boot_execute(input: TokenStream) -> TokenStream {

    // We only parse and return the modified code if the flag is activated

    #[cfg(feature="outside_contract")] {
        let ast = parse_macro_input!(input as DeriveInput);
        execute_fns::execute_fns_derive(ast)
    }

    #[cfg(not(feature="outside_contract"))] 
    input

    
}

#[proc_macro_derive(QueryFns, attributes(returns, impl_into))]
pub fn boot_query(input: TokenStream) -> TokenStream {

    #[cfg(feature="outside_contract")] {
        let ast = parse_macro_input!(input as ItemEnum);
        query_fns::query_fns_derive(ast)
    }

    #[cfg(not(feature="outside_contract"))] 
    input
}
