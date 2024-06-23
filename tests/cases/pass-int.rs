use synonym::Synonym;
use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize};

macro_rules! check {
    ($t:ty, $v:expr) => {
        #[derive(Synonym)]
        struct Foo($t);

        fn check_as_ref(_: impl AsRef<$t>) {}
        fn check_from(_: impl From<$t>) {}
        fn check_from_inner(_: impl From<Foo>) {}

        check_partial_eq(Foo($v));
        check_eq(Foo($v));
        check_partial_ord(Foo($v));
        check_ord(Foo($v));
        check_clone(Foo($v));
        check_copy(Foo($v));
        check_hash(Foo($v));
        check_debug(Foo($v));
        check_display(Foo($v));
        check_as_ref(Foo($v));
        check_from(Foo($v));
        check_from_inner($v);
        check_from_str(Foo($v));
    };
    (builtin; $t:ty, $v:expr) => {
        {
            check!($t, $v);
            check_default(Foo($v));
            check_add(Foo($v));
            check_sub(Foo($v));
            check_mul(Foo($v));
            check_div(Foo($v));
            check_add_assign(Foo($v));
            check_sub_assign(Foo($v));
            check_mul_assign(Foo($v));
            check_div_assign(Foo($v));
        }
    };
    (nonzero; $t:ty, $v:expr) => {
        {
            check!($t, $v);
        }
    }
}

fn main() {
    check!(builtin; u8,    1u8);
    check!(builtin; u16,   1u16);
    check!(builtin; u32,   1u32);
    check!(builtin; u64,   1u64);
    check!(builtin; u128,  1u128);
    check!(builtin; usize, 1usize);

    check!(builtin; i8,    1i8);
    check!(builtin; i16,   1i16);
    check!(builtin; i32,   1i32);
    check!(builtin; i64,   1i64);
    check!(builtin; i128,  1i128);
    check!(builtin; isize, 1isize);

    check!(nonzero; NonZeroU8,    NonZeroU8::new(1).unwrap());
    check!(nonzero; NonZeroU16,   NonZeroU16::new(1).unwrap());
    check!(nonzero; NonZeroU32,   NonZeroU32::new(1).unwrap());
    check!(nonzero; NonZeroU64,   NonZeroU64::new(1).unwrap());
    check!(nonzero; NonZeroU128,  NonZeroU128::new(1).unwrap());
    check!(nonzero; NonZeroUsize, NonZeroUsize::new(1).unwrap());

    check!(nonzero; NonZeroI8,    NonZeroI8::new(1).unwrap());
    check!(nonzero; NonZeroI16,   NonZeroI16::new(1).unwrap());
    check!(nonzero; NonZeroI32,   NonZeroI32::new(1).unwrap());
    check!(nonzero; NonZeroI64,   NonZeroI64::new(1).unwrap());
    check!(nonzero; NonZeroI128,  NonZeroI128::new(1).unwrap());
    check!(nonzero; NonZeroIsize, NonZeroIsize::new(1).unwrap());
}

fn check_partial_eq(_: impl PartialEq) {}
fn check_eq(_: impl Eq) {}
fn check_partial_ord(_: impl PartialOrd) {}
fn check_ord(_: impl Ord) {}
fn check_clone(_: impl Clone) {}
fn check_copy(_: impl Copy) {}
fn check_hash(_: impl core::hash::Hash) {}
fn check_default(_: impl Default) {}
fn check_debug(_: impl core::fmt::Debug) {}
fn check_display(_: impl core::fmt::Display) {}
fn check_from_str(_: impl core::str::FromStr) {}
fn check_add<T: core::ops::Add<T>>(_: T) {}
fn check_sub<T: core::ops::Add<T>>(_: T) {}
fn check_mul<T: core::ops::Add<T>>(_: T) {}
fn check_div<T: core::ops::Add<T>>(_: T) {}
fn check_add_assign(_: impl core::ops::AddAssign) {}
fn check_sub_assign(_: impl core::ops::SubAssign) {}
fn check_mul_assign(_: impl core::ops::MulAssign) {}
fn check_div_assign(_: impl core::ops::DivAssign) {}