use crate::attrs::Attrs;
use crate::info::{Info, Kind};
use darling::FromDeriveInput;
use quote::ToTokens;
use syn::{Data, DeriveInput, Fields};

pub fn analyze(input: &DeriveInput) -> Option<Info> {
    if has_generics(input) {
        return None;
    }

    let mut info = Info {
        name: input.ident.clone(),
        kind: Kind::Other,
        typ: syn::Type::Never(syn::TypeNever {
            bang_token: syn::token::Not::default(),
        }),
        attrs: Attrs::from_derive_input(input).unwrap(),
    };

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return None;
                }

                info.typ = fields.unnamed.first().unwrap().ty.clone();
                if let syn::Type::Group(g) = info.typ {
                    info.typ = *g.elem;
                }

                info.kind = match info
                    .typ
                    .to_token_stream()
                    .to_string()
                    .replace("std :: string :: ", "")
                    .replace("std :: num :: ", "")
                    .replace("std :: primitive :: ", "")
                    .replace("core :: primitive :: ", "")
                    .as_str()
                {
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
                    | "i64" | "i128" | "isize" => Kind::Integer,
                    "NonZeroU8" | "NonZeroU16" | "NonZeroU32" | "NonZeroU64" | "NonZeroU128"
                    | "NonZeroUsize" | "NonZeroI8" | "NonZeroI16" | "NonZeroI32" | "NonZeroI64"
                    | "NonZeroI128" | "NonZeroIsize" => Kind::NonZeroInteger,
                    "f32" | "f64" => Kind::Float,
                    "String" => Kind::String,
                    "Box < str >" => Kind::BoxStr,
                    "& 'static str" => Kind::StaticStr,
                    "char" => Kind::Char,
                    _ => Kind::Other,
                };
            }
            _ => return None,
        },
        _ => return None,
    }

    Some(info)
}

fn has_generics(input: &DeriveInput) -> bool {
    input.generics.lt_token.is_some()
}
