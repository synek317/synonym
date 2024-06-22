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
    BoxStr,
    StaticStr,
    Char,
    Other,
}

impl Kind {
    pub fn is_clone(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_copy(&self) -> bool {
        match self {
            Kind::Integer | Kind::Float | Kind::Char => true,
            Kind::String | Kind::BoxStr | Kind::StaticStr | Kind::Other => false,
        }
    }

    pub fn is_debug(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_deserialize(&self) -> bool {
        match self {
            Kind::Integer | Kind::Float | Kind::String | Kind::BoxStr | Kind::Char => true,
            Kind::Other | Kind::StaticStr => false,
        }
    }

    pub fn is_eq(&self) -> bool {
        match self {
            Kind::Integer | Kind::String | Kind::BoxStr | Kind::StaticStr | Kind::Char => true,
            Kind::Float | Kind::Other => false,
        }
    }

    pub fn is_hash(&self) -> bool {
        match self {
            Kind::Integer | Kind::String | Kind::BoxStr | Kind::StaticStr | Kind::Char => true,
            Kind::Float | Kind::Other => false,
        }
    }

    pub fn is_ord(&self) -> bool {
        match self {
            Kind::Integer | Kind::String | Kind::BoxStr | Kind::StaticStr | Kind::Char => true,
            Kind::Float | Kind::Other => false,
        }
    }

    pub fn is_partial_eq(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_partial_ord(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_serialize(&self) -> bool {
        match self {
            Kind::Integer | Kind::Float | Kind::String | Kind::BoxStr | Kind::Char => true,
            Kind::Other | Kind::StaticStr => false,
        }
    }

    pub fn is_display(&self) -> bool {
        match self {
            Kind::Integer
            | Kind::Float
            | Kind::String
            | Kind::BoxStr
            | Kind::StaticStr
            | Kind::Char => true,
            Kind::Other => false,
        }
    }

    pub fn is_from_str(&self) -> bool {
        match self {
            Kind::Integer | Kind::Float | Kind::String | Kind::BoxStr | Kind::Char => true,
            Kind::Other | Kind::StaticStr => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Kind::Integer | Kind::Float => true,
            Kind::String | Kind::BoxStr | Kind::StaticStr | Kind::Char | Kind::Other => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Kind::String | Kind::BoxStr | Kind::StaticStr => true,
            Kind::Integer | Kind::Float | Kind::Char | Kind::Other => false,
        }
    }
}
