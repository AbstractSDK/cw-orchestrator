mod struct_fn_constructor;

use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, ItemEnum};

#[proc_macro_derive(Boot)]
pub fn cosmwasm_contract_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = struct_fn_constructor::derive_contract_impl(input).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}