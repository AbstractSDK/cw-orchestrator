use proc_macro::TokenStream;
extern crate proc_macro;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_attribute]
pub fn cli(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output: DeriveInput = match input.data {
        syn::Data::Struct(_) => parse_quote!(#input),
        syn::Data::Enum(_) => {
            parse_quote!(
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    derive(::cw_orch_cli::strum::EnumVariantNames)
                )]
                #[strum(serialize_all = "snake_case")]
                #[strum(crate = "::cw_orch_cli::strum")]
                #input
            )
        }
        syn::Data::Union(_) => parse_quote!(#input),
    };
    TokenStream::from(output.into_token_stream())
}
