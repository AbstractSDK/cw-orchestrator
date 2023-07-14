extern crate proc_macro;
use crate::helpers::{process_fn_name, process_impl_into, LexiographicMatching};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, DeriveInput, Fields, Ident};

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

    let syn::Data::Enum(syn::DataEnum {
        variants,
        ..
    }) = input.data
     else {
        unimplemented!();
    };

    let variant_fns = variants.into_iter().filter_map( |mut variant|{
        let variant_name = variant.ident.clone();

        // We rename the variant if it has a fn_name attribute associated with it
        let mut variant_func_name =
                format_ident!("{}", process_fn_name(&variant).to_case(Case::Snake));
        variant_func_name.set_span(variant_name.span());

        let is_payable = payable(&variant);
        match &mut variant.fields {
            Fields::Unnamed(_) => None,
            Fields::Unit => None,
            Fields::Named(variant_fields) => {
                // sort fields on field name
                LexiographicMatching::default().visit_fields_named_mut(variant_fields);

                // parse these fields as arguments to function
                let mut variant_idents = variant_fields.named.clone();

                // remove any attributes for use in fn arguments
                variant_idents.iter_mut().for_each(|f| f.attrs = vec![]);

                let variant_ident_content_names = variant_idents.iter().map(|f|f.ident.clone().unwrap());

                let (maybe_coins_attr, passed_coins) = if is_payable {
                    (quote!(coins: &[::cosmwasm_std::Coin]),quote!(Some(coins)))
                } else {
                    (quote!(),quote!(None))
                };
                let variant_attr = variant_idents.iter();
                Some(quote!(
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::cw_orch::prelude::TxResponse<Chain>, ::cw_orch::prelude::CwOrchError> {
                        let msg = #name::#variant_name {
                            #(#variant_ident_content_names,)*
                        };
                        <Self as ::cw_orch::prelude::CwOrchExecute<Chain>>::execute(self, &msg #maybe_into,#passed_coins)
                    }
                ))
            }
        }
    });

    let derived_trait = quote!(
        pub trait #bname<Chain: ::cw_orch::prelude::CwEnv, #type_generics>: ::cw_orch::prelude::CwOrchExecute<Chain, ExecuteMsg = #entrypoint_msg_type #ty_generics #where_clause> {
            #(#variant_fns)*
        }
    );

    let derived_trait_impl = quote!(
        #[automatically_derived]
        impl<SupportedContract, Chain: ::cw_orch::prelude::CwEnv, #type_generics> #bname<Chain, #type_generics> for SupportedContract
        where
            SupportedContract: ::cw_orch::prelude::CwOrchExecute<Chain, ExecuteMsg = #entrypoint_msg_type #ty_generics #where_clause>{}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );

    expand.into()
}
