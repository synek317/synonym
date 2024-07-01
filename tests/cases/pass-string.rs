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
        fn check_from_string(_: impl From<String>) {}

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
    (non-static str; $t:ty, $v:expr) => {
        {
            check!($t, $v);
            fn check_from_str(_: impl core::str::FromStr) {}
            fn check_value(_: &$t) {}

            check_from_str(Foo($v));
            check_value(Foo($v).value());
            check_from_string(Foo($v));
        }
    };
    (static str; $t:ty, $v:expr) => {
        {
            check!($t, $v);
            fn check_copy(_: impl core::marker::Copy) {}
            fn check_value(_: $t) {}

            check_copy(Foo($v));
            check_value(Foo($v).value());
        }
    }
}

fn main() {
    check!(non-static str; String, "x".to_string());
    check!(non-static str; Box<str>, "x".to_string().into_boxed_str());
    check!(static str; &'static str, "x");
}