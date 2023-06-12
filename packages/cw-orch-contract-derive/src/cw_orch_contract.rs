extern crate proc_macro;
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{FnArg, Pat, Signature, __private::TokenStream2};

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

/// Returns the function type (e.g. fn (deps: Deps) -> Result<Response, ContractError>) from the function signature object
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

/// Returns the first generic of a path object
fn get_first_generic(path: &syn::Path) -> Result<syn::GenericArgument, String> {
    let args = match path.segments[0].arguments {
        syn::PathArguments::AngleBracketed(ref angle_bracketed) => &angle_bracketed.args,
        _ => return Err("Expected angle-bracketed arguments".to_string()),
    };
    Ok(args[0].clone())
}

/// Returns the type of the generics of the response type from the signature of the instantiate function
/// e.g for
/// `pub fn instantiate(...) -> Result<Response<Generic>, ContractError>`
/// This returns
/// `Generic`
fn get_response_generic(func_name: &str, signature: &Signature) -> Result<TokenStream2, String> {
    let response_type = match func_name {
        "instantiate" => match signature.clone().output {
            syn::ReturnType::Type(_, ty) => {
                if let syn::Type::Path(syn::TypePath { path, .. }) = *ty {
                    // Here we have the path that corresponds to Result<Response<R>, Error>
                    get_first_generic(&path)
                } else {
                    return Err("Instantiate function return type must be a path".to_string());
                }
            }
            syn::ReturnType::Default => {
                return Err("Instantiate function must have a return type".to_string())
            }
        },
        _ => return Err("Not instantiate entry point".to_string()),
    }?;

    let parsed_response_path = match response_type {
        syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) => path,
        _ => return Err("Response type didn't have any specified generics".to_string()),
    };

    let response_generic = get_first_generic(&parsed_response_path)?;

    Ok(quote!(#response_generic))
}

/// Generates a fallback for the get_response_generic function to always find an generic even if it's not clearly specified (and never error on the generics search)
pub fn get_response_generic_or_fallback(func_name: &str, signature: &Signature) -> TokenStream2 {
    let generic = get_response_generic(func_name, signature);

    match generic {
        Err(_) => quote!(Empty),
        Ok(o) => o,
    }
}
