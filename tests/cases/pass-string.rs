use std::borrow::Borrow;

// define Foo in a module to make sure methods defined directly for Foo (e.g. as_str) are accessible

macro_rules! check {
    ($t:ty, $v:expr) => {
        mod dummy {
            use synonym::Synonym;

            #[derive(Synonym)]
            pub struct Foo(pub $t);
        }

        use dummy::Foo;

        fn check_partial_eq(_: impl PartialEq) {}
        fn check_eq(_: impl Eq) {}
        fn check_partial_ord(_: impl PartialOrd) {}
        fn check_ord(_: impl Ord) {}
        fn check_clone(_: impl Clone) {}
        fn check_hash(_: impl core::hash::Hash) {}
        fn check_default(_: impl Default) {}
        fn check_debug(_: impl core::fmt::Debug) {}
        fn check_display(_: impl core::fmt::Display) {}
        fn check_as_ref(_: impl AsRef<$t>) {}
        fn check_from(_: impl From<$t>) {}
        fn check_from_inner(_: impl From<Foo>) {}
        fn check_as_str(_: &str) {}

        check_partial_eq(Foo($v));
        check_eq(Foo($v));
        check_partial_ord(Foo($v));
        check_ord(Foo($v));
        check_clone(Foo($v));
        check_hash(Foo($v));
        check_default(Foo($v));
        check_debug(Foo($v));
        check_display(Foo($v));
        check_as_ref(Foo($v));
        check_from(Foo($v));
        check_from_inner($v);
        check_as_str(Foo($v).as_str());
        check_as_str(Foo($v).borrow());
    };
    (with from_str; $t:ty, $v:expr) => {
        {
            check!($t, $v);
            fn check_from_str(_: impl core::str::FromStr) {}
            check_from_str(Foo($v));
        }
    };
    (without from_str; $t:ty, $v:expr) => {
        {
            check!($t, $v);
        }
    }
}

fn main() {
    check!(with from_str; String, "x".to_string());
    check!(with from_str; Box<str>, "x".to_string().into_boxed_str());
    check!(without from_str; &'static str, "x");
}