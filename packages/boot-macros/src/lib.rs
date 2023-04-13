#![recursion_limit = "128"]

extern crate proc_macro;
use syn::ItemFn;
use quote::quote;
use proc_macro::TokenStream;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, input: TokenStream) -> TokenStream {

    let cloned = input.clone();
    // If the in_contract feature is enabled, we don't add the boot_contract macro
    let input_fn = parse_macro_input!(cloned as ItemFn);

    #[cfg(feature="outside_contract")]
    {
        quote! {
            #[::boot_contract_derive::boot_contract_raw]
            #input_fn
        }.into()
    }

    #[cfg(not(feature="outside_contract"))]
    input
}
