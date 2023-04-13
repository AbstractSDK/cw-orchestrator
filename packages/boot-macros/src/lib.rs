#![recursion_limit = "128"]

extern crate proc_macro;
use syn::ItemFn;
use quote::quote;
use proc_macro::TokenStream;

extern crate boot_contract_derive;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, input: TokenStream) -> TokenStream {

    // If the in_contract feature is enabled, we don't add the boot_contract macro
    let input_fn = parse_macro_input!(input as ItemFn);

    let boot_contract_macro = quote! {
        #[boot_contract_raw]
        #input_fn
    };

    boot_contract_macro.into()
}
