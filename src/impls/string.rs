use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_string(info: &Info) -> proc_macro2::TokenStream {
    if !is_string(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::borrow::Borrow<str> for #name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl<'a> ::core::convert::From<&'a str> for #name {
            fn from(s: &'a str) -> Self {
                Self(::core::convert::From::from(s))
            }
        }
    }
}

pub fn is_string(info: &Info) -> bool {
    if info.attrs.force.string {
        return true;
    }
    if info.attrs.skip.string {
        return false;
    }

    match info.kind {
        Kind::String => true,
        _ => false,
    }
}
