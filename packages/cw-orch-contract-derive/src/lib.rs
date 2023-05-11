#![recursion_limit = "128"]

mod cw_orch_contract;

use crate::cw_orch_contract::{get_crate_to_struct, get_func_type, get_wasm_name};

use convert_case::{Case, Casing};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Fields, FnArg, GenericArgument, Item, Path};
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
// Gets the generics associated with a type
fn get_generics_from_path(p: &Path) -> Punctuated<GenericArgument, Comma> {
    let mut generics = Punctuated::new();

    for segment in p.segments.clone() {
        if let syn::PathArguments::AngleBracketed(generic_args) = &segment.arguments {
            for arg in generic_args.args.clone() {
                generics.push(arg);
            }
        }
    }

    generics
}

/**
Procedural macro to generate a cw-orchestrator interface

## Example

```ignore
#[interface(
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
pub struct Cw20<Chain: ::cw_orch::prelude::CwEnv>(::cw_orch::contract::Contract<Chain>);

impl <Chain: ::cw_orch::prelude::CwEnv> Cw20<Chain> {
    /// Constructor for the contract interface
     pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
        Self(
            ::cw_orch::contract::Contract::new(contract_id, chain)
        )
    }
}

// Traits for signaling cw-orchestrator with what messages to call the contract's entry points.
impl <Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::InstantiableContract for Cw20<Chain> {
    type InstantiateMsg = InstantiateMsg;
}
impl <Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::ExecutableContract for Cw20<Chain> {
    type ExecuteMsg = ExecuteMsg;
}
// ... other entry point & upload traits
```

## Linking the interface to its source code

The interface can be linked to its source code by implementing the `Uploadable` trait for the interface.

```ignore
use cw_orch::prelude::*;

impl Uploadable<Mock> for Cw20<Mock> {
    fn source(&self) -> <Mock as cw_orch::TxHandler>::ContractSource {
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
    fn source(&self) -> <Daemon as cw_orch::TxHandler>::ContractSource {
        WasmPath::new("path/to/cw20.wasm").unwrap()
    }
}

*/
#[proc_macro_attribute]
pub fn interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as syn::Item);

    // Try to parse the attributes to a
    let attributes = parse_macro_input!(attrs as TypesInput);

    let types_in_order = attributes.expressions;

    if types_in_order.len() != 4 {
        panic!("Expected four endpoint types (InstantiateMsg,ExecuteMsg,QueryMsg,MigrateMsg). Use cosmwasm_std::Empty if not implemented.")
    }

    let Item::Struct(cw_orch_struct) = &mut item else {
        panic!("Only works on structs");
    };
    let Fields::Unit = &mut cw_orch_struct.fields else {
        panic!("Struct must be unit-struct");
    };

    let init = types_in_order[0].clone();
    let exec = types_in_order[1].clone();
    let query = types_in_order[2].clone();
    let migrate = types_in_order[3].clone();

    // We create all generics for all types
    let all_generics: Punctuated<GenericArgument, Comma> = types_in_order
        .iter()
        .flat_map(get_generics_from_path)
        .collect();
    // We create all phantom markers because else types are unused
    let all_phantom_markers: Vec<TokenStream2> = all_generics
        .iter()
        .map(|t| {
            quote!(
                ::std::marker::PhantomData<#t>
            )
        })
        .collect();
    // We create necessary Debug + Serialize traits
    let all_debug_serialize: Vec<TokenStream2> = all_generics
        .iter()
        .map(|t| {
            quote!(
                #t: ::std::fmt::Debug + ::serde::Serialize
            )
        })
        .collect();
    let all_debug_serialize = if !all_debug_serialize.is_empty() {
        quote!(where #(#all_debug_serialize,)*)
    } else {
        quote!()
    };

    let name = cw_orch_struct.ident.clone();
    let struct_def = quote!(
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain: ::cw_orch::prelude::CwEnv, #all_generics>(::cw_orch::contract::Contract<Chain>, #(#all_phantom_markers,)*);

            impl<Chain: ::cw_orch::prelude::CwEnv, #all_generics> ::cw_orch::prelude::ContractInstance<Chain> for #name<Chain, #all_generics> {
                fn as_instance(&self) -> &::cw_orch::contract::Contract<Chain> {
                &self.0
            }
            fn as_instance_mut(&mut self) -> &mut ::cw_orch::contract::Contract<Chain> {
                &mut self.0
            }
        }

        impl<Chain: ::cw_orch::prelude::CwEnv, #all_generics> ::cw_orch::prelude::InstantiableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type InstantiateMsg = #init;
        }

        impl<Chain: ::cw_orch::prelude::CwEnv, #all_generics> ::cw_orch::prelude::ExecutableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type ExecuteMsg = #exec;
        }

        impl<Chain: ::cw_orch::prelude::CwEnv, #all_generics> ::cw_orch::prelude::QueryableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type QueryMsg = #query;
        }

        impl<Chain: ::cw_orch::prelude::CwEnv, #all_generics> ::cw_orch::prelude::MigratableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}

/**
Procedural macro to generate a cw-orchestrator interface with the kebab-case name of the crate.
Add this macro to the entry point functions of your contract to use it.
## Example
```text
// In crate "my-contract"
#[cfg_attr(feature="interface", interface_entry_point)]
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
pub struct MyContract<Chain: ::cw_orch::prelude::CwEnv>(::cw_orch::contract::Contract<Chain>);

impl <Chain: ::cw_orch::prelude::CwEnv> MyContract<Chain> {
    /// Constructor for the contract interface
     pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
        Self(
            ::cw_orch::contract::Contract::new(contract_id, chain)
        )
    }
}

// Traits for signaling cw-orchestrator with what messages to call the contract's entry points.
impl <Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::InstantiableContract for MyContract<Chain> {
    type InstantiateMsg = InstantiateMsg;
}
impl <Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::ExecutableContract for MyContract<Chain> {
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
pub fn interface_entry_point(_attrs: TokenStream, mut input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let mut item = parse_macro_input!(cloned as syn::Item);

    let Item::Fn(cw_orch_func) = &mut item else {
        panic!("Only works on functions");
    };

    // Now we get the fourth function argument that should be the instantiate message
    let signature = &mut cw_orch_func.sig;
    let func_ident = signature.ident.clone();
    let func_type = get_func_type(signature);

    let wasm_name = get_wasm_name();
    let name = get_crate_to_struct();

    let struct_def = quote!(
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain: ::cw_orch::prelude::CwEnv>(::cw_orch::contract::Contract<Chain>);

            impl<Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::cw_orch::contract::Contract<Chain> {
            &self.0
        }
            fn as_instance_mut(&mut self) -> &mut ::cw_orch::contract::Contract<Chain> {
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

        // We need to create default reply, sudo and migrate getter functions because those functions may not be implemented by the contract
        // These are fallback in case the functions are not defined at a later time
        type ReplyFn<C, E, Q> = fn(deps: ::cosmwasm_std::DepsMut<Q>, env: ::cosmwasm_std::Env, msg: ::cosmwasm_std::Reply) -> Result<::cosmwasm_std::Response<C>, E>;
        type PermissionedFn<T, C, E, Q> = fn(deps: ::cosmwasm_std::DepsMut<Q>, env: ::cosmwasm_std::Env, msg: T) -> Result<::cosmwasm_std::Response<C>, E>; // For SUDO

        pub trait DefaultReply<C,  Q: ::cosmwasm_std::CustomQuery, E5A> {
            fn get_reply() -> Option<ReplyFn<C, E5A , Q>> {
                None
            }
        }
        pub trait DefaultSudo<C, Q: ::cosmwasm_std::CustomQuery, T4A, E4A> {
            fn get_sudo() -> Option<PermissionedFn<T4A, C, E4A, Q>,> {
                None
            }
        }
        pub trait DefaultMigrate<C, Q: ::cosmwasm_std::CustomQuery, E6A, T6A > {
            fn get_migrate() -> Option<PermissionedFn<T6A, C, E6A, Q>> {
                None
            }
        }
        impl<Chain: ::cw_orch::prelude::CwEnv, C, Q: ::cosmwasm_std::CustomQuery> DefaultMigrate<C, Q, ::cosmwasm_std::StdError, ::cosmwasm_std::Empty> for #name<Chain> {}
        impl<Chain: ::cw_orch::prelude::CwEnv, C,  Q: ::cosmwasm_std::CustomQuery> DefaultReply<C,  Q, ::cosmwasm_std::StdError> for #name<Chain> {}
        impl<Chain: ::cw_orch::prelude::CwEnv, C, Q: ::cosmwasm_std::CustomQuery> DefaultSudo<C, Q, ::cosmwasm_std::Empty, ::cosmwasm_std::StdError> for #name<Chain> {}

        // We add the contract creation script
        impl<Chain: ::cw_orch::prelude::CwEnv> #name<Chain> {
            pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
                Self(
                    ::cw_orch::contract::Contract::new(contract_id, chain)
                )
            }
        }

        // We implement the Contract trait directly for our structure
        impl ::cw_orch::prelude::MockContract<::cosmwasm_std::Empty, ::cosmwasm_std::Empty> for #name<::cw_orch::prelude::Mock>{
            fn execute(&self, deps: ::cosmwasm_std::DepsMut, env: ::cosmwasm_std::Env, info: ::cosmwasm_std::MessageInfo, msg: std::vec::Vec<u8>) -> std::result::Result<::cosmwasm_std::Response<::cosmwasm_std::Empty>, ::cw_orch::anyhow::Error> {
                let msg = ::cosmwasm_std::from_slice(&msg)?;
                #name::<::cw_orch::prelude::Mock>::get_execute()(deps, env, info, msg).map_err(|err| ::cw_orch::anyhow::anyhow!(err))
            }
            fn instantiate(&self, deps: ::cosmwasm_std::DepsMut, env: ::cosmwasm_std::Env, info: ::cosmwasm_std::MessageInfo, msg: std::vec::Vec<u8>) -> std::result::Result<::cosmwasm_std::Response<::cosmwasm_std::Empty>, ::cw_orch::anyhow::Error> {
                let msg = ::cosmwasm_std::from_slice(&msg)?;
                #name::<::cw_orch::prelude::Mock>::get_instantiate()(deps, env, info, msg).map_err(|err| ::cw_orch::anyhow::anyhow!(err))
            }
            fn query(&self, deps: ::cosmwasm_std::Deps, env: ::cosmwasm_std::Env, msg: std::vec::Vec<u8>) -> std::result::Result<::cosmwasm_std::Binary, ::cw_orch::anyhow::Error> {
                let msg = ::cosmwasm_std::from_slice(&msg)?;
                #name::<::cw_orch::prelude::Mock>::get_query()(deps, env, msg).map_err(|err| ::cw_orch::anyhow::anyhow!(err))
            }
            fn sudo(&self, deps: ::cosmwasm_std::DepsMut, env: ::cosmwasm_std::Env, msg: std::vec::Vec<u8>) -> std::result::Result<::cosmwasm_std::Response<::cosmwasm_std::Empty>, ::anyhow::Error> {
                if let Some(sudo) = #name::<::cw_orch::prelude::Mock>::get_sudo() {
                    let msg = ::cosmwasm_std::from_slice(&msg)?;
                    sudo(deps, env, msg).map_err(|err| ::anyhow::anyhow!(err))
                }else{
                    panic!("No sudo registered");
                }
            }
            fn reply(&self, deps: ::cosmwasm_std::DepsMut, env: ::cosmwasm_std::Env, reply_msg: ::cosmwasm_std::Reply) -> std::result::Result<::cosmwasm_std::Response<::cosmwasm_std::Empty>, ::cw_orch::anyhow::Error> {
                if let Some(reply) = #name::<::cw_orch::prelude::Mock>::get_reply() {
                    reply(deps, env, reply_msg).map_err(|err| ::cw_orch::anyhow::anyhow!(err))
                }else{
                    panic!("No reply registered");
                }
            }
            fn migrate(&self, deps: cosmwasm_std::DepsMut, env: cosmwasm_std::Env, msg: std::vec::Vec<u8>) -> std::result::Result<cosmwasm_std::Response<::cosmwasm_std::Empty>, ::cw_orch::anyhow::Error> {
                if let Some(migrate) = #name::<::cw_orch::prelude::Mock>::get_migrate() {
                    let msg = ::cosmwasm_std::from_slice(&msg)?;
                    migrate(deps, env, msg).map_err(|err| ::cw_orch::anyhow::anyhow!(err))
                }else{
                    panic!("No migrate registered");
                }
            }
        }

        // We need to implement the Uploadable trait for both Mock and Daemon to be able to use the contract later
        impl <Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::Uploadable for #name<Chain>{
            fn wrapper(&self) -> Box<dyn ::cw_orch::prelude::ContractWrapper>{
                // For Mock contract, we need to return a cw_multi_test Contract trait
                Box::new(self.clone())
            }

            fn wasm(&self) -> ::cw_orch::prelude::WasmPath {
                // For Daemon contract, we need to return a path for the artifacts to be uploaded
                // Remember that this is a helper for easy definition of all the traits needed.
                // We just need to get the local artifacts folder at the root of the workspace
                // 1. We get the path to the local artifacts dir
                // We get the workspace dir
                let mut workspace_dir = find_workspace_dir();

                // We build the artifacts from the artifacts folder (by default) of the package
                workspace_dir.push("artifacts");
                let artifacts_dir = ::cw_orch::prelude::ArtifactsDir::new(workspace_dir);
                artifacts_dir.find_wasm_path(#wasm_name).unwrap()
            }
        }
    );

    // if daemon is enabled on cw-orc it will implement Uploadable<Daemon>

    #[cfg(feature = "propagate_daemon")]
    let daemon_uploadable: TokenStream = quote!(
            impl ::cw_orch::prelude::Uploadable<::cw_orch::prelude::Daemon> for #name<::cw_orch::prelude::Daemon>{

        }
    )
    .into();

    let new_func_name = format_ident!("get_{}", func_ident);

    let pascal_function_name = func_ident.to_string().to_case(Case::Pascal);
    let trait_name = format_ident!("{}ableContract", pascal_function_name);
    let message_name = format_ident!("{}Msg", pascal_function_name);

    let func_name: String = func_ident.to_string();

    let message_part = match func_name.as_str() {
        "instantiate" | "execute" | "query" | "migrate" => {
            // If we have the instantiate / execute / query / migrate, we define user-messages
            let message_idx = match func_name.as_str() {
                "instantiate" | "execute" => 3,
                "query" | "migrate" => 2,
                _ => panic!("Unreachable"),
            };
            let message = match signature.inputs[message_idx].clone() {
                FnArg::Typed(syn::PatType { ty, .. }) => *ty,
                _ => panic!("Only typed arguments"),
            };
            quote!(
                impl<Chain: ::cw_orch::prelude::CwEnv> ::cw_orch::prelude::#trait_name for #name<Chain> {
                    type #message_name = #message;
                }
            )
        }
        // in the next 2 cases case we implement the optional Sudo or Reply traits on the contract, to signal the function exists
        "sudo" => {
            quote!()
        }
        "reply" => {
            quote!()
        }
        _ => panic!("Macro not supported for this funciton name"),
    };

    let func_part = match func_name.as_str() {
        "instantiate" | "execute" | "query" => {
            quote!(
                impl<Chain: ::cw_orch::prelude::CwEnv> #name<Chain>{
                    fn #new_func_name() ->  #func_type /*(cw_orch_func.sig.inputs) -> cw_orch_func.sig.output*/
                    {
                        #func_ident
                    }
                }
            )
        }
        "migrate" | "sudo" | "reply" => {
            quote!(
                impl<Chain: ::cw_orch::prelude::CwEnv> #name<Chain>{
                    fn #new_func_name() -> Option<#func_type> /*(cw_orch_func.sig.inputs) -> cw_orch_func.sig.output*/
                    {
                        Some(#func_ident)
                    }
                }
            )
        }
        _ => panic!("Unreachable"),
    };

    let addition: TokenStream = if func_ident == "instantiate" {
        #[allow(unused_mut)]
        let mut interface_def: TokenStream = quote!(
            #struct_def
            #message_part
            #func_part
        )
        .into();
        // Add the Uploadable<Daemon> trait for the contract
        #[cfg(feature = "propagate_daemon")]
        interface_def.extend(daemon_uploadable);

        interface_def
    } else {
        quote!(
            #message_part
            #func_part
        )
        .into()
    };

    input.extend(addition);
    input
}
