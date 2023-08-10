use proc_macro::TokenStream;
extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(ParseCwMsg)]
pub fn derive_parse_cw_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    parse_fn_derive(input)
}

fn parse_fn_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    match &input.data {
        syn::Data::Struct(data) => {
            let syn::Fields::Named(FieldsNamed { named, .. }) = data.fields.clone() else {
                unimplemented!()
            };
            let fields = named.into_iter().map(|field| {
                let ident = field.ident.unwrap();
                let ty = field.ty;
                let message = format!("{}({})", ident, quote!(#ty).to_string());
                quote!(#ident: ::cw_orch_cli::custom_type_serialize(#message)?)
            });
            let derived_trait_impl = quote!(
                #[automatically_derived]
                impl ::cw_orch_cli::ParseCwMsg for #name {
                    fn parse() -> ::cw_orch::anyhow::Result<Self> {
                        Ok(Self {
                            #(#fields),*
                        })
                    }
                }
            );
            derived_trait_impl.into()
        }
        syn::Data::Enum(data) => {
            let idents: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();
            let enum_variants_ident =
                proc_macro2::Ident::new(&format!("{name}Variants"), name.span());
            let enum_of_variant_names = quote!(
                enum #enum_variants_ident {
                    #(#idents),*
                }
            );
            let display_for_enum_variant_names = idents.iter().map(|&ident| {
                let name = ident.to_string().to_lowercase();
                quote!(
                    #enum_variants_ident::#ident => f.pad(#name)
                )
            });
            let display_for_enum_variant_names = quote!(
                impl ::std::fmt::Display for #enum_variants_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match self {
                            #(#display_for_enum_variant_names),*
                        }
                    }
                }
            );
            let variants_as_structs = data.variants.iter().map(|variant| {
                let struct_name = &variant.ident;
                let fields = &variant.fields;
                let into_enum = quote!(
                    impl From<#struct_name> for #name {
                        fn from(val: #struct_name) -> Self {
                            let #struct_name #fields = val;
                            #name::#struct_name
                                #fields

                        }
                    }
                );
                let sub_fields = fields.iter().map(|field| {
                    let ident = field.ident.clone().unwrap();
                    let ty = &field.ty;
                    let message = format!("{}({})", ident, quote!(#ty).to_string());
                    quote!(#ident: ::cw_orch_cli::custom_type_serialize(#message)?)
                });
                let sub_derived_trait_impl = quote!(
                    #[automatically_derived]
                    impl ::cw_orch_cli::ParseCwMsg for #struct_name {
                        fn parse() -> ::cw_orch::anyhow::Result<Self> {
                            Ok(Self {
                                #(#sub_fields),*
                            })
                        }
                    }
                );
                quote!(
                    struct #struct_name
                    #fields
                    #into_enum
                    #sub_derived_trait_impl
                )
            });
            let derived_trait_impl = quote!(
                #[automatically_derived]
                impl ::cw_orch_cli::ParseCwMsg for #name {
                    fn parse() -> ::cw_orch::anyhow::Result<Self> {
                        #enum_of_variant_names
                        #display_for_enum_variant_names
                        let options = vec![#(#enum_variants_ident::#idents),*];
                        let variant = ::cw_orch_cli::select_msg(options)?;
                        #(#variants_as_structs)*
                        let msg = match variant {
                            #(#enum_variants_ident:: #idents => #idents::parse()?.into()),*
                        };
                        Ok(msg)
                    }
                }
            );
            derived_trait_impl.into()
        }
        syn::Data::Union(_) => {
            unimplemented!()
        }
    }
}
