use crate::info::Info;
use quote::quote;

pub fn impl_as_ref(info: &Info) -> proc_macro2::TokenStream {
    if !is_as_ref(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    quote! {
        #[allow(missing_docs)]
        impl ::core::convert::AsRef<#typ> for #name {
            fn as_ref(&self) -> &#typ {
                &self.0
            }
        }
    }
}

pub fn is_as_ref(info: &Info) -> bool {
    if info.attrs.skip.as_ref {
        return false;
    }

    true
}
