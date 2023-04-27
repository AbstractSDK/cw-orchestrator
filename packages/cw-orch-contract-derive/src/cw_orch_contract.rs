extern crate proc_macro;
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{FnArg, Pat, Signature};

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
        let mut arg_type = arg.clone();
        // We need to get rid of the following error that happends when using mut deps for instance
        // error[E0561]: patterns aren't allowed in function pointer types
        let arg_without_mut = match &mut arg_type {
            FnArg::Receiver(_) => panic!("self is not allowed in a function endpoint"),
            FnArg::Typed(typed_argument) => {
                match &mut *typed_argument.pat {
                    Pat::Ident(id) => {
                        id.mutability = None; // We simply remove the mut from the function pointer type
                        typed_argument.pat = Box::new(Pat::Ident(id.clone()));
                        FnArg::Typed(typed_argument.clone())
                    }
                    _ => arg_type,
                }
            }
        };
        quote! { #arg_without_mut }
    });

    quote! {
        fn(#(#arg_types),*) -> #output_type
    }
}
