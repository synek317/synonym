use synonym::Synonym;

#[derive(Synonym)]
struct Foo(char);

fn main() {
    check_partial_eq(Foo('x'));
    check_eq(Foo('x'));
    check_partial_ord(Foo('x'));
    check_ord(Foo('x'));
    check_clone(Foo('x'));
    check_copy(Foo('x'));
    check_hash(Foo('x'));
    check_default(Foo('x'));
    check_debug(Foo('x'));
    check_as_ref(Foo('x'));
    check_from(Foo('x'));
    check_from_inner('x');
    check_from_str(Foo('x'));
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
fn check_as_ref(_: impl AsRef<char>) {}
fn check_from(_: impl From<char>) {}
fn check_from_inner(_: impl From<Foo>) {}
fn check_from_str(_: impl core::str::FromStr) {}
