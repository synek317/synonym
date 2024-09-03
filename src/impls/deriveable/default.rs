use crate::info::Info;
use quote::quote;

pub fn impl_default(info: &Info) -> proc_macro2::TokenStream {
    if !is_default(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        #[allow(missing_docs)]
        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self(::core::default::Default::default())
            }
        }
    }
}

pub fn is_default(info: &Info) -> bool {
    if info.attrs.force.default {
        return true;
    }
    if info.attrs.skip.default {
        return false;
    }

    info.kind.is_default()
}
