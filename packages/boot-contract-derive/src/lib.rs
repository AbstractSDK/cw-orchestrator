#![recursion_limit = "128"]

extern crate proc_macro;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{
    parse_macro_input, AttributeArgs, Fields, FnArg, Item, Meta, NestedMeta, Path, Signature,
};

#[proc_macro_attribute]
pub fn contract(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as syn::Item);
    let attributes = parse_macro_input!(attrs as AttributeArgs);

    let types_in_order: Vec<Path> = attributes
        .into_iter()
        .map(|attr| {
            let NestedMeta::Meta(Meta::Path(type_path)) = attr else {
            panic!("Expected a contract endpoint type.");
        };
            type_path
        })
        .collect();

    if types_in_order.len() != 4 {
        panic!("Expected four endpoint types (InstantiateMsg,ExecuteMsg,QueryMsg,MigrateMsg). Use cosmwasm_std::Empty if not implemented.")
    }

    let Item::Struct(boot_struct) = &mut item else {
        panic!("Only works on structs");
    };
    let Fields::Unit = &mut boot_struct.fields else {
        panic!("Struct must be unit-struct");
    };

    let init = types_in_order[0].clone();
    let exec = types_in_order[1].clone();
    let query = types_in_order[2].clone();
    let migrate = types_in_order[3].clone();

    let name = boot_struct.ident.clone();
    let struct_def = quote!(
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain: ::boot_core::CwEnv>(::boot_core::Contract<Chain>);

            impl<Chain: ::boot_core::CwEnv> ::boot_core::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::boot_core::Contract<Chain> {
            &self.0
        }
            fn as_instance_mut(&mut self) -> &mut ::boot_core::Contract<Chain> {
                &mut self.0
            }
        }

        impl<Chain: ::boot_core::CwEnv> ::boot_core::InstantiateableContract for #name<Chain> {
            type InstantiateMsg = #init;
        }

        impl<Chain: ::boot_core::CwEnv> ::boot_core::ExecuteableContract for #name<Chain> {
            type ExecuteMsg = #exec;
        }

        impl<Chain: ::boot_core::CwEnv> ::boot_core::QueryableContract for #name<Chain> {
            type QueryMsg = #query;
        }

        impl<Chain: ::boot_core::CwEnv> ::boot_core::MigrateableContract for #name<Chain> {
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}

fn get_crate_to_struct() -> syn::Ident {
    let kebab_case_pkg = get_raw_crate();
    let name = kebab_case_pkg.to_case(Case::Pascal);

    format_ident!("{}", name)
}

fn get_wasm_name() -> String {
    let kebab_case_pkg = get_raw_crate();
    kebab_case_pkg.replace('-', "_")
}

fn get_raw_crate() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap()
}

fn get_func_type(sig: &Signature) -> TokenStream2 {
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

#[proc_macro_attribute]
pub fn boot_contract(_attrs: TokenStream, mut input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let mut item = parse_macro_input!(cloned as syn::Item);

    let Item::Fn(boot_func) = &mut item else {
        panic!("Only works on functions");
    };

    // Now we get the fourth function argument that should be the instantiate message
    let signature = &mut boot_func.sig;
    let func_ident = signature.ident.clone();
    let func_type = get_func_type(signature);

    let message_idx = match func_ident.to_string().as_ref() {
        "instantiate" | "execute" => 3,
        "query" | "migrate" => 2,
        _ => panic!("Function name not supported for the macro"),
    };

    let message = match signature.inputs[message_idx].clone() {
        FnArg::Typed(syn::PatType { ty, .. }) => *ty,
        _ => panic!("Only typed arguments"),
    };

    let wasm_name = get_wasm_name();
    let name = get_crate_to_struct();

    let struct_def = quote!(
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain: ::boot_core::CwEnv>(::boot_core::Contract<Chain>);

            impl<Chain: ::boot_core::CwEnv> ::boot_core::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::boot_core::Contract<Chain> {
            &self.0
        }
            fn as_instance_mut(&mut self) -> &mut ::boot_core::Contract<Chain> {
                &mut self.0
            }
        }


        // We add the contract creation script
        impl<Chain: ::boot_core::CwEnv> #name<Chain> {
            pub fn new(contract_id: &str, chain: Chain) -> Self {

                log::info!("{:?}", #wasm_name);
                Self(
                    ::boot_core::Contract::new(contract_id, chain)
                        .with_wasm_path(#wasm_name) // Adds the wasm path for uploading to a node is simple
                         .with_mock(Box::new(
                            // Adds the contract's endpoint functions for mocking
                            ::boot_core::ContractWrapper::new_with_empty(
                                #name::<Chain>::get_execute(),
                                #name::<Chain>::get_instantiate(),
                                #name::<Chain>::get_query(),
                            ),
                        )),
                )
            }
        }
    );

    let new_func_name = format_ident!("get_{}", func_ident);

    let pascal_function_name = func_ident.to_string().to_case(Case::Pascal);
    let trait_name = format_ident!("{}ableContract", pascal_function_name);
    let message_name = format_ident!("{}Msg", pascal_function_name);

    let func_part = quote!(

        impl<Chain: ::boot_core::CwEnv> ::boot_core::#trait_name for #name<Chain> {
            type #message_name = #message;
        }


        impl<Chain: ::boot_core::CwEnv> #name<Chain>{
            fn #new_func_name() ->  #func_type /*(boot_func.sig.inputs) -> boot_func.sig.output*/
            {
                return #func_ident;
            }
        }
    );

    let addition: TokenStream = if func_ident == "instantiate" {
        quote!(
         #struct_def

        #func_part
        )
        .into()
    } else {
        func_part.into()
    };

    input.extend(addition);
    input
}
