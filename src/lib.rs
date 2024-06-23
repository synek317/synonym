//! # Overview
//!
//! The `synonym` library is a Rust crate designed to simplify the creation of newtypes. It provides a customizable `#[derive(Synonym)]` macro that automatically implements various traits based on the underlying type of your newtype. This saves you from the boilerplate code usually required when defining newtypes.
//!
//! # Usage
//!
//! To use `synonym`, add it to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! synonym = "0.1.3"
//! ```
//!
//! ## Basic example
//!
//! Import the `Synonym` trait into your Rust file:
//!
//! ```rust
//! use synonym::Synonym;
//! ```
//!
//! Then, define your newtype and annotate it with `#[derive(Synonym)]:`
//! ```rust
//! #[derive(Synonym)]
//! pub struct MyInt(i32);
//! ```
//!
//! ## Customization with Attributes
//! You can customize which traits are implemented or skipped using the `#[synonym(skip(...))]` and `#[synonym(force(...))]` attributes:
//! ```rust
//! #[derive(Synonym)]
//! #[synonym(skip(Eq, PartialEq))]
//! pub struct MyString(String);
//! ```
//!
//! # Generated code
//! When you use `#[derive(Synonym)]`, the library generates implementations for various traits. Here's a simplified example for a newtype `MyInt(i32)`:
//! ```rust
//! impl Eq for MyInt {}
//! impl PartialEq for MyInt {
//!     fn eq(&self, other: &Self) -> bool {
//!         self.0 == other.0
//!     }
//! }
//! // ... and so on for other traits
//! ```
//!
//! # Trait implementation table
//!
//! | Kind    | Traits / Methods Implemented |
//! | ------- | ---------------------------- |
//! | Integer<br>`u8`, `u16`, `u32`, `u64`, `u128`, `usize`,<br>`i8`, `i16`, `i32`, `i64`, `i128`, `isize` |  `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Copy`, `Hash`, `Default`, `Debug`, `Add`, `Sub`, `Mul`, `Div`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `FromStr`, `From`, `AsRef`, `Deref` |
//! | Integer<br>`NonZeroU8`, `NonZeroU16`, `NonZeroU32`, `NonZeroU64`, `NonZeroU128`, `NonZeroUsize`,<br>`NonZeroI8`, `NonZeroI16`, `NonZeroI32`, `NonZeroI64`, `NonZeroI128`, `NonZeroIsize` |  `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Copy`, `Hash`, `Debug`, `FromStr`, `From`, `AsRef`, `Deref` |
//! | Float<br>`f32`, `f64`   | `PartialEq`, `PartialOrd`, `Clone`, `Default`, `Debug`, `Add`, `Sub`, `Mul`, `Div`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `FromStr`, `From`, `AsRef`, `Deref` |
//! | String<br>`String`, `Box<str>`  | `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Hash`, `Default`, `Debug`, `FromStr`, `From`, `AsRef`, `Deref`, `Borrow<str>`, `as_str()` |
//! | String<br>`&'static str`  | `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Hash`, `Default`, `Debug`, `From`, `AsRef`, `Deref`, `Borrow<str>`, `as_str()` |
//! | Char<br>`char`    | `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Copy`, `Hash`, `Default`, `Debug`, `FromStr`, `From`, `AsRef`, `Deref` |
//!
//! # Fine-tuning
//!
//! ## Display
//! To specify how the Display trait should be implemented, you can use the `#[synonym(display = "...")]` attribute. Here are the available options:
//!
//! * `Opaque`: Formats the output as TypeName(Value).
//! * `Transparent`: Directly uses the inner type's Display implementation.
//! * `UpperCase`: Converts the inner value to uppercase before displaying.
//! * `LowerCase`: Converts the inner value to lowercase before displaying.
//! * `OpaqueUpperCase`: Formats the output as TypeName(VALUE) where VALUE is uppercase.
//! * `OpaqueLowerCase`: Formats the output as TypeName(value) where value is lowercase.
//! * `Custom string`: Allows for a custom format string
//!
//! ### Examples
//! ```rust
//! #[derive(Synonym)]
//! #[synonym(display = "UpperCase")]
//! struct CountryName(String);
//!
//! #[derive(Synonym)]
//! #[synonym(display = "::<> {} <>::")]
//! struct Turbo(String);
//! ```
//!
//! ## Serde Support
//!
//! To enable Serde support for serialization and deserialization, you'll need to enable the `with_serde` feature flag in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! synonym = { version = "0.1.3", features = ["with_serde"] }
//! ```
//!
//! With this feature enabled, the `Serialize` and `Deserialize` traits will be automatically implemented for your type.
//!
//! ---
//! This documentation was generated with the assistance of ChatGPT-4 by OpenAI.

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
        impl_display(&info),
        impl_string(&info),
        impl_number(&info),
        impl_serialize(&info),
        impl_deserialize(&info),
    ]);

    TokenStream::from(expanded)
}
