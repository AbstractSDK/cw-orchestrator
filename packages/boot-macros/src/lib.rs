#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::__private::TokenStream2;
use syn::ItemFn;
use syn::parse_macro_input;

use quote::quote;
#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, input: TokenStream) -> TokenStream {

    let input_fn = parse_macro_input!(input as ItemFn);
    

    // The boot macro part
    let mut new_input: TokenStream2;
    #[cfg(feature="outside_contract")] {
       new_input = quote!(
            #[::boot_contract_derive::boot_contract_raw]
            #input_fn
        );
    }
    #[cfg(not(feature="outside_contract"))] {
        new_input = quote!{
            #input_fn
        };
    }
    

    // The cosmwasm_std::entry_point part
    #[cfg(feature="library")] {
       new_input = quote!(
            #[::cosmwasm_std::entry_point]
            #new_input
        );
    }
    #[cfg(not(feature="library"))] {
        new_input = quote!{
            #new_input
        };
    }

    new_input.into()
}
