macro_rules! check {
    ($t:ty, $v:expr) => {
        {
            mod dummy {
                use synonym::Synonym;

                #[derive(Synonym)]
                pub struct Foo(pub $t);
            }

            use dummy::Foo;

            fn check_as_ref(_: impl AsRef<$t>) {}
            fn check_from(_: impl From<$t>) {}
            fn check_from_inner(_: impl From<Foo>) {}
            fn check_value(_: $t) {}

            check_partial_eq(Foo($v));
            check_partial_ord(Foo($v));
            check_clone(Foo($v));
            check_copy(Foo($v));
            check_default(Foo($v));
            check_debug(Foo($v));
            check_display(Foo($v));
            check_as_ref(Foo($v));
            check_from(Foo($v));
            check_from_inner($v);
            check_from_str(Foo($v));
            check_value(Foo($v).value());
            check_add(Foo($v));
            check_sub(Foo($v));
            check_mul(Foo($v));
            check_div(Foo($v));
            check_add_assign(Foo($v));
            check_sub_assign(Foo($v));
            check_mul_assign(Foo($v));
            check_div_assign(Foo($v));
        }
    }
}

fn main() {
    check!(f32, 1f32);
    check!(f64, 1f64);
}

fn check_partial_eq(_: impl PartialEq) {}
fn check_partial_ord(_: impl PartialOrd) {}
fn check_clone(_: impl Clone) {}
fn check_copy(_: impl Copy) {}
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
