use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::{self, Debug},
    sync::Arc,
};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::lexicon::{self, UserType};

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

        let item = compiler.lower_item(&path, ty);

        insert_new(&mut compiler.items, path, item);
    }

    compiler.items
}

impl Compiler {
    fn lower_item(&self, path: &ItemPath, ty: &lexicon::UserType) -> proc_macro2::TokenStream {
        match ty {
            UserType::Record(r) => self.lower_record(path, r),
            UserType::Object(o) => self.lower_object(path, o, &o.description),
            _ => todo!("lower_item: {ty:?}"),
        }
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
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            pub struct #name {
                #(#fields),*
            }
        )
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
    match s {
        _ => format_ident!("{}", s),
    }
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
