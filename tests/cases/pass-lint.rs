use synonym::Synonym;

/// Verifies that derived code is compatible with `#[deny(missing_docs)]`. See
/// https://github.com/synek317/synonym/pull/10 for further context.

/// Ordinary struct that triggers many synonym derivations
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct Plain(char);

// Custom methods (all are implemented for char/String)

/// Copy struct that triggers synonym custom methods
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct CustomMethodCopyTest(char);

/// Non-Copy struct that triggers synonym custom methods
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct CustomMethodNonCopyTest(String);

// Conversion (all are implemented for Box<str> with force attribute)

/// Struct that triggers all synonym conversion derivations
#[deny(missing_docs)]
#[derive(Synonym)]
#[synonym(force(Deref, DerefMut))]
pub struct CustomMethodTest(Box<str>);

// Fundamental traits (all are implemented for char)

/// Struct that triggers all synonym fundamental derivations
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct FundamentalTraitsTest(char);

// Comparison (all are implemented for char)

/// Struct that triggers all synonym comparison derivations
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct ComparisonTest(char);

// Serde (all are implemented for char)

#[cfg(any(test, feature = "with_serde"))]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IsSerde(char);

/// Struct that triggers all synonym Serde derivations
#[cfg(any(test, feature = "with_serde"))]
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct SerdeTest(IsSerde);

// Math (all are implemented for float)

/// Struct that triggers all synonym math derivations
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct IsNumberTest(f32);

fn main() {}
