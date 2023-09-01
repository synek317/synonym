use synonym::Synonym;

// see https://rust-lang.github.io/rust-clippy/master/index.html#/incorrect_clone_impl_on_copy_type
// this test only ensures that cargo clippy doesn't show a warning
#[test]
fn clone_derived_() {
    #[derive(Synonym)]
    struct Foo(u32);
}
