use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_ord(info: &Info) -> proc_macro2::TokenStream {
    if !is_ord(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
    }
}

pub fn is_ord(info: &Info) -> bool {
    if info.attrs.force.ord {
        return true;
    }
    if info.attrs.skip.ord || info.attrs.skip.partial_ord {
        return false;
    }

    match info.kind {
        Kind::Integer | Kind::String | Kind::Char => true,
        _ => false,
    }
}
