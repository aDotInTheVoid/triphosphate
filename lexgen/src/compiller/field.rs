use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::lexicon::{
    ArrayItem, Boolean, Bytes, CidLink, ObjectProperty, ParameterProperty, StringFormat,
    XrpcParameters,
};

use super::{doc_comment, ident, path_for_def, snake, Compiler, ItemPath};

impl Compiler {
    pub(super) fn lower_obj_prop(
        &self,
        name: &str,
        prop: &ObjectProperty,
        o: &crate::lexicon::Object,
    ) -> Field {
        let required = o.required.contains(&name.to_owned());

        let (ty, docs) = FieldType::from_obj_prop(prop, &self.doc.id);

        Field {
            name: name.to_owned(),
            docs: docs.to_owned(),
            ty,
            required,
            use_serde: true,
        }
    }

    pub(super) fn lower_param_prop(
        &self,
        name: &str,
        prop: &ParameterProperty,
        ps: &XrpcParameters,
    ) -> Field {
        let required = ps.required.contains(&name.to_owned());

        let (ty, docs) = FieldType::from_param_prop(prop);

        Field {
            name: name.to_owned(),
            docs: docs.to_owned(),
            required,
            ty,
            use_serde: false,
        }
    }
}

#[derive(Debug)]
pub(super) struct Field {
    pub name: String,
    docs: Option<String>,
    pub required: bool,
    pub ty: FieldType,
    use_serde: bool,
}

impl quote::ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (ty, serde_optional) = if self.required {
            (self.ty.to_token_stream(), quote!())
        } else {
            let ty = &self.ty;
            (
                quote!(Option<#ty>),
                if self.use_serde {
                    quote!(#[serde(default, skip_serializing_if = "Option::is_none")])
                } else {
                    quote!()
                },
            )
        };

        let (name, serde_rename) = field_name(&self.name);

        let doc = doc_comment(&self.docs);

        quote!(
            #serde_rename
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

#[derive(Debug)]
pub(super) enum FieldType {
    Ref(ItemPath),

    Vec(Box<FieldType>),

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
    CidLink,
    Bytes,
}

impl FieldType {
    pub fn from_obj_prop<'a>(prop: &'a ObjectProperty, doc_id: &str) -> (Self, &'a Option<String>) {
        match prop {
            ObjectProperty::Ref(path) => Self::ref_(path, doc_id),
            ObjectProperty::Boolean(b) => Self::bool(b),
            ObjectProperty::String(s) => Self::string(s),
            ObjectProperty::Integer(i) => Self::integer(i),
            ObjectProperty::Bytes(b) => Self::bytes(b),
            ObjectProperty::CidLink(c) => Self::cid_link(c),

            ObjectProperty::Array(a) => Self::array(a, doc_id),

            // TODO: Blob details.
            ObjectProperty::Blob(b) => (FieldType::Blob, &b.description),

            // TODO: Implement.
            ObjectProperty::Union(u) => (FieldType::Unit, &u.description),
            ObjectProperty::Unknown(u) => (FieldType::Unknown, &u.description),
        }
    }

    fn from_param_prop(prop: &ParameterProperty) -> (Self, &Option<String>) {
        match prop {
            ParameterProperty::Boolean(b) => Self::bool(b),
            ParameterProperty::Integer(i) => Self::integer(i),
            ParameterProperty::String(s) => Self::string(s),
            ParameterProperty::Unknown(_) => todo!(),
            ParameterProperty::Array(_) => todo!(),
        }
    }

    fn from_array_items<'a>(prop: &'a ArrayItem, doc_id: &str) -> (Self, &'a Option<String>) {
        match prop {
            ArrayItem::Boolean(b) => Self::bool(b),
            ArrayItem::Integer(i) => Self::integer(i),
            ArrayItem::String(s) => Self::string(s),
            ArrayItem::CidLink(c) => Self::cid_link(c),

            ArrayItem::Ref(r) => Self::ref_(r, doc_id),

            // TODO: This needs a major refractor so we can insert a enum into the compiller map here.
            ArrayItem::Union(_) => (Self::Unit, &None),

            _ => todo!("{prop:?}"),
        }
    }

    fn string(s: &crate::lexicon::LexString) -> (FieldType, &Option<String>) {
        let this = if let Some(format) = &s.format {
            FieldType::RtType(*format)
        } else {
            // TODO: Do more here.
            FieldType::StdString
        };

        (this, &s.description)
    }

    fn integer(i: &crate::lexicon::Integer) -> (Self, &Option<String>) {
        // Max int size is 64 bits when not stated: https://atproto.com/specs/data-model#data-types.
        let this = match i.minimum {
            Some(0) => match i.maximum {
                None => FieldType::U64,
                _ => todo!(),
            },
            None => match i.maximum {
                None => FieldType::I64,
                _ => todo!(),
            },
            _ => todo!(),
        };

        (this, &i.description)
    }

    fn bool(b: &Boolean) -> (Self, &Option<String>) {
        // TODO: Use default, const.
        (Self::Bool, &b.description)
    }

    fn cid_link(c: &CidLink) -> (Self, &Option<String>) {
        (Self::CidLink, &c.description)
    }

    fn bytes(b: &Bytes) -> (Self, &Option<String>) {
        // TODO: min and max lenght
        (Self::Bytes, &b.description)
    }

    fn array<'a>(a: &'a crate::lexicon::Array, doc_id: &str) -> (FieldType, &'a Option<String>) {
        let (inner, desc) = FieldType::from_array_items(&a.items, doc_id);

        assert_eq!(desc, &None); // TODO: What if both have docs??

        (FieldType::Vec(Box::new(inner)), &a.description)
    }

    fn ref_<'a>(path: &'a crate::lexicon::Ref, doc_id: &str) -> (FieldType, &'a Option<String>) {
        (FieldType::Ref(type_ref(path, doc_id)), &path.description)
    }
}

// TODO: Put on compiller.
pub(crate) fn type_ref(path: &crate::lexicon::Ref, doc_id: &str) -> ItemPath {
    let refpath = &path.r#ref;

    let (mod_, name) = refpath.split_once('#').unwrap_or((refpath, ""));

    let mod_ = if mod_.is_empty() { doc_id } else { mod_ };
    let name = if name.is_empty() { "main" } else { name };

    // TODO: We're just quessing the type, but it *probably* doesn't matter.
    let mut p = path_for_def(mod_, name, super::ItemKind::OtherType);
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
            FieldType::Bytes => quote!(_lex::_rt::Bytes).to_tokens(tokens),
            FieldType::Unknown => quote!(_lex::_rt::Unknown).to_tokens(tokens),
            FieldType::CidLink => quote!(_lex::_rt::CidLink).to_tokens(tokens),

            FieldType::StdString => quote!(::std::string::String).to_tokens(tokens),
            FieldType::Bool => quote!(bool).to_tokens(tokens),
            FieldType::U64 => quote!(u64).to_tokens(tokens),
            FieldType::I64 => quote!(i64).to_tokens(tokens),

            FieldType::Vec(inner) => {
                let inner = inner.to_token_stream();
                quote!(Vec<#inner>).to_tokens(tokens)
            }
        }
    }
}

impl Field {
    pub fn name(&self) -> Ident {
        field_name(&self.name).0
    }
}
