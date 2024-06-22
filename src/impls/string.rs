use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_string(info: &Info) -> proc_macro2::TokenStream {
    if !is_string(info) {
        return quote! {};
    }

    let name = &info.name;

    let mut tokens = quote! {
        impl ::core::borrow::Borrow<str> for #name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl #name {
            #[allow(missing_docs)]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };

    if info.kind != Kind::StaticStr {
        tokens.extend(quote! {
            impl<'a> ::core::convert::From<&'a str> for #name {
                fn from(s: &'a str) -> Self {
                    Self(::core::convert::From::from(s))
                }
            }
        });
    }

    tokens
}

pub fn is_string(info: &Info) -> bool {
    if info.attrs.force.string {
        return true;
    }
    if info.attrs.skip.string {
        return false;
    }

    info.kind.is_string()
}
