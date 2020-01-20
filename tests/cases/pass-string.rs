use synonym::Synonym;

#[derive(Synonym)]
struct Foo(String);

fn main() {
    check_partial_eq(Foo("x".to_string()));
    check_eq(Foo("x".to_string()));
    check_partial_ord(Foo("x".to_string()));
    check_ord(Foo("x".to_string()));
    check_clone(Foo("x".to_string()));
    check_hash(Foo("x".to_string()));
    check_default(Foo("x".to_string()));
    check_debug(Foo("x".to_string()));
    check_as_ref(Foo("x".to_string()));
    check_from(Foo("x".to_string()));
    check_from_inner("x".to_string());
    check_from_str(Foo("x".to_string()));
}

fn check_partial_eq(_: impl PartialEq) {}
fn check_eq(_: impl Eq) {}
fn check_partial_ord(_: impl PartialOrd) {}
fn check_ord(_: impl Ord) {}
fn check_clone(_: impl Clone) {}
fn check_hash(_: impl core::hash::Hash) {}
fn check_default(_: impl Default) {}
fn check_debug(_: impl core::fmt::Debug) {}
fn check_as_ref(_: impl AsRef<String>) {}
fn check_from(_: impl From<String>) {}
fn check_from_inner(_: impl From<Foo>) {}
fn check_from_str(_: impl core::str::FromStr) {}
