#![recursion_limit = "128"]

extern crate proc_macro;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Fields, Ident};

#[proc_macro_derive(ExecuteFns,attributes(payable))]
pub fn boot_execute(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = Ident::new(&format!("{}Fns", name), name.span());

    let syn::Data::Enum(syn::DataEnum {
        variants,
        ..
    }) = ast.data
     else {
        unimplemented!();
    };

    let variant_fns = variants.iter().filter_map( |variant|{
        let variant_name = variant.ident.clone();
        let is_payable = payable(variant);
        match &variant.fields {
            Fields::Unnamed(_) => None,
            Fields::Unit => None,
            Fields::Named(variant_fields) => {

                // parse these fields as arguments to function
                let mut variant_func_name =
                format_ident!("{}", variant_name.to_string().to_case(Case::Snake));
                variant_func_name.set_span(variant_name.span());

                let variant_idents = variant_fields.named.clone();
                let variant_ident_content_names = variant_idents.iter().map(|f|f.ident.clone().unwrap());

                let (maybe_coins_attr, passed_coins) = if is_payable {
                    (quote!(coins: &[::cosmwasm_std::Coin]),quote!(Some(coins)))
                }else { 
                    (quote!(),quote!(None))
                };
                Some(if variant_idents.is_empty() {
                    quote!(
                        #[allow(clippy::too_many_arguments)]
                        fn #variant_func_name(&self, #maybe_coins_attr) -> Result<::boot_core::TxResponse<Chain>, ::boot_core::BootError> {
                            let msg = #name::#variant_name {};
                            self.execute(&msg,#passed_coins)
                        }
                    )
                } else {
                    quote!(
                        #[allow(clippy::too_many_arguments)]
                        fn #variant_func_name(&self, #variant_idents, #maybe_coins_attr) -> Result<::boot_core::TxResponse<Chain>, ::boot_core::BootError> {
                            let msg = #name::#variant_name {
                                #(#variant_ident_content_names,)*
                            };
                            self.execute(&msg,#passed_coins)
                        }
                    )
                })
            },
        }
    }
);

    let derived_trait = quote!(
        pub trait #bname<Chain: ::boot_core::BootEnvironment>: ::boot_core::prelude::BootExecute<Chain, ExecuteMsg = #name> {
            #(#variant_fns)*
        }
    );

    let derived_trait_impl = quote!(
        impl<T, Chain: ::boot_core::BootEnvironment> #bname<Chain> for T
        where
            T: ::boot_core::prelude::BootExecute<Chain, ExecuteMsg = #name>{}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );
    expand.into()
}


fn payable(v: &syn::Variant) -> bool {
    for attr in &v.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "payable" {
            return true
        }
    }
    false
}