#![recursion_limit = "128"]

mod boot_contract;
use syn::{
    parse_macro_input, AttributeArgs, Fields, Item, Meta, NestedMeta, Path,
};
extern crate proc_macro;
use proc_macro::TokenStream;

use syn::__private::TokenStream2;

use quote::quote;

 #[cfg(feature="outside_contract")]
use boot_contract::boot_contract_raw;

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


#[proc_macro_attribute]
pub fn boot_contract(attrs: TokenStream, input: TokenStream) -> TokenStream {
    
    // The boot macro part
    let mut new_input: TokenStream2;
    #[cfg(feature="outside_contract")] {
       new_input = boot_contract_raw(attrs,input).into();
    }
    #[cfg(not(feature="outside_contract"))] {
        new_input = input.into();
    }
    

    // The cosmwasm_std::entry_point part
   new_input = quote!(
        #[cfg_attr(feature="library",::cosmwasm_std::entry_point)]
        #new_input
    );

    new_input.into()
}
