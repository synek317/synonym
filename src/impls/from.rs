use crate::info::Info;
use quote::quote;

pub fn impl_from(info: &Info) -> proc_macro2::TokenStream {
    if !is_from(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    quote! {
        #[allow(missing_docs)]
        impl ::core::convert::From<#typ> for #name {
            fn from(t: #typ) -> Self {
                Self(::core::convert::From::from(t))
            }
        }

        #[allow(missing_docs)]
        impl ::core::convert::From<#name> for #typ {
            fn from(t: #name) -> Self {
                t.0
            }
        }
    }
}

pub fn is_from(info: &Info) -> bool {
    if info.attrs.skip.from {
        return false;
    }

    true
}
