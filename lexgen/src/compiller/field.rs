use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::lexicon::{ObjectProperty, StringFormat};

use super::{doc_comment, ident, path_for_def, snake, Compiler, ItemPath};

impl Compiler {
    pub(super) fn lower_field(
        &self,
        name: &str,
        prop: &ObjectProperty,
        o: &crate::lexicon::Object,
    ) -> Field {
        let required = o.required.contains(&name.to_owned());

        let (ty, docs) = FieldType::from_prop(prop, &self.doc.id);

        Field {
            name: name.to_owned(),
            docs: docs.to_owned(),
            ty,
            required,
        }
    }
}

pub(super) struct Field {
    name: String,
    docs: Option<String>,
    required: bool,
    ty: FieldType,
}

impl quote::ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (ty, serde_optional) = if self.required {
            (self.ty.to_token_stream(), quote!())
        } else {
            let ty = &self.ty;
            (
                quote!(Option<#ty>),
                quote!(#[serde(default, skip_serializing_if = "Option::is_none")]),
            )
        };

        let (name, serde_name) = field_name(&self.name);

        let doc = doc_comment(&self.docs);

        quote!(
            #serde_name
            #serde_optional
            #doc
            pub #name: #ty
        )
        .to_tokens(tokens);
    }
}

fn field_name(name: &str) -> (Ident, TokenStream) {
    if name == "type" {
        return (format_ident!("type_"), quote!(#[serde(rename = "type")]));
    }

    let name_snake = snake(name);
    let ident = ident(&name_snake);

    (
        ident,
        if name_snake == name {
            quote!()
        } else {
            quote!(#[serde(rename = #name)])
        },
    )
}

pub(super) enum FieldType {
    Ref(ItemPath),

    Unit,
    RtType(StringFormat),
    StdString, // TODO: Remove

    // U8,
    // U16,
    // U32,
    I64,
    U64,
    Blob,
    Unknown,
    Bool,
}

impl FieldType {
    pub fn from_prop<'a>(prop: &'a ObjectProperty, doc_id: &str) -> (Self, &'a Option<String>) {
        match prop {
            ObjectProperty::Ref(path) => {
                (FieldType::Ref(type_ref(path, doc_id)), &path.description)
            }

            ObjectProperty::String(s) => (Self::str(s), &s.description),
            ObjectProperty::Integer(i) => (Self::int(i), &i.description),
            ObjectProperty::Union(u) => (FieldType::Unit, &u.description),
            ObjectProperty::Array(a) => (FieldType::Unit, &a.description),

            // TODO: Handle default, const
            ObjectProperty::Boolean(b) => (FieldType::Bool, &b.description),

            ObjectProperty::Unknown(u) => (FieldType::Unknown, &u.description),

            // TODO: Blob details.
            ObjectProperty::Blob(b) => (FieldType::Blob, &b.description),

            _ => todo!("FieldType::from_prop: {prop:?}"),
        }
    }

    fn str(s: &crate::lexicon::LexString) -> FieldType {
        if let Some(format) = &s.format {
            return FieldType::RtType(*format);
        }

        // TODO: Do more here.

        FieldType::StdString
    }

    fn int(i: &crate::lexicon::Integer) -> FieldType {
        // if i.minimum == Some(0) {
        //     if i.maximum.is_none() {
        //         FieldType::U64 // Sensible fallback.
        //     } else {
        //         todo!()
        //     }
        // } else {
        //     todo!("{i:?}");
        // }

        // Max int size is 64 bits when not stated: https://atproto.com/specs/data-model#data-types.

        match i.minimum {
            Some(0) => match i.maximum {
                None => FieldType::U64,
                _ => todo!(),
            },
            None => match i.maximum {
                None => FieldType::I64,
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

fn type_ref(path: &crate::lexicon::Ref, doc_id: &str) -> ItemPath {
    let refpath = &path.r#ref;

    let (mod_, name) = refpath.split_once('#').unwrap_or((refpath, ""));

    let mod_ = if mod_.is_empty() { doc_id } else { mod_ };
    let name = if name.is_empty() { "main" } else { name };

    let mut p = path_for_def(mod_, name, super::ItemKind::Type);
    p.0 .0.insert(0, "_lex".to_owned());
    p
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldType::Ref(path) => path.to_tokens(tokens),

            FieldType::Unit => quote!(()).to_tokens(tokens),

            FieldType::RtType(format) => {
                let name = ident(&format!("{format:?}"));
                quote!(_lex::_rt::#name).to_tokens(tokens);
            }

            FieldType::Blob => quote!(_lex::_rt::Blob).to_tokens(tokens),
            FieldType::Unknown => quote!(_lex::_rt::Unknown).to_tokens(tokens),

            FieldType::StdString => quote!(::std::string::String).to_tokens(tokens),
            FieldType::Bool => quote!(bool).to_tokens(tokens),
            FieldType::U64 => quote!(u64).to_tokens(tokens),
            FieldType::I64 => quote!(i64).to_tokens(tokens),
        }
    }
}
