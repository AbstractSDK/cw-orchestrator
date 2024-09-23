extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};
const INTERFACE_POSTFIX: &str = "Interface";

#[proc_macro_derive(FromInterface)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let interface_name = &ast.ident;
    let original_name = {
        let counter_name = interface_name.to_string();
        // Supporting "{OriginalName}Interface" format only, to simplify macro and interface names
        let (counter_name, _) = counter_name
            .split_once(INTERFACE_POSTFIX)
            .expect(r#"Interface message type supposed to have "Interface" postfix"#);
        proc_macro2::Ident::new(counter_name, proc_macro2::Span::call_site())
    };
    let froms = ast.variants.into_iter().map(|variant| {
        let variant_name = variant.ident.clone();
        let fields = match variant.fields {
            syn::Fields::Unnamed(variant_fields) => {
                let variant_fields = (0..variant_fields.unnamed.len()).map(|i| {
                    proc_macro2::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site())
                });
                quote!( ( #(#variant_fields,)* ) )
            }
            syn::Fields::Named(variant_fields) => {
                let idents = variant_fields
                    .named
                    .into_iter()
                    .map(|field| field.ident.unwrap());
                quote!( { #(#idents,)* } )
            }
            syn::Fields::Unit => quote!(),
        };
        quote! ( #interface_name::#variant_name #fields => #original_name::#variant_name #fields )
    });
    quote!(
        impl #impl_generics From<#interface_name #ty_generics> for #original_name #ty_generics
        #where_clause
        {
            fn from(value: #interface_name #ty_generics) -> #original_name #ty_generics {
                match value {
                    #(#froms,)*
                }
            }
        }
    )
    .into()
}
