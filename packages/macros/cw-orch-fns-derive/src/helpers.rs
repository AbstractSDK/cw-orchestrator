use quote::ToTokens;
use std::cmp::Ordering;
use syn::{
    parenthesized, punctuated::Punctuated, token::Comma, Field, FieldsNamed, ItemEnum, LitStr, Type,
};

pub enum MsgType {
    Execute,
    Query,
}

pub enum SyncType {
    Sync,
    Async,
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
#[derive(Default)]
pub struct EnumAttributes {
    pub disable_fields_sorting: bool,
}

pub(crate) fn parse_enum_attributes(item_enum: &ItemEnum) -> EnumAttributes {
    let mut enum_attributes = EnumAttributes::default();
    for attr in &item_enum.attrs {
        if attr.path().is_ident("cw_orch") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("disable_fields_sorting") {
                    enum_attributes.disable_fields_sorting = true;
                }
                Ok(())
            })
            .unwrap();
        }
    }
    enum_attributes
}

#[derive(Default)]
pub struct VariantAttributes {
    pub fn_name: String,
    pub payable: bool,
}

pub(crate) fn parse_variant_attributes(variant: &syn::Variant) -> VariantAttributes {
    let mut cw_orch_attributes = VariantAttributes {
        fn_name: variant.ident.to_string(),
        ..Default::default()
    };
    for attr in &variant.attrs {
        if attr.path().is_ident("cw_orch") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("payable") {
                    cw_orch_attributes.payable = true;
                }
                if meta.path.is_ident("fn_name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: LitStr = content.parse()?;
                    cw_orch_attributes.fn_name = lit.value();
                }
                Ok(())
            })
            .unwrap();
        }
    }

    cw_orch_attributes
}

#[derive(Default)]
pub struct FieldAttributes {
    pub into: bool,
}

pub(crate) fn parse_field_attributes(field: &syn::Field) -> FieldAttributes {
    let mut cw_orch_attributes = FieldAttributes::default();
    for attr in &field.attrs {
        if attr.path().is_ident("cw_orch") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("into") {
                    cw_orch_attributes.into = true;
                }
                Ok(())
            })
            .unwrap();
        }
    }

    cw_orch_attributes.into = cw_orch_attributes.into || is_type_using_into(&field.ty);

    cw_orch_attributes
}
