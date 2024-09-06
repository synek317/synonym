use crate::info::Info;
use quote::quote;

pub fn impl_serialize(info: &Info) -> proc_macro2::TokenStream {
    if !is_serialize(info) {
        return quote! {};
    }

    let name = &info.name;

    quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }
    }
}

#[cfg(any(test, feature = "with_serde"))]
pub fn is_serialize(info: &Info) -> bool {
    if info.attrs.force.serialize {
        return true;
    }
    if info.attrs.skip.serialize {
        return false;
    }

    info.kind.is_serialize()
}

#[cfg(not(any(test, feature = "with_serde")))]
pub fn is_serialize(_: &Info) -> bool {
    false
}
