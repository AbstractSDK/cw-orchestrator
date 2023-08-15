use convert_case::Casing;
use proc_macro::TokenStream;
extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(ParseCwMsg)]
pub fn derive_parse_cw_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    parse_fn_derive(input)
}

fn parse_fn_derive(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    match &input.data {
        syn::Data::Struct(data) => impl_parse_for_struct(&data.fields, name),
        syn::Data::Enum(data) => {
            let idents: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();
            // Generate helper enum
            let enum_variants_ident =
                proc_macro2::Ident::new(&format!("{name}Variants"), name.span());
            let enum_of_variant_names = quote!(
                enum #enum_variants_ident {
                    #(#idents),*
                }
            );
            let display_for_enum_variant_names = idents.iter().map(|&ident| {
                let name = ident.to_string().to_case(convert_case::Case::Snake);
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
                let syn::Fields::Named(FieldsNamed {
                    named,
                    ..
                }) = &variant.fields else {
                    unimplemented!()
                };
                let field_names = named.into_iter().map(|a| a.ident.clone().unwrap());
                let field_names = quote!({#(#field_names),*});
                let into_enum = quote!(
                    impl From<#struct_name> for #name {
                        fn from(val: #struct_name) -> Self {
                            let #struct_name #field_names = val;
                            #name::#struct_name #field_names
                        }
                    }
                );
                let fields = &variant.fields;
                let sub_derived_trait_impl = impl_parse_for_struct(fields, struct_name);
                quote!(
                    struct #struct_name
                    #fields
                    #into_enum
                    #sub_derived_trait_impl
                )
            });
            quote!(
                #[automatically_derived]
                impl ::cw_orch_cli::ParseCwMsg for #name {
                    fn cw_parse(state_interface: &impl ::cw_orch::state::StateInterface) -> ::cw_orch::anyhow::Result<Self> {
                        #enum_of_variant_names
                        #display_for_enum_variant_names
                        let options = vec![#(#enum_variants_ident::#idents),*];
                        let variant = ::cw_orch_cli::select_msg(options)?;
                        #(#variants_as_structs)*
                        let msg = match variant {
                            #(#enum_variants_ident::#idents => #idents::cw_parse(state_interface)?.into()),*
                        };
                        Ok(msg)
                    }
                }
            )
        }
        syn::Data::Union(_) => {
            unimplemented!()
        }
    }
    .into()
}

fn impl_parse_for_struct(fields: &Fields, name: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let syn::Fields::Named(FieldsNamed {
        named,
        ..
    }) = fields else {
        unimplemented!()
    };
    let fields = named.into_iter().map(|field| {
        let ident = field.ident.clone().unwrap();
        let ty = field.ty.clone();
        let message = format!("{}({})", ident, quote!(#ty));
        quote!(#ident: ::cw_orch_cli::custom_type_serialize(#message)?)
    });
    let derived_trait_impl = quote!(
        #[automatically_derived]
        impl ::cw_orch_cli::ParseCwMsg for #name {
            fn cw_parse(_state_interface: &impl ::cw_orch::state::StateInterface) -> ::cw_orch::anyhow::Result<Self> {
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    );
    derived_trait_impl
}
