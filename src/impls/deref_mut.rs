use crate::info::Info;
use quote::quote;

pub fn impl_deref_mut(info: &Info) -> proc_macro2::TokenStream {
    if !is_deref_mut(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    quote! {
        impl ::core::ops::DerefMut for #name {
            #[inline]
            fn deref_mut(&mut self) -> &mut #typ {
                &mut self.0
            }
        }
    }
}

pub fn is_deref_mut(info: &Info) -> bool {
    if info.attrs.force.deref_mut {
        return true;
    }

    false
}
