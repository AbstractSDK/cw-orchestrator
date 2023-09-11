extern crate proc_macro;
use crate::helpers::{process_fn_name, process_impl_into, process_sorting, LexiographicMatching};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, Fields, Ident, ItemEnum, Type};

const RETURNS: &str = "returns";

/// Extract the query -> response mapping out of an enum variant.
fn parse_query_type(v: &syn::Variant) -> Type {
    let response_ty: syn::Type = v
        .attrs
        .iter()
        .find(|a| a.path.get_ident().unwrap() == RETURNS)
        .unwrap_or_else(|| panic!("missing return type for query: {}", v.ident))
        .parse_args()
        .unwrap_or_else(|_| panic!("return for {} must be a type", v.ident));
    response_ty
}

pub fn query_fns_derive(input: ItemEnum) -> TokenStream {
    let name = &input.ident;
    let bname = Ident::new(&format!("{name}Fns"), name.span());

    let generics = input.generics.clone();
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl().clone();
    let (maybe_into, entrypoint_msg_type, type_generics) =
        process_impl_into(&input.attrs, name, input.generics);

    let is_attributes_sorted = process_sorting(&input.attrs);

    let variants = input.variants;

    let variant_fns = variants.into_iter().map( |mut variant|{
        let variant_name = variant.ident.clone();
        let response = parse_query_type(&variant);
        let mut variant_func_name =
                format_ident!("{}", process_fn_name(&variant).to_case(Case::Snake));
        variant_func_name.set_span(variant_name.span());

        match &mut variant.fields {
            Fields::Unnamed(variant_fields) => {
                let mut variant_idents = variant_fields.unnamed.clone();

                // remove attributes from fields
                variant_idents.iter_mut().for_each(|f| f.attrs = vec![]);

                // Parse these fields as arguments to function

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
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)*) -> ::core::result::Result<#response, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name (#(#variant_ident_content_names,)*);
                        self.query(&msg #maybe_into)
                    }
                )
            }
            Fields::Unit => {
                quote!(
                    fn #variant_func_name(&self) -> ::core::result::Result<#response, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name;
                        self.query(&msg #maybe_into)
                    }
                )
            },
            Fields::Named(variant_fields) => {
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
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)*) -> ::core::result::Result<#response, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name {
                            #(#variant_idents,)*
                        };
                        self.query(&msg #maybe_into)
                    }
                )
            }
        }
    });

    let derived_trait = quote!(
        pub trait #bname<Chain: ::cw_orch::prelude::CwEnv, #type_generics>: ::cw_orch::prelude::CwOrchQuery<Chain, QueryMsg = #entrypoint_msg_type #ty_generics > #where_clause {
            #(#variant_fns)*
        }
    );

    // We need to merge the where clauses (rust doesn't support 2 wheres)
    // If there is no where clause, we simply add the necessary where
    let necessary_where = quote!(SupportedContract: ::cw_orch::prelude::CwOrchQuery<Chain, QueryMsg = #entrypoint_msg_type #ty_generics >);
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
        impl<SupportedContract, Chain: ::cw_orch::prelude::CwEnv, #type_generics> #bname<Chain, #type_generics> for SupportedContract
        #combined_where_clause {}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );

    expand.into()
}
