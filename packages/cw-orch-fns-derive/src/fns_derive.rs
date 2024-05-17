extern crate proc_macro;
use crate::{
    execute_fns::payable,
    helpers::{
        process_fn_name, process_sorting, to_generic_arguments, LexiographicMatching, MsgType,
    },
    query_fns::parse_query_type,
};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_quote, visit_mut::VisitMut, Fields, Ident, ItemEnum};

pub fn fns_derive(msg_type: MsgType, input: ItemEnum) -> TokenStream {
    let name = &input.ident;
    let bname = Ident::new(&format!("{name}Fns"), name.span());

    let generics = input.generics.clone();
    let (_impl_generics, _ty_generics, where_clause) = generics.split_for_impl().clone();
    let type_generics = to_generic_arguments(&generics);

    let is_attributes_sorted = process_sorting(&input.attrs);

    let variants = input.variants;

    let (trait_name, func_name, trait_msg_type, generic_msg_type, chain_trait) = match msg_type {
        MsgType::Execute => (
            quote!(CwOrchExecute),
            quote!(execute),
            quote!(ExecuteMsg),
            quote!(CwOrchExecuteMsgType),
            quote!(::cw_orch::core::environment::TxHandler),
        ),
        MsgType::Query => (
            quote!(CwOrchQuery),
            quote!(query),
            quote!(QueryMsg),
            quote!(CwOrchQueryMsgType),
            quote!(
                ::cw_orch::core::environment::QueryHandler
                    + ::cw_orch::core::environment::ChainState
            ),
        ),
    };

    let variant_fns = variants.into_iter().map( |mut variant|{
        let variant_name = variant.ident.clone();

        // We rename the variant if it has a fn_name attribute associated with it
        let mut variant_func_name =
                format_ident!("{}", process_fn_name(&variant).to_case(Case::Snake));
        variant_func_name.set_span(variant_name.span());


        let variant_doc: syn::Attribute = {
            let doc = format!("Automatically generated wrapper around {}::{} variant", name, variant_name);
            parse_quote!(
                #[doc=#doc]
            )
        };

        // TODO
        // Execute Specific
        let (maybe_coins_attr,passed_coins) = match msg_type{
            MsgType::Execute => {
                let is_payable = payable(&variant);
                if is_payable {
                    (quote!(coins: &[::cosmwasm_std::Coin]),quote!(Some(coins)))
                } else {
                    (quote!(),quote!(None))
                }
            }
            MsgType::Query => {
                (quote!(), quote!())
            }
        };


        let response = match msg_type{
            MsgType::Execute => quote!(::cw_orch::core::environment::TxResponse<Chain>),
            MsgType::Query => parse_query_type(&variant)
        };

        match &mut variant.fields {
            Fields::Unnamed(variant_fields) => {
                let mut variant_idents = variant_fields.unnamed.clone();

                // remove any attributes for use in fn arguments
                variant_idents.iter_mut().for_each(|f| f.attrs = vec![]);

                // We need to figure out a parameter name for all fields associated to their types
                // They will be numbered from 0 to n-1
                
                let variant_fields: Vec<_> = variant_idents.clone().into_iter()
                    .enumerate()
                    .map(|(i, mut id)| {
                    id.ident = Some(Ident::new(&format!("arg{}", i), Span::call_site()));
                    id
                }).collect();

                // Generate the struct members (This can be kept, it doesn't disturb)
                let variant_ident_content_names = variant_fields
                    .iter()
                    .map(|id| {
                        let ident = &id.ident;
                        quote!(#ident.into())
                    });

                // Generate the function arguments (This may be made optional)
                let variant_params = variant_fields.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    quote! (#field_name: impl Into<#field_type> )
                });


                quote!(
                    #variant_doc
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_params,)* #maybe_coins_attr) -> Result<#response, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name (
                            #(#variant_ident_content_names,)*
                        );
                        <Self as ::cw_orch::core::contract::interface_traits::#trait_name<Chain>>::#func_name(self, &msg.into(),#passed_coins)
                    }
                )
            },
            Fields::Unit => {

                quote!(
                    #variant_doc
                    fn #variant_func_name(&self, #maybe_coins_attr) -> Result<#response, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name;
                        <Self as ::cw_orch::core::contract::interface_traits::#trait_name<Chain>>::#func_name(self, &msg.into(),#passed_coins)
                    }
                )
            }
            Fields::Named(variant_fields) => {
                if is_attributes_sorted{
                    // sort fields on field name
                    LexiographicMatching::default().visit_fields_named_mut(variant_fields);
                }

                // remove attributes from fields
                variant_fields.named.iter_mut().for_each(|f| f.attrs = vec![]);

                // Parse these fields as arguments to function
                let variant_fields = variant_fields.named.clone();

                // Generate the struct members (This can be kept, it doesn't disturb)
                let variant_idents = variant_fields.iter().map(|f|{
                    let ident = f.ident.clone().unwrap();
                    quote!(#ident: #ident.into())
                });

                // Generate the function arguments (This may be made optional)
                let variant_attr = variant_fields.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    quote! (#field_name: impl Into<#field_type> )
                });
                quote!(
                    #variant_doc
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<#response, ::cw_orch::core::CwEnvError> {
                        let msg = #name::#variant_name {
                            #(#variant_idents,)*
                        };
                        <Self as ::cw_orch::core::contract::interface_traits::#trait_name<Chain>>::#func_name(self, &msg.into(),#passed_coins)
                    }
                )
            }
        }
    });

    let necessary_trait_where = quote!(#name<#type_generics>: Into<#generic_msg_type>);
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

    let derived_trait = quote!(
        #[cfg(not(target_arch = "wasm32"))]
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname<Chain: #chain_trait, #generic_msg_type, #type_generics>: ::cw_orch::core::contract::interface_traits::#trait_name<Chain, #trait_msg_type = #generic_msg_type> #combined_trait_where_clause {
            #(#variant_fns)*
        }

        #[cfg(target_arch = "wasm32")]
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname{

        }
    );

    // We need to merge the where clauses (rust doesn't support 2 wheres)
    // If there is no where clause, we simply add the necessary where
    let necessary_where = quote!(SupportedContract: ::cw_orch::core::contract::interface_traits::#trait_name<Chain, #trait_msg_type = #generic_msg_type >, #necessary_trait_where);
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
        impl<SupportedContract, Chain: #chain_trait, #generic_msg_type, #type_generics> #bname<Chain, #generic_msg_type, #type_generics> for SupportedContract
        #combined_where_clause {}
    );

    let expand = quote!(
        #derived_trait

        #[cfg(not(target_arch = "wasm32"))]
        #derived_trait_impl
    );

    expand.into()
}
