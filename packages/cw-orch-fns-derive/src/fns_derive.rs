extern crate proc_macro;
use crate::{
    execute_fns::payable,
    helpers::{process_fn_name, process_sorting, LexiographicMatching, MsgType},
    query_fns::parse_query_type,
};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_quote, visit_mut::VisitMut, Fields, Generics, Ident, ItemEnum, WhereClause};

pub fn fns_derive(msg_type: MsgType, input: ItemEnum) -> TokenStream {
    let name = &input.ident;

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

    let variant_fns = input.variants.into_iter().map( |mut variant|{
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
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<#response, ::cw_orch::core::CwEnvError> {
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
                let is_attributes_sorted = process_sorting(&input.attrs);
                if is_attributes_sorted{
                    // sort fields on field name
                    LexiographicMatching::default().visit_fields_named_mut(variant_fields);
                }

                // remove attributes from fields
                variant_fields.named.iter_mut().for_each(|f| f.attrs = vec![]);

                // Parse these fields as arguments to function
                let variant_fields = variant_fields.named.clone();
                let variant_idents = variant_fields.iter().map(|f|f.ident.clone().unwrap());

                let variant_attr = variant_fields.iter();
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

    // Generics for the Trait
    let mut cw_orch_generics: Generics = parse_quote!(<Chain: #chain_trait,  #generic_msg_type>);
    cw_orch_generics
        .params
        .extend(input.generics.params.clone());

    // Where clause for the Trait
    let mut combined_trait_where_clause = {
        let (_, ty_generics, where_clause) = input.generics.split_for_impl().clone();

        // Adding a where clause for the derive message type to implement into the contract message type
        let mut clause: WhereClause =
            parse_quote!(where #name #ty_generics: Into<#generic_msg_type>);

        // Adding eventual where clauses that were present on the original QueryMsg
        if let Some(w) = where_clause {
            clause.predicates.extend(w.predicates.clone());
        }
        clause
    };

    let bname = Ident::new(&format!("{name}Fns"), name.span());
    let trait_condition = quote!(::cw_orch::core::contract::interface_traits::#trait_name<Chain, #trait_msg_type = #generic_msg_type>);

    let derived_trait = quote!(
        #[cfg(not(target_arch = "wasm32"))]
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname #cw_orch_generics : #trait_condition #combined_trait_where_clause {
            #(#variant_fns)*
        }

        #[cfg(target_arch = "wasm32")]
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname{

        }
    );

    // Generating the generics for the blanket implementation
    let mut supported_contract_generics = cw_orch_generics.clone();
    supported_contract_generics
        .params
        .push(parse_quote!(SupportedContract));

    // Generating the where clause for the blanket implementation
    combined_trait_where_clause
        .predicates
        .push(parse_quote!(SupportedContract: #trait_condition));

    let (support_contract_impl, _, _) = supported_contract_generics.split_for_impl();
    let (_, cw_orch_generics, _) = cw_orch_generics.split_for_impl();

    let derived_trait_blanket_impl = quote!(
        #[automatically_derived]
        impl #support_contract_impl #bname #cw_orch_generics for SupportedContract
        #combined_trait_where_clause {}
    );

    let expand = quote!(
        #derived_trait

        #[cfg(not(target_arch = "wasm32"))]
        #derived_trait_blanket_impl
    );

    expand.into()
}
