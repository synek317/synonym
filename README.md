[![Crates.io](https://img.shields.io/crates/v/synonym.svg)](https://crates.io/crates/synonym)
[![Docs.rs](https://docs.rs/synonym/badge.svg)](https://docs.rs/synonym)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
[![Build Status](https://travis-ci.org/synek317/synonym.svg?branch=master)](https://travis-ci.org/synek317/synonym)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# Synonym

## Overview

The `synonym` library is a Rust crate designed to simplify the creation of newtypes. It provides a customizable `#[derive(Synonym)]` macro that automatically implements various traits based on the underlying type of your newtype. This saves you from the boilerplate code usually required when defining newtypes.

## Usage

To use `synonym`, add it to your Cargo.toml:

```toml
[dependencies]
synonym = "0.1.0"
```

### Basic example

Import the `Synonym` trait into your Rust file:

```rust
use synonym::Synonym;
```

Then, define your newtype and annotate it with `#[derive(Synonym)]:`
```rust
#[derive(Synonym)]
pub struct MyInt(i32);
```

### Customization with Attributes
You can customize which traits are implemented or skipped using the `#[synonym(skip(...))]` and `#[synonym(force(...))]` attributes:
```rust
#[derive(Synonym)]
#[synonym(skip(Eq, PartialEq))]
pub struct MyString(String);
```

## Generated code
When you use `#[derive(Synonym)]`, the library generates implementations for various traits. Here's a simplified example for a newtype `MyInt(i32)`:
```rust
impl Eq for MyInt {}
impl PartialEq for MyInt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
// ... and so on for other traits
```

## Trait implementation table

| Kind    | Traits / Methods Implemented |
| ------- | ---------------------------- |
| Integer<br>`u8`, `u16`, `u32`, `u64`, `u128`, `usize`,<br>`i8`, `i16`, `i32`, `i64`, `i128`, `isize` |  `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Copy`, `Hash`, `Default`, `Debug`, `Add`, `Sub`, `Mul`, `Div`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `FromStr`, `From`, `AsRef`, `Deref` |
| Float<br>`f32`, `f64`   | `PartialEq`, `PartialOrd`, `Clone`, `Default`, `Debug`, `Add`, `Sub`, `Mul`, `Div`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `FromStr`, `From`, `AsRef`, `Deref` |
| String<br>`String`  | `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Hash`, `Default`, `Debug`, `FromStr`, `From`, `AsRef`, `Deref`, `Borrow<str>`, `as_str()` |
| Char<br>`char`    | `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Clone`, `Copy`, `Hash`, `Default`, `Debug`, `FromStr`, `From`, `AsRef`, `Deref` |

## License

Licensed under of MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contribution

All contributions and comments are more than welcome! Don't be afraid to open an issue or PR whenever you find a bug or have an idea to improve this crate.
