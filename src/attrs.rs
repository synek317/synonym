use darling::{FromDeriveInput, FromMeta};

#[derive(Default, Debug, FromDeriveInput)]
#[darling(default, attributes(synonym))]
pub struct Attrs {
    pub skip: ImplList,
    pub force: ImplList,
    #[darling(rename = "caseinsensitive")]
    pub case_insensitive: bool, // TODO
    #[darling(rename = "display", map = "parse_display_kind")]
    pub display: DisplayKind,
}

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
pub struct ImplList {
    #[darling(rename = "Eq")]
    pub eq: bool,
    #[darling(rename = "PartialEq")]
    pub partial_eq: bool,
    #[darling(rename = "Ord")]
    pub ord: bool,
    #[darling(rename = "PartialOrd")]
    pub partial_ord: bool,
    #[darling(rename = "Clone")]
    pub clone: bool,
    #[darling(rename = "Copy")]
    pub copy: bool,
    #[darling(rename = "Hash")]
    pub hash: bool,
    #[darling(rename = "Default")]
    pub default: bool,
    #[darling(rename = "Debug")]
    pub debug: bool,
    #[darling(rename = "Display")]
    pub display: bool,
    #[darling(rename = "FromStr")]
    pub from_str: bool,
    #[darling(rename = "AsRef")]
    pub as_ref: bool,
    #[darling(rename = "Deref")]
    pub deref: bool,
    #[darling(rename = "DerefMut")]
    pub deref_mut: bool,
    #[darling(rename = "From")]
    pub from: bool,
    #[darling(rename = "String")]
    pub string: bool,
    #[darling(rename = "Number")]
    pub number: bool,
    #[darling(rename = "Serialize")]
    pub serialize: bool,
    #[darling(rename = "Deserialize")]
    pub deserialize: bool,
}

#[derive(Debug, FromMeta, Default)]
pub enum DisplayKind {
    Opaque,
    #[default]
    Transparent,
    UpperCase,
    LowerCase,
    OpaqueUpperCase,
    OpaqueLowerCase,
    Custom(String),
}

fn parse_display_kind(s: String) -> DisplayKind {
    s.parse().unwrap()
}

impl core::str::FromStr for DisplayKind {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "opaque" => Ok(DisplayKind::Opaque),
            "transparent" => Ok(DisplayKind::Transparent),
            "uppercase" => Ok(DisplayKind::UpperCase),
            "lowercase" => Ok(DisplayKind::LowerCase),
            "opaquelowercase" | "lowercaseopaque" => Ok(DisplayKind::OpaqueLowerCase),
            "opaqueuppercase" | "uppercaseopaque" => Ok(DisplayKind::OpaqueUpperCase),
            _ => Ok(DisplayKind::Custom(s.to_string())),
        }
    }
}
