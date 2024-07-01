use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_from_str(info: &Info) -> proc_macro2::TokenStream {
    if !is_from_str(info) {
        return quote! {};
    }

    let name = &info.name;
    let typ = &info.typ;

    if info.kind == Kind::BoxStr {
        quote! {
            impl ::core::str::FromStr for #name {
                type Err = ::core::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self(s.into()))
                }
            }
        }
    } else {
        quote! {
            impl ::core::str::FromStr for #name {
                type Err = <#typ as ::core::str::FromStr>::Err;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    ::core::str::FromStr::from_str(s).map(Self)
                }
            }
        }
    }
}

pub fn is_from_str(info: &Info) -> bool {
    if info.attrs.force.from_str {
        return true;
    }
    if info.attrs.skip.from_str {
        return false;
    }

    info.kind.is_from_str()
}
