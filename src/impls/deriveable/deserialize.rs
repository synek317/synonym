use crate::info::Info;
use quote::quote;

pub fn impl_deserialize(info: &Info) -> proc_macro2::TokenStream {
    if !is_deserialize(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    quote! {
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <#typ as ::serde::Deserialize<'de>>::deserialize(deserializer).map(#name)
            }
        }
    }
}

#[cfg(any(test, feature = "with_serde"))]
pub fn is_deserialize(info: &Info) -> bool {
    if info.attrs.force.serialize {
        return true;
    }
    if info.attrs.skip.serialize {
        return false;
    }

    info.kind.is_deserialize()
}

#[cfg(not(any(test, feature = "with_serde")))]
pub fn is_deserialize(_: &Info) -> bool {
    false
}
