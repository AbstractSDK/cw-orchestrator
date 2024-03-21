#![recursion_limit = "128"]

use syn::{Expr, Token};
use syn::{__private::TokenStream2, parse_macro_input, Fields, GenericArgument, Item, Path};
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use syn::{punctuated::Punctuated, token::Comma};

use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(id);
}
// This is used to parse the types into a list of types separated by Commas
// and default contract id if provided by "id = $expr"
struct InterfaceInput {
    expressions: Punctuated<Path, Comma>,
    _kw_id: Option<kw::id>,
    _eq_token: Option<Token![=]>,
    default_id: Option<Expr>,
}

// Implement the `Parse` trait for your input struct
impl Parse for InterfaceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut expressions: Punctuated<Path, Comma> = Punctuated::new();

        while let Ok(path) = input.parse() {
            expressions.push(path);
            let _: Option<Token![,]> = input.parse().ok();

            // If we found id = break
            if input.peek(kw::id) {
                break;
            }
        }
        // Parse if there is any
        let kw_id: Option<kw::id> = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "The 5th argument of the macro should be of the format `id=my_contract_id`",
            )
        })?;
        let eq_token: Option<Token![=]> = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "The 5th argument of the macro should be of the format `id=my_contract_id`",
            )
        })?;
        let default_id: Option<Expr> = input.parse().ok();
        Ok(Self {
            expressions,
            _kw_id: kw_id,
            _eq_token: eq_token,
            default_id,
        })
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
pub struct Cw20<Chain: ::cw_orch::prelude::CwEnv>(::cw_orch::prelude::Contract<Chain>);

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

impl <Chain> Uploadable for Cw20<Chain> {
    fn wrapper(&self) -> <Mock as cw_orch::TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }

    fn wasm(&self) -> <Daemon as cw_orch::TxHandler>::ContractSource {
        WasmPath::new("path/to/cw20.wasm").unwrap()
    }
}
*/
#[proc_macro_attribute]
pub fn interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as syn::Item);

    // Try to parse the attributes to a
    let attributes = parse_macro_input!(attrs as InterfaceInput);

    let types_in_order = attributes.expressions;
    let default_id = attributes.default_id;

    if types_in_order.len() != 4 {
        panic!("Expected four endpoint types (InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg). Use cosmwasm_std::Empty if not implemented.")
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

    let all_phantom_marker_values: Vec<TokenStream2> = all_generics
        .iter()
        .map(|_| quote!(::std::marker::PhantomData::default()))
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
    let default_num = if let Some(id_expr) = default_id {
        quote!(
            impl <Chain, #all_generics> #name<Chain, #all_generics> {
                pub fn new(chain: Chain) -> Self {
                    Self(
                        ::cw_orch::contract::Contract::new(#id_expr, chain)
                    , #(#all_phantom_marker_values,)*)
                }
            }
        )
    } else {
        quote!(
            impl <Chain, #all_generics> #name<Chain, #all_generics> {
                pub fn new(contract_id: impl ToString, chain: Chain) -> Self {
                    Self(
                        ::cw_orch::contract::Contract::new(contract_id, chain)
                    , #(#all_phantom_marker_values,)*)
                }
            }
        )
    };
    let struct_def = quote!(
            #[cfg(not(target_arch = "wasm32"))]
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name<Chain, #all_generics>(::cw_orch::contract::Contract<Chain>, #(#all_phantom_markers,)*);

            #[cfg(target_arch = "wasm32")]
            #[derive(
                ::std::clone::Clone,
            )]
            pub struct #name;
      
            #[cfg(not(target_arch = "wasm32"))]
            #default_num

            #[cfg(not(target_arch = "wasm32"))]
            impl<Chain: ::cw_orch::environment::ChainState, #all_generics> ::cw_orch::prelude::ContractInstance<Chain> for #name<Chain, #all_generics> {
                fn as_instance(&self) -> &::cw_orch::contract::Contract<Chain> {
                    &self.0
                }
                fn as_instance_mut(&mut self) -> &mut ::cw_orch::contract::Contract<Chain> {
                    &mut self.0
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        impl<Chain: ::cw_orch::environment::QueryHandler + ::cw_orch::environment::ChainState, #all_generics> ::cw_orch::prelude::QueryableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type QueryMsg = #query;
        }
        #[cfg(not(target_arch = "wasm32"))]
        impl<Chain: ::cw_orch::prelude::TxHandler, #all_generics> ::cw_orch::prelude::InstantiableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type InstantiateMsg = #init;
        }
        #[cfg(not(target_arch = "wasm32"))]
        impl<Chain: ::cw_orch::prelude::TxHandler, #all_generics> ::cw_orch::prelude::ExecutableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type ExecuteMsg = #exec;
        }
        #[cfg(not(target_arch = "wasm32"))]
        impl<Chain: ::cw_orch::prelude::TxHandler, #all_generics> ::cw_orch::prelude::MigratableContract for #name<Chain, #all_generics> #all_debug_serialize {
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}
