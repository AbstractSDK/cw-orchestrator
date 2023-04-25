#![recursion_limit = "128"]

mod boot_contract;

use crate::boot_contract::{get_func_type,get_wasm_name,get_crate_to_struct};

use convert_case::{Casing, Case};
use syn::{parse_macro_input, AttributeArgs, Fields, Item, Meta, NestedMeta, Path};
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{FnArg};

use quote::{quote, format_ident};

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

/**  
Procedural macro to generate a boot-interface contract with the kebab-case name of the crate.
Add this macro to the entry point functions of your contract to use it. 
## Example
```rust,ignore
#[cfg_attr(feature="boot", boot_contract)]
#[cfg_attr(feature="export", entry_point)]
pub fn instantiate(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: InstantiateMsg,
 -> StdResult<Response> {
    // ...
}
// ... other entrypoints (execute, query, migrate)
```
*/
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

        fn find_workspace_dir() -> ::std::path::PathBuf{
            let crate_path = env!("CARGO_MANIFEST_DIR");
            let mut current_dir = ::std::path::PathBuf::from(crate_path);
            match find_workspace_dir_worker(&mut current_dir) {
                Some(path) => path,
                None => current_dir,
            }
        }

        fn find_workspace_dir_worker(dir: &mut::std::path::PathBuf) -> Option<::std::path::PathBuf> {
            loop {
                // First we pop the dir
                if !dir.pop() {
                    return None;
                }
                let cargo_toml = dir.join("Cargo.toml");
                if ::std::fs::metadata(&cargo_toml).is_ok() {
                    return Some(dir.clone());
                }
            }
        }

        // We add the contract creation script
        impl<Chain: ::boot_core::CwEnv> #name<Chain> {
            pub fn new(contract_id: &str, chain: Chain) -> Self {

                // We get the workspace dir
                let workspace_dir = find_workspace_dir();

                // We build the artifacts from the artifacts folder (by default) of the package
                let file_path = &format!("{}/artifacts/{}.wasm", workspace_dir.display().to_string(), #wasm_name);
                //panic!("Wasm_file_path: {}", file_path);

                Self(
                    ::boot_core::Contract::new(contract_id, chain)
                )
            }
        }

        // We need to implement the Uploadable trait for both Mock and Daemon to be able to use the contract later
        impl ::boot_core::Uploadable<::boot_core::Mock> for #name<::boot_core::Mock>{
            fn source(&self) -> <::boot_core::Mock as ::boot_core::TxHandler>::ContractSource{
                // For Mock contract, we need to return a cw_multi_test Contract trait
                let contract = ::cw_multi_test::ContractWrapper::new(
                    #name::<::boot_core::Mock>::get_execute(),
                    #name::<::boot_core::Mock>::get_instantiate(),
                    #name::<::boot_core::Mock>::get_query()
                );
                Box::new(contract)
            }
        }

        impl ::boot_core::Uploadable<::boot_core::Daemon> for #name<::boot_core::Daemon>{
            fn source(&self) -> <::boot_core::Daemon as ::boot_core::TxHandler>::ContractSource{
                // For Daemon contract, we need to return a path for the artifacts to be uploaded
                // Remember that this is a helper for easy definitino of all the traits needed.
                // We just need to get the local artifacts folder at the root of the workspace
                // 1. We get the path to the local artifacts dir
                // We get the workspace dir
                let mut workspace_dir = find_workspace_dir();

                // We build the artifacts from the artifacts folder (by default) of the package
                workspace_dir.push("artifacts");
                let artifacts_dir = ::boot_core::ArtifactsDir::new(workspace_dir);
                artifacts_dir.find_wasm_path(#wasm_name).unwrap()
            }
        }



        /*
    

                        .with_wasm_path(file_path) // Adds the wasm path for uploading to a node is simple
                         .with_mock(Box::new(
                            // Adds the contract's endpoint functions for mocking
                            ::boot_core::ContractWrapper::new_with_empty(
                                #name::<Chain>::get_execute(),
                                #name::<Chain>::get_instantiate(),
                                #name::<Chain>::get_query(),
                            ),
                        )),

        */


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