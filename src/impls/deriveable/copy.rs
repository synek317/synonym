use crate::info::Info;
use quote::quote;

pub fn impl_copy(info: &Info) -> proc_macro2::TokenStream {
    if !is_copy(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::marker::Copy for #name {}
    }
}

pub fn is_copy(info: &Info) -> bool {
    if info.attrs.force.copy {
        return true;
    }
    if info.attrs.skip.copy {
        return false;
    }

    info.kind.is_copy()
}
