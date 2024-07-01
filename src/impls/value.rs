use crate::info::Info;
use quote::quote;

pub fn impl_value(info: &Info) -> proc_macro2::TokenStream {
    if !is_value(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    if info.kind.is_copy() || info.attrs.force.copy {
        quote! {
            impl #name {
                pub fn value(&self) -> #typ {
                    self.0
                }
            }
        }
    } else {
        quote! {
            impl #name {
                pub fn value(&self) -> &#typ {
                    &self.0
                }
            }
        }
    }
}

pub fn is_value(info: &Info) -> bool {
    if info.attrs.skip.value {
        return false;
    }

    true
}
