use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::cmp::Ordering;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, FieldsNamed, Lit, Meta, NestedMeta,
    Type,
};

pub enum MsgType {
    Execute,
    Query,
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

pub(crate) fn process_sorting(attrs: &Vec<Attribute>) -> bool {
    // If the disable_fields_sorting attribute is enabled, we return false, no sorting should be done
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "disable_fields_sorting"
        {
            return false;
        }
    }
    true
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

pub(crate) fn has_impl_into(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "impl_into" {
            return true;
        }
    }
    false
}

pub(crate) fn impl_into_deprecation(attrs: &Vec<Attribute>) -> TokenStream {
    if has_impl_into(attrs) {
        quote!(
            #[deprecated = "the `impl_into` attribute is deprecated. You don't need to use it anymore"]
        )
    } else {
        quote!()
    }
}

pub(crate) fn is_type_using_into(field_type: &Type) -> bool {
    // We match Strings
    match field_type {
        Type::Path(type_path) => {
            let path_string = type_path.clone().into_token_stream().to_string();

            if path_string == "String" {
                return true;
            }

            if path_string.contains("Uint") {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub(crate) fn has_force_into(field: &syn::Field) -> bool {
    for attr in &field.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "into" {
            return true;
        }
    }
    false
}

pub(crate) fn has_into(field: &syn::Field) -> bool {
    is_type_using_into(&field.ty) || has_force_into(field)
}
