use crate::info::Info;
use quote::quote;

pub fn impl_partial_eq(info: &Info) -> proc_macro2::TokenStream {
    if !is_partial_eq(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                ::core::cmp::PartialEq::eq(&self.0, &other.0)
            }
        }
    }
}

pub fn is_partial_eq(info: &Info) -> bool {
    if info.attrs.force.partial_eq || info.attrs.force.eq {
        return true;
    }
    if info.attrs.skip.partial_eq {
        return false;
    }

    info.kind.is_partial_eq()
}
