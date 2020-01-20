use crate::attrs::Attrs;
use syn::{Ident, Type};

#[derive(Debug)]
pub struct Info {
    pub name: Ident,
    pub kind: Kind,
    pub typ: Type,
    pub attrs: Attrs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Integer,
    Float,
    String,
    Char,
    Other,
}
