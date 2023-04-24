#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use syn::token::Comma;
use syn::punctuated::Punctuated;

use syn::parse::{Parse,ParseStream};
use syn::{parse_macro_input, Fields, Item, Path};

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

#[proc_macro_attribute]
pub fn contract(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as syn::Item);
    
    // Try to parse the attributes to a 
    let attributes = parse_macro_input!(attrs as TypesInput);

    let types_in_order = attributes.expressions;

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

        impl<Chain: ::boot_core::CwEnv> ::boot_core::CwInterface for #name<Chain> {
            type InstantiateMsg = #init;
            type ExecuteMsg = #exec;
            type QueryMsg = #query;
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}
