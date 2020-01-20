use synonym::Synonym;

macro_rules! check {
    ($t:ty, $v:expr) => {
        {
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
            check_default(Foo($v));
            check_debug(Foo($v));
            check_as_ref(Foo($v));
            check_from(Foo($v));
            check_from_inner($v);
            check_from_str(Foo($v));
        }
    }
}

fn main() {
    check!(i8,    1i8);
    check!(i16,   1i16);
    check!(i32,   1i32);
    check!(i64,   1i64);
    check!(i128,  1i128);
    check!(isize, 1isize);
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
fn check_from_str(_: impl core::str::FromStr) {}
