use synonym::Synonym;

/// Verifies that derived code is compatible with `#[deny(missing_docs)]`. See
/// https://github.com/synek317/synonym/pull/10 for further context.

// Custom methods (all are implemented for char/String)

/// Copy struct that triggers synonym custom methods
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct CustomMethodCopyTest(char);

/// Non-Copy struct that triggers synonym custom methods
#[deny(missing_docs)]
#[derive(Synonym)]
pub struct CustomMethodNonCopyTest(String);

fn main() {}
