use crate::attrs::DisplayKind;
use crate::info::Info;
use quote::quote;

pub fn impl_display(info: &Info) -> proc_macro2::TokenStream {
    if !is_display(info) {
        return quote! {};
    }

    let name = &info.name;
    let name_s = name.to_string();

    let fmt_impl = match info.attrs.display {
        DisplayKind::Opaque => quote! {
            ::core::write!(f, "{}({})", #name_s, self.0)
        },
        DisplayKind::Transparent => quote! {
            ::core::fmt::Display::fmt(&self.0, f)
        },
        DisplayKind::UpperCase => quote! {
          ::core::fmt::Display::fmt(&self.0.to_uppercase(), f)
        },
        DisplayKind::LowerCase => quote! {
            ::core::fmt::Display::fmt(&self.0.to_lowercase(), f)
        },
        DisplayKind::OpaqueUpperCase => quote! {
            ::core::write!(f, "{}({})", #name_s, self.0.to_uppercase())
        },
        DisplayKind::OpaqueLowerCase => quote! {
            ::core::write!(f, "{}({})", #name_s, self.0.to_lowercase())
        },
        DisplayKind::Custom(ref fmt) => quote! {
            ::core::write!(f, #fmt, self.0)
        },
    };

    quote! {
        #[allow(missing_docs)]
        impl ::core::fmt::Display for #name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #fmt_impl
            }
        }
    }
}

pub fn is_display(info: &Info) -> bool {
    if info.attrs.force.display {
        return true;
    }
    if info.attrs.skip.display {
        return false;
    }

    info.kind.is_display()
}
