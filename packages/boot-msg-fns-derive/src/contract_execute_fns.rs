extern crate proc_macro;
use crate::helpers::{process_impl_into, LexiographicMatching};
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

pub fn contract_execute_fns_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let bname = Ident::new(&format!("Contract{name}Fns"), name.span());
    let (maybe_into, entrypoint_msg_type, type_generics) = process_impl_into(&input.attrs, name);
    let syn::Data::Enum(syn::DataEnum {
        variants,
        ..
    }) = input.data
     else {
        unimplemented!();
    };

    let variant_fns = variants.into_iter().filter_map( |mut variant|{
        let variant_name = variant.ident.clone();
        let is_payable = payable(&variant);
        match &mut variant.fields {
            Fields::Unnamed(_) => None,
            Fields::Unit => None,
            Fields::Named(variant_fields) => {
                // sort fields on field name
                LexiographicMatching::default().visit_fields_named_mut(variant_fields);

                // parse these fields as arguments to function
                let mut variant_func_name =
                format_ident!("{}", variant_name.to_string().to_case(Case::Snake));
                variant_func_name.set_span(variant_name.span());

                let mut variant_idents = variant_fields.named.clone();
                // remove any attributes for use in fn arguments
                variant_idents.iter_mut().for_each(|f| f.attrs = vec![]);

                let variant_ident_content_names = variant_idents.iter().map(|f|f.ident.clone().unwrap());

                let (maybe_coins_attr, passed_coins) = if is_payable {
                    (quote!(coins: &[::cosmwasm_std::Coin]),quote!(coins))
                } else {
                    (quote!(),quote!(vec![]))
                };
                let variant_attr = variant_idents.iter();
                Some(quote!(
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> ::cosmwasm_std::StdResult<::cosmwasm_std::CosmosMsg> {
                        let msg = #name::#variant_name {
                            #(#variant_ident_content_names,)*
                        };
                        <Self as ::boot_msg::CwContractExecute>::execute_msg(self, &msg #maybe_into,#passed_coins)
                    }
                ))
            }
        }
    });

    let cw_variant_functions = variant_fns.clone();

    let derived_trait = quote!(

        pub trait #bname<#type_generics>: ::boot_msg::CwContractExecute<ExecuteMsg = #entrypoint_msg_type> {
            #(#cw_variant_functions)*
        }
    );

    let derived_trait_impl = quote!(
        #[automatically_derived]

        impl<SupportedContract, #type_generics> #bname<#type_generics> for SupportedContract
        where
            SupportedContract: ::boot_msg::CwContractExecute<ExecuteMsg = #entrypoint_msg_type>{}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );

    expand.into()
}
