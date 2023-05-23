use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::cmp::Ordering;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Attribute, Field, FieldsNamed,
    GenericArgument, GenericParam, Generics, Lit, Meta, NestedMeta, PathArguments, Type,
};

pub(crate) fn impl_into(attrs: &Vec<Attribute>) -> Option<Type> {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "impl_into" {
            return Some(
                attr.parse_args().unwrap_or_else(|_| {
                    panic!("impl_into must be followed by the entrypoint type")
                }),
            );
        }
    }
    None
}

pub(crate) fn process_fn_name(v: &syn::Variant) -> String {
    for attr in &v.attrs {
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            if let Some(ident) = list.path.get_ident() {
                if ident == "fn_name" {
                    if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = list.nested.last() {
                        return lit_str.value();
                    }
                }
            }
        }
    }
    v.ident.to_string()
}

pub fn to_generic_argument(p: &GenericParam) -> GenericArgument {
    match p {
        GenericParam::Type(t) => {
            let ident = &t.ident;
            GenericArgument::Type(parse_quote!(#ident))
        }
        GenericParam::Lifetime(l) => GenericArgument::Lifetime(l.lifetime.clone()),
        GenericParam::Const(c) => GenericArgument::Const(parse_quote!(#c)),
    }
}

pub(crate) fn process_impl_into(
    attrs: &Vec<Attribute>,
    ident: &Ident,
    generics: Generics,
) -> (TokenStream, TokenStream, Punctuated<GenericArgument, Comma>) {
    // Does the struct have an #[impl_into] attribute?
    let impl_into = impl_into(attrs);
    // expect empty generics
    let mut type_generics = generics.params.iter().map(to_generic_argument).collect();
    // If so, we need to add a .into() to the execute fn and set the entrypoint message message
    if let Some(entrypoint_msg_type) = impl_into {
        // extract the type generics
        if let Type::Path(e) = &entrypoint_msg_type {
            let type_args = e.path.segments[0].arguments.clone();
            if let PathArguments::AngleBracketed(argo) = type_args {
                type_generics = argo.args
            }
        };
        (quote!(.into()), quote!(#entrypoint_msg_type), type_generics)
    } else {
        (quote!(), quote!(#ident), type_generics)
    }
}

#[derive(Default)]
pub(crate) struct LexiographicMatching {}

impl syn::visit_mut::VisitMut for LexiographicMatching {
    fn visit_fields_named_mut(&mut self, i: &mut FieldsNamed) {
        let mut fields: Vec<Field> = i.named.iter().map(Clone::clone).collect();
        // sort fields on field name and optionality
        fields.sort_by(|a, b| {
            maybe_compare_option(a, b, "Option").unwrap_or_else(|| {
                a.ident
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .cmp(&b.ident.as_ref().unwrap().to_string())
            })
        });
        let sorted_fields: Punctuated<Field, Comma> = Punctuated::from_iter(fields);
        *i = FieldsNamed {
            named: sorted_fields,
            ..i.clone()
        };
    }
}

fn maybe_compare_option(a: &Field, b: &Field, wrapper: &str) -> Option<Ordering> {
    if is_option(wrapper, &a.ty) && is_option(wrapper, &b.ty) {
        return Some(
            a.ident
                .as_ref()
                .unwrap()
                .to_string()
                .cmp(&b.ident.as_ref().unwrap().to_string()),
        );
    }
    // if one is an option, the other one is lesser
    else if is_option(wrapper, &a.ty) {
        return Some(Ordering::Greater);
    } else if is_option(wrapper, &b.ty) {
        return Some(Ordering::Less);
    }
    None
}

fn is_option(wrapper: &str, ty: &'_ syn::Type) -> bool {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return false;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return false;
            }
            return true;
        }
    }
    false
}
