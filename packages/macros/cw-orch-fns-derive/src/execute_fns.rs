use crate::helpers::has_cw_orch_attribute;

pub fn payable(v: &syn::Variant) -> bool {
    has_cw_orch_attribute(&v.attrs, "payable")
}
