#![recursion_limit = "128"]

mod cw_orc_contract;

use crate::cw_orc_contract::{get_crate_to_struct, get_func_type, get_wasm_name};

use convert_case::{Case, Casing};
use syn::{parse_macro_input, Fields, FnArg, Item, Path};
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote};

use syn::punctuated::Punctuated;
use syn::token::Comma;

use syn::parse::{Parse, ParseStream};

// This is used to parse the types into a list of types separated by Commas
struct TypesInput {
    expressions: Punctuated<Path, Comma>,
}

// Implement the `Parse` trait for your input struct
impl Parse for TypesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expressions = input.parse_terminated(Path::parse)?;
        Ok(Self { expressions })
    }
}

/**
Procedural macro to generate a cw-orchestrator interface

## Example

```ignore
#[contract(
    cw20_base::msg::InstantiateMsg,
    cw20_base::msg::ExecuteMsg,
    cw20_base::msg::QueryMsg,
    cw20_base::msg::MigrateMsg
)]
pub struct Cw20;
```
This generated the following code:

```ignore

// This struct represents the interface to the contract.
pub struct Cw20<Chain: ::cw_orc::CwEnv>(::cw_orc::Contract<Chain>);

impl <Chain: ::cw_orc::CwEnv> Cw20<Chain> {
    /// Constructor for the contract interface
     pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
        Self(
            ::cw_orc::Contract::new(contract_id, chain)
        )
    }
}

// Traits for signaling cw-orchestrator with what messages to call the contract's entry points.
impl <Chain: ::cw_orc::CwEnv> ::cw_orc::InstantiateableContract for Cw20<Chain> {
    type InstantiateMsg = InstantiateMsg;
}
impl <Chain: ::cw_orc::CwEnv> ::cw_orc::ExecuteableContract for Cw20<Chain> {
    type ExecuteMsg = ExecuteMsg;
}
// ... other entry point & upload traits
```

## Linking the interface to its source code

The interface can be linked to its source code by implementing the `Uploadable` trait for the interface.

```ignore
use cw_orc::{
    Mock, Daemon, Uploadable, WasmPath, ContractWrapper,
}

impl Uploadable<Mock> for Cw20<Mock> {
    fn source(&self) -> <Mock as cw_orc::TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }
}

impl Uploadable<Daemon> for Cw20<Daemon> {
    fn source(&self) -> <Daemon as cw_orc::TxHandler>::ContractSource {
        WasmPath::new("path/to/cw20.wasm").unwrap()
    }
}

*/
#[proc_macro_attribute]
pub fn contract(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as syn::Item);

    // Try to parse the attributes to a
    let attributes = parse_macro_input!(attrs as TypesInput);

    let types_in_order = attributes.expressions;

    if types_in_order.len() != 4 {
        panic!("Expected four endpoint types (InstantiateMsg,ExecuteMsg,QueryMsg,MigrateMsg). Use cosmwasm_std::Empty if not implemented.")
    }

    let Item::Struct(cw_orc_struct) = &mut item else {
        panic!("Only works on structs");
    };
    let Fields::Unit = &mut cw_orc_struct.fields else {
        panic!("Struct must be unit-struct");
    };

    let init = types_in_order[0].clone();
    let exec = types_in_order[1].clone();
    let query = types_in_order[2].clone();
    let migrate = types_in_order[3].clone();

    let name = cw_orc_struct.ident.clone();
    let struct_def = quote!(
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain: ::cw_orc::CwEnv>(::cw_orc::Contract<Chain>);

            impl<Chain: ::cw_orc::CwEnv> ::cw_orc::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::cw_orc::Contract<Chain> {
                &self.0
            }
            fn as_instance_mut(&mut self) -> &mut ::cw_orc::Contract<Chain> {
                &mut self.0
            }
        }

        impl<Chain: ::cw_orc::CwEnv> ::cw_orc::InstantiateableContract for #name<Chain> {
            type InstantiateMsg = #init;
        }

        impl<Chain: ::cw_orc::CwEnv> ::cw_orc::ExecuteableContract for #name<Chain> {
            type ExecuteMsg = #exec;
        }

        impl<Chain: ::cw_orc::CwEnv> ::cw_orc::QueryableContract for #name<Chain> {
            type QueryMsg = #query;
        }

        impl<Chain: ::cw_orc::CwEnv> ::cw_orc::MigrateableContract for #name<Chain> {
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}

/**
Procedural macro to generate a cw-orchestrator interface with the kebab-case name of the crate.
Add this macro to the entry point functions of your contract to use it.
## Example
```ignore,ignore
// In crate "my-contract"
#[cfg_attr(feature="interface", interface)]
#[cfg_attr(feature="export", entry_point)]
pub fn instantiate(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: InstantiateMsg,
 -> StdResult<Response> {
    // ...
}
// ... other entry points (execute, query, migrate)
```
### Generated code

```ignore,ignore
// This struct represents the interface to the contract.
pub struct MyContract<Chain: ::cw_orc::CwEnv>(::cw_orc::Contract<Chain>);

impl <Chain: ::cw_orc::CwEnv> MyContract<Chain> {
    /// Constructor for the contract interface
     pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
        Self(
            ::cw_orc::Contract::new(contract_id, chain)
        )
    }
}

// Traits for signaling cw-orchestrator with what messages to call the contract's entry points.
impl <Chain: ::cw_orc::CwEnv> ::cw_orc::InstantiateableContract for MyContract<Chain> {
    type InstantiateMsg = InstantiateMsg;
}
impl <Chain: ::cw_orc::CwEnv> ::cw_orc::ExecuteableContract for MyContract<Chain> {
    type ExecuteMsg = ExecuteMsg;
}
// ... other entry point & upload traits
```

### Interface usage

Now you can use the generated interface to call the contract's entry points in your tests/scripts.

```ignore,ignore
use my_contract::contract::MyContract;

pub fn my_script() {
    let sender = "my_address";
    let mock_chain = Mock::new(sender);
    // create a new interface for the contract
    let my_contract = MyContract::new("my_contract", mock_chain.clone());
    // upload it
    my_contract.upload().unwrap();

    // Instantiate the contract
    let instantiate_msg = InstantiateMsg {
        // ...
    };
    my_contract.instantiate(instantiate_msg, None, &[]).unwrap();

    // Execute, Query, Migrate, etc.
    //...
}
```
*/
#[proc_macro_attribute]
pub fn interface(_attrs: TokenStream, mut input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let mut item = parse_macro_input!(cloned as syn::Item);

    let Item::Fn(cw_orc_func) = &mut item else {
        panic!("Only works on functions");
    };

    // Now we get the fourth function argument that should be the instantiate message
    let signature = &mut cw_orc_func.sig;
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
            pub struct #name<Chain: ::cw_orc::CwEnv>(::cw_orc::Contract<Chain>);

            impl<Chain: ::cw_orc::CwEnv> ::cw_orc::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::cw_orc::Contract<Chain> {
            &self.0
        }
            fn as_instance_mut(&mut self) -> &mut ::cw_orc::Contract<Chain> {
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
        impl<Chain: ::cw_orc::CwEnv> #name<Chain> {
            pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
                Self(
                    ::cw_orc::Contract::new(contract_id, chain)
                )
            }
        }

        // We need to implement the Uploadable trait for both Mock and Daemon to be able to use the contract later
        impl ::cw_orc::Uploadable<::cw_orc::Mock> for #name<::cw_orc::Mock>{
            fn source(&self) -> <::cw_orc::Mock as ::cw_orc::TxHandler>::ContractSource{
                // For Mock contract, we need to return a cw_multi_test Contract trait
                let contract = ::cw_orc::ContractWrapper::new(
                    #name::<::cw_orc::Mock>::get_execute(),
                    #name::<::cw_orc::Mock>::get_instantiate(),
                    #name::<::cw_orc::Mock>::get_query()
                );
                Box::new(contract)
            }
        }

        impl ::cw_orc::Uploadable<::cw_orc::Daemon> for #name<::cw_orc::Daemon>{
            fn source(&self) -> <::cw_orc::Daemon as ::cw_orc::TxHandler>::ContractSource{
                // For Daemon contract, we need to return a path for the artifacts to be uploaded
                // Remember that this is a helper for easy definition of all the traits needed.
                // We just need to get the local artifacts folder at the root of the workspace
                // 1. We get the path to the local artifacts dir
                // We get the workspace dir
                let mut workspace_dir = find_workspace_dir();

                // We build the artifacts from the artifacts folder (by default) of the package
                workspace_dir.push("artifacts");
                let artifacts_dir = ::cw_orc::ArtifactsDir::new(workspace_dir);
                artifacts_dir.find_wasm_path(#wasm_name).unwrap()
            }
        }



        /*


                        .with_wasm_path(file_path) // Adds the wasm path for uploading to a node is simple
                         .with_mock(Box::new(
                            // Adds the contract's endpoint functions for mocking
                            ::cw_orc::ContractWrapper::new_with_empty(
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

        impl<Chain: ::cw_orc::CwEnv> ::cw_orc::#trait_name for #name<Chain> {
            type #message_name = #message;
        }


        impl<Chain: ::cw_orc::CwEnv> #name<Chain>{
            fn #new_func_name() ->  #func_type /*(cw_orc_func.sig.inputs) -> cw_orc_func.sig.output*/
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
