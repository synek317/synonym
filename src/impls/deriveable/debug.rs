use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_debug(info: &Info) -> proc_macro2::TokenStream {
    if !is_debug(info) {
        return quote! {};
    }

    let name = &info.name;
    let name_str = name.to_string();

    quote! {
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(#name_str)
                    .field(&self.0)
                    .finish()
            }
        }
    }
}

pub fn is_debug(info: &Info) -> bool {
    if info.attrs.force.debug {
        return true;
    }
    if info.attrs.skip.debug {
        return false;
    }

    match info.kind {
        Kind::Integer | Kind::Float | Kind::String | Kind::Char => true,
        _ => false,
    }
}
