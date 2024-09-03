use crate::{impls::is_ord, info::Info};
use quote::quote;

pub fn impl_partial_ord(info: &Info) -> proc_macro2::TokenStream {
    if !is_partial_ord(info) {
        return quote! {};
    }

    let name = &info.name;

    if is_ord(info) {
        quote! {
            #[allow(missing_docs)]
            impl ::core::cmp::PartialOrd for #name {
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        }
    } else {
        quote! {
            #[allow(missing_docs)]
            impl ::core::cmp::PartialOrd for #name {
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
        }
    }
}

pub fn is_partial_ord(info: &Info) -> bool {
    if info.attrs.force.partial_ord || info.attrs.force.ord {
        return true;
    }
    if info.attrs.skip.partial_ord {
        return false;
    }

    info.kind.is_partial_ord()
}
