use synonym::Synonym;

#[derive(Synonym, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default, Debug)]
#[synonym(skip(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default, Debug, AsRef, From, FromStr))]
struct FooU32(u32);

impl core::convert::AsRef<u32> for FooU32 {
    fn as_ref(&self) -> &u32 {
        unimplemented!()
    }
}

impl core::convert::From<u32> for FooU32 {
    fn from(_t: u32) -> Self {
        unimplemented!()
    }
}

impl core::convert::From<FooU32> for u32 {
    fn from(_t: FooU32) -> Self {
        unimplemented!()
    }
}

impl ::core::str::FromStr for FooU32 {
    type Err = <u32 as ::core::str::FromStr>::Err;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[derive(Synonym)]
#[synonym(skip(String))]
struct FooString(String);

impl ::core::borrow::Borrow<str> for FooString {
    fn borrow(&self) -> &str {
        unimplemented!()
    }
}

impl<'a> ::core::convert::From<&'a str> for FooString {
    fn from(_s: &'a str) -> Self {
        unimplemented!()
    }
}

fn main() {
    let _ = FooU32(42);
}
