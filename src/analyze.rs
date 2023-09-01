use crate::attrs::Attrs;
use crate::info::{Info, Kind};
use darling::FromDeriveInput;
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

                match info.typ {
                    syn::Type::Path(ref path)
                        => {
                        if let Some(type_name) =
                            path.path.segments.first().map(|ps| ps.ident.to_string())
                        {
                            info.kind = match &*type_name {
                                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16"
                                | "i32" | "i64" | "i128" | "isize" => Kind::Integer,
                                "f32" | "f64" => Kind::Float,
                                "String" => Kind::String,
                                "char" => Kind::Char,
                                _ => Kind::Other,
                            }
                        }
                    }
                    _ => return None
                }
            }
            _ => return None
        },
        _ => return None
    }

    Some(info)
}

fn has_generics(input: &DeriveInput) -> bool {
    input.generics.lt_token.is_some()
}
