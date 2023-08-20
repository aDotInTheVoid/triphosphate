use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::{self, Debug},
    mem,
    sync::Arc,
};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::lexicon::{self, Array, Token, UserType, XrpcBody, XrpcBodySchema, XrpcParameters};

use self::field::FieldType;

mod field;

struct Compiler {
    doc: Arc<lexicon::LexiconDoc>,
    items: BTreeMap<ItemPath, TokenStream>,
}

pub(crate) fn lower_lexicon(
    lex: lexicon::LexiconDoc,
) -> BTreeMap<ItemPath, proc_macro2::TokenStream> {
    let lex = Arc::new(lex);

    let mut compiler = Compiler {
        doc: Arc::clone(&lex),
        items: BTreeMap::new(),
    };

    for (name, ty) in &lex.defs {
        let path = compiler.path_for_def(name, ty.item_kind());

        compiler.lower_item(&path, ty, name);
    }

    compiler.items
}

impl Compiler {
    fn insert_item(&mut self, p: &ItemPath, toks: proc_macro2::TokenStream) {
        insert_new(&mut self.items, p.clone(), toks);
    }

    fn lower_item(&mut self, path: &ItemPath, ty: &lexicon::UserType, lex_name: &str) {
        let item = match ty {
            UserType::Record(r) => self.lower_record(path, r),
            UserType::Object(o) => self.lower_object(path, o, &o.description),
            UserType::Array(arr) => self.lower_array(path, arr),
            UserType::String(str) => self.lower_string(path, str),
            UserType::Token(t) => self.lower_token(path, t),
            UserType::Query(q) => {
                // lower_query get's the URL of the method to call from the id
                // of the lexicon document, so THERE CAN ONLY BE ONE.
                assert_eq!(lex_name, "main");
                self.lower_query(path, q)
            }
            UserType::Procedure(p) => {
                assert_eq!(lex_name, "main");
                self.lower_procedure(path, p)
            }
            _ => todo!("lower_item: {ty:?}"),
        };

        let toks = match ty.item_kind() {
            ItemKind::Func => item,
            ItemKind::Type => {
                let name = path.name();
                let uri = self.uri(lex_name);

                quote! {
                    #item
                    impl _lex::_rt::LexItem for #name {
                        const URI: &'static str = #uri;
                    }
                }
            }
        };

        self.insert_item(path, toks);
    }

    fn lower_record(&self, path: &ItemPath, r: &lexicon::Record) -> proc_macro2::TokenStream {
        self.lower_object(path, &r.record, &r.description)
    }

    fn lower_object(
        &self,
        path: &ItemPath,
        o: &lexicon::Object,
        desc: &Option<String>,
    ) -> proc_macro2::TokenStream {
        let name = path.name();
        let doc = doc_comment(desc);

        let fields = o
            .properties
            .iter()
            .map(|(name, prop)| self.lower_field(name, prop, o));

        quote!(
            #doc
            #[derive(::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
            pub struct #name {
                #(#fields),*
            }
        )
    }

    fn lower_array(&self, path: &ItemPath, arr: &Array) -> TokenStream {
        let op = &lexicon::ObjectProperty::Array(arr.clone());

        let (field, desc) = FieldType::from_prop(op, &self.doc.id);

        let name = path.name();

        let docs = doc_comment(desc);

        quote!(
            #docs
            pub type #name = ::std::vec::Vec<#field>;
        )
        .to_token_stream()
    }

    fn lower_string(&self, path: &ItemPath, str: &lexicon::LexString) -> TokenStream {
        let docs = doc_comment(&str.description);
        let name = path.name();

        // TODO: Use fields.

        quote!(
            #docs
            pub type #name = ::std::string::String;
        )
        .to_token_stream()
    }

    fn lower_token(&self, path: &ItemPath, tk: &Token) -> TokenStream {
        let name = path.name();
        let docs = doc_comment(&tk.description);

        quote!(
            #docs
            #[derive(::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
            pub struct #name;
        )
        .to_token_stream()
    }

    fn lower_query(&mut self, path: &ItemPath, query: &lexicon::XrpcQuery) -> TokenStream {
        let name = path.name();

        let docs = doc_comment(&query.description);

        let params_type = self.xrpc_parameter(&query.parameters, path);

        let ret_type = self.xrpc_body(&query.output, path, "Responce");
        let params_ty = params_type.unwrap();
        let xrpc_id = &self.doc.id;

        // TODO: Error handling.
        quote! {
            #docs
            pub async fn #name(client: &_lex::_rt::Client, args: &_lex::#params_ty) -> ::reqwest::Result<_lex::#ret_type> {
                client.do_query(#xrpc_id, args).await
            }
        }
    }

    fn lower_procedure(
        &mut self,
        path: &ItemPath,
        proc: &lexicon::XprcProcedure,
    ) -> proc_macro2::TokenStream {
        let docs = doc_comment(&proc.description);
        let name = path.name();

        assert_eq!(proc.parameters, None);

        let output_type = self.xrpc_body(&proc.output, path, "Responce");
        let input_type = self.xrpc_body(&proc.input, path, "Args");
        let xrpc_id = &self.doc.id;

        // TODO: Error Handling

        quote! {
            #docs
            pub async fn #name(client: &_lex::_rt::Client, args: &_lex::#input_type) -> ::reqwest::Result<_lex::#output_type> {
                client.do_procedure(#xrpc_id, args).await
            }
        }
    }

    fn xrpc_parameter(
        &mut self,
        params: &Option<XrpcParameters>,
        path: &ItemPath,
    ) -> Option<ItemPath> {
        if let Some(params) = params {
            let object = conv::params_as_object(params.clone());

            let params_path = path.extend("Params");

            let obj = self.lower_object(&params_path, &object, &object.description);

            self.insert_item(&params_path, obj);

            Some(params_path)
        } else {
            None
        }
    }

    fn xrpc_body(&mut self, body: &Option<XrpcBody>, xrpc_path: &ItemPath, kind: &str) -> ItemPath {
        if let Some(resp) = body {
            match &resp.schema {
                Some(XrpcBodySchema::Object(o)) => {
                    assert_eq!(resp.encoding, "application/json");

                    let path = xrpc_path.extend(kind);
                    let resp = self.lower_object(&path, o, &o.description);
                    self.insert_item(&path, resp);
                    path
                }
                other => todo!("{other:?}"),
            }
        } else {
            todo!();
        }
    }
}

/////
// Vocab Types
/////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) struct Mod(pub Vec<String>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) struct ItemPath(pub Mod, pub String);

impl fmt::Display for ItemPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ItemPath(Mod(mod_), name) = self;
        for m in mod_ {
            write!(f, "{m}::")?;
        }
        write!(f, "{name}")
    }
}

impl ItemPath {
    fn name(&self) -> Ident {
        ident(&self.1)
    }

    fn extend(&self, extra: &str) -> Self {
        let mut this = self.clone();

        this.0 .0.push(mem::replace(&mut this.1, extra.to_owned()));

        this
    }
}

enum ItemKind {
    Type,
    Func,
}

impl lexicon::UserType {
    fn item_kind(&self) -> ItemKind {
        match self {
            lexicon::UserType::Query(_)
            | lexicon::UserType::Procedure(_)
            | lexicon::UserType::Subscription(_) => ItemKind::Func,

            _ => ItemKind::Type,
        }
    }
}

fn path_for_def(lex_id: &str, def_name: &str, kind: ItemKind) -> ItemPath {
    let parts: Vec<&str> = lex_id.split('.').collect();

    let (mod_parts, main_part) = if def_name == "main" {
        (&parts[..parts.len() - 1], parts[parts.len() - 1])
    } else {
        (parts.as_slice(), def_name)
    };

    ItemPath(
        Mod(mod_parts.iter().map(|s| snake(s)).collect()),
        match kind {
            ItemKind::Type => pascal(main_part),
            ItemKind::Func => snake(main_part),
        },
    )
}
impl Compiler {
    fn path_for_def(&self, def_name: &str, kind: ItemKind) -> ItemPath {
        path_for_def(&self.doc.id, def_name, kind)
    }

    fn uri(&self, name: &str) -> String {
        if name == "main" {
            self.doc.id.clone()
        } else {
            format!("{}#{name}", self.doc.id)
        }
    }
}

impl quote::ToTokens for ItemPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ItemPath(Mod(mod_), name) = self;
        let mod_ = mod_.iter().map(|s| ident(s));
        let name = ident(name);
        (quote! { #(#mod_::)* #name }).to_tokens(tokens);
    }
}

/////
// Utils
/////

fn snake(s: &str) -> String {
    heck::ToSnakeCase::to_snake_case(s)
}
fn pascal(s: &str) -> String {
    heck::ToPascalCase::to_pascal_case(s)
}
#[track_caller]
pub fn insert_new<K: Ord + Debug, V>(m: &mut BTreeMap<K, V>, k: K, v: V) {
    match m.entry(k) {
        Entry::Vacant(e) => {
            e.insert(v);
        }
        Entry::Occupied(e) => {
            panic!("duplicate key: {:?}", e.key())
        }
    }
}

fn ident(s: &str) -> Ident {
    format_ident!("{}", s)
}

fn doc_comment(desc: &Option<String>) -> proc_macro2::TokenStream {
    match desc {
        Some(desc) => {
            let desc = desc.trim();
            quote! { #[doc = #desc] }
        }
        None => quote! {},
    }
}

mod conv;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_path_for_def() {
        #[track_caller]
        fn check(lex_id: &str, def_name: &str, kind: ItemKind, expected: &str) {
            let path = path_for_def(lex_id, def_name, kind);
            let path_s = path.to_string();
            assert_eq!(path_s, expected);
        }

        check(
            "app.bsky.actor.searchActorsTypeahead",
            "main",
            ItemKind::Func,
            "app::bsky::actor::search_actors_typeahead",
        );

        // I don't think any functions occor at non-main, but just in case.
        check(
            "foo.bar.baz",
            "doSomething",
            ItemKind::Func,
            "foo::bar::baz::do_something",
        );

        check(
            "app.bsky.actor.defs",
            "profileViewDetailed",
            ItemKind::Type,
            "app::bsky::actor::defs::ProfileViewDetailed",
        );

        check(
            "app.bsky.actor.profile",
            "main",
            ItemKind::Type,
            "app::bsky::actor::Profile",
        );
    }
}
