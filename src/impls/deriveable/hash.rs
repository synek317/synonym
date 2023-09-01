use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_hash(info: &Info) -> proc_macro2::TokenStream {
    if !is_hash(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::hash::Hash for #name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash::<H>(&self.0, state)
            }
        }
    }
}

pub fn is_hash(info: &Info) -> bool {
    if info.attrs.force.hash {
        return true;
    }
    if info.attrs.skip.hash {
        return false;
    }

    matches!(info.kind, Kind::Integer | Kind::String | Kind::Char)
}
