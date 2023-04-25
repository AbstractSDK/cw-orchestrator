extern crate proc_macro;
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::Signature;
use syn::__private::TokenStream2;

pub fn get_crate_to_struct() -> syn::Ident {
    let kebab_case_pkg = get_raw_crate();
    let name = kebab_case_pkg.to_case(Case::Pascal);

    format_ident!("{}", name)
}

pub fn get_wasm_name() -> String {
    let kebab_case_pkg = get_raw_crate();
    kebab_case_pkg.replace('-', "_")
}

pub fn get_raw_crate() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap()
}

pub fn get_func_type(sig: &Signature) -> TokenStream2 {
    let output_type = match &sig.output {
        syn::ReturnType::Default => {
            quote! { () }
        }
        syn::ReturnType::Type(_, ty) => {
            quote! { #ty }
        }
    };
    let arg_types = sig.inputs.iter().map(|arg| {
        let arg_type = &arg;
        quote! { #arg_type }
    });

    quote! {
        fn(#(#arg_types),*) -> #output_type
    }
}
