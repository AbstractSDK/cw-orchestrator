use std::cmp::Ordering;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Attribute, Field, FieldsNamed,
    GenericArgument, GenericParam, Generics, Lit, Meta, NestedMeta,
};

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

pub fn to_generic_arguments(generics: &Generics) -> Punctuated<GenericArgument, Comma> {
    generics.params.iter().map(to_generic_argument).collect()
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
