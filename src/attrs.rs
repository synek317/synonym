use darling::{FromDeriveInput, FromMeta};

#[derive(Default, Debug, FromDeriveInput)]
#[darling(default, attributes(synonym))]
pub struct Attrs {
    pub skip: ImplList,
    pub force: ImplList,
    #[darling(rename = "caseinsensitive")]
    pub case_insensitive: bool, // TODO
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
}
