use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_partial_ord(info: &Info) -> proc_macro2::TokenStream {
    if !is_partial_ord(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
    }
}

pub fn is_partial_ord(info: &Info) -> bool {
    if info.attrs.force.partial_ord || info.attrs.force.ord {
        return true;
    }
    if info.attrs.skip.partial_ord {
        return false;
    }

    matches!(
        info.kind,
        Kind::Integer | Kind::Float | Kind::String | Kind::Char
    )
}
