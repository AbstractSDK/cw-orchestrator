extern crate proc_macro;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, DeriveInput, Fields, Ident};

use crate::helpers::{impl_into, LexiographicMatching};

fn payable(v: &syn::Variant) -> bool {
    for attr in &v.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "payable" {
            return true;
        }
    }
    false
}

pub fn execute_fns_derive(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Does the struct have an #[impl_into] attribute?
    let impl_into = impl_into(&ast);

    // If so, we need to add a .into() to the execute fn and set the entrypoint message message
    let (maybe_into, entrypoint_msg_type) = if let Some(entrypoint_msg_type) = impl_into {
        (quote!(.into()), quote!(#entrypoint_msg_type))
    } else {
        (quote!(), quote!(#name))
    };

    let bname = Ident::new(&format!("{}Fns", name), name.span());

    let syn::Data::Enum(syn::DataEnum {
        variants,
        ..
    }) = ast.data
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
                    (quote!(coins: &[::cosmwasm_std::Coin]),quote!(Some(coins)))
                } else {
                    (quote!(),quote!(None))
                };
                let variant_attr = variant_idents.iter();
                Some(quote!(
                    #[allow(clippy::too_many_arguments)]
                    fn #variant_func_name(&self, #(#variant_attr,)* #maybe_coins_attr) -> Result<::boot_core::TxResponse<Chain>, ::boot_core::BootError> {
                        let msg = #name::#variant_name {
                            #(#variant_ident_content_names,)*
                        };
                        self.execute(&msg #maybe_into,#passed_coins)
                    }
                ))
            }
        }
    });

    let derived_trait = quote!(
        pub trait #bname<Chain: ::boot_core::BootEnvironment>: ::boot_core::prelude::BootExecute<Chain, ExecuteMsg = #entrypoint_msg_type> {
            #(#variant_fns)*
        }
    );

    let derived_trait_impl = quote!(
        #[automatically_derived]
        impl<T, Chain: ::boot_core::BootEnvironment> #bname<Chain> for T
        where
            T: ::boot_core::prelude::BootExecute<Chain, ExecuteMsg = #entrypoint_msg_type>{}
    );

    let expand = quote!(
        #derived_trait

        #derived_trait_impl
    );

    expand.into()
}
