extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{self, parse_macro_input, DeriveInput, Field, Visibility, Generics, GenericParam, parse_quote};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Error, Fields};
// https://crates.io/crates/convert_case
use convert_case::{Case, Casing};

// cargo watch -q -c -x 'expand --lib macro_dev'

// #[proc_macro_derive(contract)]
// pub fn derive_contract(item: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(item as DeriveInput);
//     let name = &ast.ident;

//     let token_stream2 = quote! {
//         impl #name {
//             pub fn test() -> String {
//                 format!("{:?}",#name)
//             }
//         }
//     };

//     TokenStream::from(token_stream2)
// }

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

pub fn derive_contract_impl(input: DeriveInput) -> TokenStream2 {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    // let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    impl_generics.
    let gen = quote! {
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #name {
                    #defaults
                }
            }
        }
    };
    gen.into()
}
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(terra_rust_script_derive::CosmWasmContract));
        }
    }
    generics
}

// Above we are making a TokenStream using extend()
// This is because TokenStream is an Iterator,
// so we can keep extending it.
//
// proc_macro2::TokenStream:- https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.TokenStream.html

// Read about
// quote:- https://docs.rs/quote/1.0.7/quote/
// quote_spanned:- https://docs.rs/quote/1.0.7/quote/macro.quote_spanned.html
// spans:- https://docs.rs/syn/1.0.54/syn/spanned/index.html