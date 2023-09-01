use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_eq(info: &Info) -> proc_macro2::TokenStream {
    if !is_eq(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::cmp::Eq for #name {}
    }
}

pub fn is_eq(info: &Info) -> bool {
    if info.attrs.force.eq {
        return true;
    }
    if info.attrs.skip.eq || info.attrs.skip.partial_eq {
        return false;
    }

    matches!(info.kind, Kind::Integer | Kind::String | Kind::Char)
}
