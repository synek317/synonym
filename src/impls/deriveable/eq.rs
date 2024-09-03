use crate::info::Info;
use quote::quote;

pub fn impl_eq(info: &Info) -> proc_macro2::TokenStream {
    if !is_eq(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        #[allow(missing_docs)]
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

    info.kind.is_eq()
}
