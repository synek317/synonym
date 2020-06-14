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
                #typ::deserialize(deserializer).map(#name)
            }
        }
    }
}

#[cfg(any(test, feature = "with_serde"))]
pub fn is_deserialize(info: &Info) -> bool {
    use crate::info::Kind;

    if info.attrs.force.serialize {
        return true;
    }
    if info.attrs.skip.serialize {
        return false;
    }

    match info.kind {
        Kind::Other => false,
        Kind::Integer | Kind::Float | Kind::String | Kind::Char => true,
    }
}

#[cfg(not(any(test, feature = "with_serde")))]
pub fn is_deserialize(_: &Info) -> bool {
    false
}