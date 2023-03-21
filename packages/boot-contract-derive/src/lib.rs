#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Fields, Item, Meta, NestedMeta, Path};

#[proc_macro_attribute]
pub fn boot_contract(attrs: TokenStream, input: TokenStream) -> TokenStream {
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
            pub struct #name<Chain: ::boot_core::BootEnvironment>(::boot_core::Contract<Chain>);

            impl<Chain: ::boot_core::BootEnvironment> ::boot_core::ContractInstance<Chain> for #name<Chain> {
                fn as_instance(&self) -> &::boot_core::Contract<Chain> {
            &self.0
        }
            fn as_instance_mut(&mut self) -> &mut ::boot_core::Contract<Chain> {
                &mut self.0
            }
        }

        impl<Chain: ::boot_core::BootEnvironment> ::boot_core::CwInterface for #name<Chain> {
            type InstantiateMsg = #init;
            type ExecuteMsg = #exec;
            type QueryMsg = #query;
            type MigrateMsg = #migrate;
        }
    );
    struct_def.into()
}
