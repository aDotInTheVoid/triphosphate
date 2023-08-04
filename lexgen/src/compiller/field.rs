use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::lexicon::{ObjectProperty, StringFormat};

use super::{doc_comment, ident, path_for_def, snake, ItemPath};

pub(super) fn lower_field(
    name: &str,
    prop: &ObjectProperty,
    o: &crate::lexicon::Object,
    doc_id: &str,
) -> Field {
    let required = o.required.contains(&name.to_owned());

    let (ty, docs) = FieldType::from_prop(prop, doc_id);

    Field {
        name: name.to_owned(),
        docs: docs.to_owned(),
        ty,
        required,
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
        let ty = if self.required {
            self.ty.to_token_stream()
        } else {
            let ty = &self.ty;
            quote!(Option<#ty>)
        };

        let (name, serde_name) = field_name(&self.name);

        let doc = doc_comment(&self.docs);

        quote!(
            #serde_name
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

enum FieldType {
    Ref(ItemPath),

    Unit,
    RtType(StringFormat),
    StdString, // TODO: Remove

    U8,
    U16,
    U32,
    U64,
}

impl FieldType {
    fn from_prop<'a>(prop: &'a ObjectProperty, doc_id: &str) -> (Self, &'a Option<String>) {
        match prop {
            ObjectProperty::Ref(path) => {
                (FieldType::Ref(type_ref(path, doc_id)), &path.description)
            }
            ObjectProperty::String(s) => (Self::str(s), &s.description),

            ObjectProperty::Integer(i) => (Self::int(i), &i.description),
            ObjectProperty::Union(u) => (FieldType::Unit, &u.description),
            ObjectProperty::Array(a) => (FieldType::Unit, &a.description),

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
        if i.minimum == Some(0) {
            if i.maximum == None {
                FieldType::U64 // Sensible fallback.
            } else {
                todo!()
            }
        } else {
            todo!();
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

            FieldType::StdString => quote!(::std::string::String).to_tokens(tokens),

            FieldType::U8 => quote!(u8).to_tokens(tokens),
            FieldType::U16 => quote!(u16).to_tokens(tokens),
            FieldType::U32 => quote!(u32).to_tokens(tokens),
            FieldType::U64 => quote!(u64).to_tokens(tokens),
        }
    }
}
