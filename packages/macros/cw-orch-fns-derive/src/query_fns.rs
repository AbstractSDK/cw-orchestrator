use quote::quote;

const RETURNS: &str = "returns";

/// Extract the query -> response mapping out of an enum variant.
pub fn parse_query_type(v: &syn::Variant) -> proc_macro2::TokenStream {
    let response_ty: syn::Type = v
        .attrs
        .iter()
        .find(|a| a.path.get_ident().unwrap() == RETURNS)
        .unwrap_or_else(|| panic!("missing return type for query: {}", v.ident))
        .parse_args()
        .unwrap_or_else(|_| panic!("return for {} must be a type", v.ident));
    quote!(#response_ty)
}
