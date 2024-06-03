extern crate proc_macro;
use crate::helpers::{
    impl_into_deprecation, process_fn_name, process_sorting, to_generic_arguments,
    LexiographicMatching,
};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_quote, visit_mut::VisitMut, DeriveInput, Fields, Ident};

fn payable(v: &syn::Variant) -> bool {
    for attr in &v.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "payable" {
            return true;
        }
    }
    false
}

pub fn execute_fns_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let bname = Ident::new(&format!("{name}Fns"), name.span());

    let generics = input.generics.clone();
    let (_impl_generics, _ty_generics, where_clause) = generics.split_for_impl().clone();
    let type_generics = to_generic_arguments(&generics);

    let is_attributes_sorted = process_sorting(&input.attrs);

    let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data else {
        unimplemented!();
    };

    let variant_fns = variants.into_iter().map( |mut variant|{
        let variant_name = variant.ident.clone();

        // We rename the variant if it has a fn_name attribute associated with it
        let mut variant_func_name =
                format_ident!("{}", process_fn_name(&variant).to_case(Case::Snake));
        variant_func_name.set_span(variant_name.span());

        let is_payable = payable(&variant);

        let variant_doc: syn::Attribute = {
            let doc = format!("Automatically generated wrapper around {}::{} variant", name, variant_name);
            parse_quote!(
                #[doc=#doc]
            )
        };

        let (maybe_coins_attr, passed_coins) = if is_payable {
            (quote!(coins: &[::cosmwasm_std::Coin]),quote!(Some(coins)))
        } else {
            (quote!(),quote!(None))
        };

        match &mut variant.fields {
            Fields::Unnamed(variant_fields) => {

                let mut variant_idents = variant_fields.unnamed.clone();
                // remove any attributes for use in fn arguments
                variant_idents.iter_mut().for_each(|f: &mut syn::Field| f.attrs = vec![]);


                // We need to figure out a parameter name for all fields associated to their types
                // They will be numbered from 0 to n-1
                let variant_ident_content_names = variant_idents
                    .iter()
                    .enumerate()
                    .map(|(i, _)|  Ident::new(&format!("arg{}", i), Span::call_site()));

                let variant_attr = variant_idents.clone().into_iter()
                    .enumerate()
                    .map(|(i, mut id)| {
                    id.ident = Some(Ident::new(&format!("arg{}", i), Span::call_site()));
                    id
                });

                quote!(
                    #variant_doc
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::cw_orch::core::environment::TxResponse<Chain>, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name (
                            #(#variant_ident_content_names,)*
                        );
                        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<Chain>>::execute(self, &msg.into(),#passed_coins)
                    }
                )
            },
            Fields::Unit => {

                quote!(
                    #variant_doc
                    fn #variant_func_name(&self, #maybe_coins_attr) -> Result<::cw_orch::core::environment::TxResponse<Chain>, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name;
                        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<Chain>>::execute(self, &msg.into(),#passed_coins)
                    }
                )
            }
            Fields::Named(variant_fields) => {
                if is_attributes_sorted{
                    // sort fields on field name
                    LexiographicMatching::default().visit_fields_named_mut(variant_fields);
                }

                // parse these fields as arguments to function
                let mut variant_idents = variant_fields.named.clone();

                // remove any attributes for use in fn arguments
                variant_idents.iter_mut().for_each(|f: &mut syn::Field| f.attrs = vec![]);

                let variant_ident_content_names = variant_idents.iter().map(|f|f.ident.clone().unwrap());

                let variant_attr = variant_idents.iter();
                quote!(
                    #variant_doc
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::cw_orch::core::environment::TxResponse<Chain>, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name {
                            #(#variant_ident_content_names,)*
                        };
                        <Self as ::cw_orch::core::contract::interface_traits::CwOrchExecute<Chain>>::execute(self, &msg.into(),#passed_coins)
                    }
                )
            }
        }
    });
    let necessary_trait_where = quote!(#name<#type_generics>: Into<CwOrchExecuteMsgType>);
    let combined_trait_where_clause = where_clause
        .map(|w| {
            quote!(
                #w #necessary_trait_where
            )
        })
        .unwrap_or(quote!(
            where
                #necessary_trait_where
        ));

    let impl_into_depr = impl_into_deprecation(&input.attrs);
    let derived_trait = quote!(
        #[cfg(not(target_arch = "wasm32"))]
        #impl_into_depr
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname<Chain: ::cw_orch::core::environment::TxHandler, CwOrchExecuteMsgType, #type_generics>: ::cw_orch::core::contract::interface_traits::CwOrchExecute<Chain, ExecuteMsg = CwOrchExecuteMsgType> #combined_trait_where_clause {
            #(#variant_fns)*
        }

        #[cfg(target_arch = "wasm32")]
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname{

        }
    );

    // We need to merge the where clauses (rust doesn't support 2 wheres)
    // If there is no where clause, we simply add the necessary where
    let necessary_where = quote!(SupportedContract: ::cw_orch::core::contract::interface_traits::CwOrchExecute<Chain, ExecuteMsg = CwOrchExecuteMsgType >, #necessary_trait_where);
    let combined_where_clause = where_clause
        .map(|w| {
            quote!(
                #w #necessary_where
            )
        })
        .unwrap_or(quote!(
            where
                #necessary_where
        ));

    let derived_trait_impl = quote!(
        #[automatically_derived]
        impl<SupportedContract, Chain: ::cw_orch::core::environment::TxHandler, CwOrchExecuteMsgType, #type_generics> #bname<Chain, CwOrchExecuteMsgType, #type_generics> for SupportedContract
        #combined_where_clause {}
    );

    let expand = quote!(
        #derived_trait

        #[cfg(not(target_arch = "wasm32"))]
        #derived_trait_impl
    );

    expand.into()
}
