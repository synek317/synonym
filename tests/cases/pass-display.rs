use synonym::Synonym;

#[derive(Synonym)]
struct TransparentString(String);

#[derive(Synonym)]
#[synonym(display = "LowerCase")]
struct LowerCaseString(String);

#[derive(Synonym)]
#[synonym(display = "UpperCase")]
struct UpperCaseString(String);

#[derive(Synonym)]
#[synonym(display = "opaque")]
struct OpaqueString(String);

#[derive(Synonym)]
#[synonym(display = "OpaqueLowerCase")]
struct OpaqueLowerCaseString(String);

#[derive(Synonym)]
#[synonym(display = "OpaqueUpperCase")]
struct OpaqueUpperCaseString(String);

#[derive(Synonym)]
#[synonym(display = "CustomFormat - {}")]
struct CustomString(String);

macro_rules! assert {
    ($expected:expr, $actual:expr) => (
        if $actual != $expected {
            panic!("Assertion failed! Expected: {}, got: {}", $expected, $actual)
        }
    )
}

fn main() {
    let transparent = TransparentString::from("transparent");
    let lower_case = LowerCaseString::from("Lower cAse");
    let upper_case = UpperCaseString::from("Upper Case");
    let opaque = OpaqueString::from("opaque");
    let opaque_lower_case = OpaqueLowerCaseString::from("Opaque Lower case");
    let opaque_upper_case = OpaqueUpperCaseString::from("opaque Upper Case");
    let custom = CustomString::from("custom");

    assert!("transparent", transparent.to_string());
    assert!("lower case", lower_case.to_string());
    assert!("UPPER CASE", upper_case.to_string());
    assert!("OpaqueString(opaque)", opaque.to_string());
    assert!("OpaqueLowerCaseString(opaque lower case)", opaque_lower_case.to_string());
    assert!("OpaqueUpperCaseString(OPAQUE UPPER CASE)", opaque_upper_case.to_string());
    assert!("CustomFormat - custom", custom.to_string());
}
