use proc_macro2::TokenTree;
use quote::ToTokens;
use std::cmp::Ordering;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, FieldsNamed, Meta, MetaList,
    NestedMeta, Path, Type,
};

pub enum MsgType {
    Execute,
    Query,
}

pub(crate) fn process_fn_name(v: &syn::Variant) -> String {
    for attr in &v.attrs {
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            if let Some(ident) = list.path.get_ident() {
                if ident == "cw_orch" {
                    for meta in list.nested {
                        if let NestedMeta::Meta(Meta::List(MetaList { nested, .. })) = &meta {
                            if let Some(NestedMeta::Meta(Meta::Path(Path { segments, .. }))) =
                                nested.last()
                            {
                                if let Some(ident) = segments.last() {
                                    return ident.ident.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    v.ident.to_string()
}

pub(crate) fn process_sorting(attrs: &[Attribute]) -> bool {
    !has_cw_orch_attribute(attrs, "disable_fields_sorting")
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

pub(crate) fn has_into(field: &syn::Field) -> bool {
    is_type_using_into(&field.ty) || has_cw_orch_attribute(&field.attrs, "into")
}

pub(crate) fn has_cw_orch_attribute(attrs: &[Attribute], attribute_name: &str) -> bool {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "cw_orch" {
            // We check the payable attribute is in there
            for token_tree in attr.tokens.clone() {
                if let TokenTree::Group(e) = token_tree {
                    for ident in e.stream() {
                        if ident.to_string() == attribute_name {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}
