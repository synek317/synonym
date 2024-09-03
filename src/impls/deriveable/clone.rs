use crate::{info::Info, is_copy};
use quote::quote;

pub fn impl_clone(info: &Info) -> proc_macro2::TokenStream {
    if !is_clone(info) {
        return quote! {};
    }

    let name = &info.name;

    if is_copy(info) {
        quote! {
            #[allow(missing_docs)]
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else {
        quote! {
            #[allow(missing_docs)]
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
        }
    }
}

pub fn is_clone(info: &Info) -> bool {
    if info.attrs.force.clone {
        return true;
    }
    if info.attrs.skip.clone {
        return false;
    }

    info.kind.is_clone()
}
