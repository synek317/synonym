use crate::info::{Info, Kind};
use quote::quote;

pub fn impl_number(info: &Info) -> proc_macro2::TokenStream {
    if !is_number(info) {
        return quote! {};
    }

    let name = &info.name;
    // let typ = &info.typ;

    quote! {
        impl ::core::ops::Add<#name> for #name {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
            }
        }

        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl ::core::ops::Sub<#name> for #name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
            }
        }

        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl ::core::ops::Mul<#name> for #name {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                Self(self.0 * other.0)
            }
        }

        impl ::core::ops::MulAssign for #name {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl ::core::ops::Div<#name> for #name {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                Self(self.0 / other.0)
            }
        }

        impl ::core::ops::DivAssign for #name {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }
    }
}

pub fn is_number(info: &Info) -> bool {
    if info.attrs.force.number {
        return true;
    }
    if info.attrs.skip.number {
        return false;
    }

    match info.kind {
        | Kind::Integer
        | Kind::Float => true,
        _ => false,
    }
}
