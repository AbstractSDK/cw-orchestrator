use proc_macro::TokenStream;
extern crate proc_macro;
use quote::quote;

// TODO: check what's the best way to serialize for user
#[proc_macro_attribute]
pub fn cli(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = item.into();
    let output = quote! {
        #[derive(::cw_orch_cli::strum::EnumDiscriminants, ::cw_orch_cli::strum::EnumIter, ::cw_orch_cli::strum::EnumVariantNames)]
        #[strum(serialize_all = "snake_case")]
        #[strum(crate = "::cw_orch_cli::strum")]
        #input
    };
    output.into()
}
