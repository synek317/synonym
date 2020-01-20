//! # Overview
//! Another try to simplify newtype creation in rust.
//!
//! This crate provides customizable `synonym` derive macro to create newtype and automatically implement some traits basing on the underlying type.
//!
//! # Work in progress
//!
//! **Warning:** the code works fine, it is tested and should be useful, but is still not well documented. The documentation will be delivered with the version `0.1.0`, hopefully soon.

extern crate proc_macro;

mod analyze;
mod attrs;
mod impls;
mod info;

use crate::proc_macro::TokenStream;
use crate::{analyze::analyze, impls::*};
use quote::quote;
use quote::TokenStreamExt;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Synonym, attributes(synonym))]
pub fn synonym_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let info = analyze(&input).expect("Synonym supports only tuple structs with single unnamed field. Enums, unions and generic type parameters are not supported");
    let mut expanded = quote! {};

    expanded.append_all(&[
        impl_eq(&info),
        impl_partial_eq(&info),
        impl_ord(&info),
        impl_partial_ord(&info),
        impl_clone(&info),
        impl_copy(&info),
        impl_hash(&info),
        impl_default(&info),
        impl_debug(&info),
        impl_as_ref(&info),
        impl_deref(&info),
        impl_deref_mut(&info),
        impl_from(&info),
        impl_from_str(&info),
        impl_string(&info),
        impl_number(&info),
    ]);

    TokenStream::from(expanded)
}
