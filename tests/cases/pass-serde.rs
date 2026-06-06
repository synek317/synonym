#[cfg(not(feature = "with_serde"))]
fn main() {
    panic!("This test requires the 'with_serde' feature to be enabled. Run with `cargo test --features with_serde` or `cargo test --all-features`");
}

#[cfg(feature = "with_serde")]
fn main() {
    use synonym::Synonym;
    use serde::{Serialize, Deserialize};

    macro_rules! check {
        ($t:ty, $v:expr, $json:expr) => {
            {
                #[derive(Synonym)]
                struct Foo($t);

                let foo = Foo($v);
                let json = serde_json::to_string(&foo).unwrap();
                let deserialized = serde_json::from_str(&json).unwrap();

                assert_eq!(foo, deserialized);
                assert_eq!(json, $json);
            }
        }
    }

    check!(u8, 1u8, "1");
    check!(u16, 2u16, "2");
    check!(u32, 3u32, "3");
    check!(u64, 4u64, "4");
    check!(u128, 5u128, "5");
    check!(usize, 6usize, "6");
    check!(usize, 7usize, "7");
    check!(String, "Foo".to_string(), r#""Foo""#);
    check!(Box<str>, "Foo".to_string().into_boxed_str(), r#""Foo""#);
    check!(char, 'X', r#""X""#);

    #[derive(Synonym)]
    struct Baz(u32);

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Foo {
        bar: Baz
    }

    let foo = Foo { bar: Baz(42) };
    let json = serde_json::to_string(&foo).unwrap();
    let deserialized = serde_json::from_str(&json).unwrap();

    assert_eq!(foo, deserialized);
    assert_eq!(json, r#"{"bar":42}"#);

    #[derive(Synonym)]
    #[synonym(force(Serialize), skip(Deserialize))]
    struct SkipDeserialize(u32);

    impl<'de> Deserialize<'de> for SkipDeserialize {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            u32::deserialize(deserializer).map(Self)
        }
    }

    let skipped = SkipDeserialize(8);
    let json = serde_json::to_string(&skipped).unwrap();
    let deserialized: SkipDeserialize = serde_json::from_str(&json).unwrap();

    assert_eq!(skipped, deserialized);
    assert_eq!(json, "8");

    #[derive(Synonym)]
    #[synonym(force(Deserialize), skip(Serialize))]
    struct ForceDeserialize(u32);

    let deserialized: ForceDeserialize = serde_json::from_str("9").unwrap();

    assert_eq!(deserialized, ForceDeserialize(9));
}
