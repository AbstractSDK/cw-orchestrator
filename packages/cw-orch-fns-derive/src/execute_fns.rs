extern crate proc_macro;
use crate::helpers::{process_fn_name, process_impl_into, process_sorting, LexiographicMatching};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, DeriveInput, Fields, Ident, parse_quote};

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
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl().clone();
    let (maybe_into, entrypoint_msg_type, type_generics) =
        process_impl_into(&input.attrs, name, input.generics);

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
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::cw_orch::prelude::TxResponse<Chain>, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name (
                            #(#variant_ident_content_names,)*
                        );
                        <Self as ::cw_orch::prelude::CwOrchExecute<Chain>>::execute(self, &msg #maybe_into,#passed_coins)
                    }
                )
            },
            Fields::Unit => {

                quote!(
                    #variant_doc
                    fn #variant_func_name(&self, #maybe_coins_attr) -> Result<::cw_orch::prelude::TxResponse<Chain>, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name;
                        <Self as ::cw_orch::prelude::CwOrchExecute<Chain>>::execute(self, &msg #maybe_into,#passed_coins)
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
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::cw_orch::prelude::TxResponse<Chain>, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name {
                            #(#variant_ident_content_names,)*
                        };
                        <Self as ::cw_orch::prelude::CwOrchExecute<Chain>>::execute(self, &msg #maybe_into,#passed_coins)
                    }
                )
            }
        }
    });

    let derived_trait = quote!(
        /// Automatically derived trait that allows you to call the variants of the message directly without the need to construct the struct yourself.
        pub trait #bname<Chain: ::cw_orch::prelude::CwEnv, #type_generics>: ::cw_orch::prelude::CwOrchExecute<Chain, ExecuteMsg = #entrypoint_msg_type #ty_generics> #where_clause {
            #(#variant_fns)*
        }
    );

    // We need to merge the where clauses (rust doesn't support 2 wheres)
    // If there is no where clause, we simply add the necessary where
    let necessary_where = quote!(SupportedContract: ::cw_orch::prelude::CwOrchExecute<Chain, ExecuteMsg = #entrypoint_msg_type #ty_generics >);
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
        #[allow(undocumented)]
        impl<SupportedContract, Chain: ::cw_orch::prelude::CwEnv, #type_generics> #bname<Chain, #type_generics> for SupportedContract
        #combined_where_clause {}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );

    expand.into()
}
