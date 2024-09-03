use crate::info::Info;
use quote::quote;

pub fn impl_deref(info: &Info) -> proc_macro2::TokenStream {
    if !is_deref(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    quote! {
        #[allow(missing_docs)]
        impl ::core::ops::Deref for #name {
            type Target = #typ;

            #[inline]
            fn deref(&self) -> &#typ {
                &self.0
            }
        }
    }
}

pub fn is_deref(info: &Info) -> bool {
    if info.attrs.force.deref || info.attrs.force.deref_mut {
        return true;
    }

    false
}
