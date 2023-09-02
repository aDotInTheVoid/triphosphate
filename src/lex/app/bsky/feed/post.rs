// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
///Deprecated: use facets instead.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Entity {
    pub index: _lex::app::bsky::feed::post::TextSlice,
    #[serde(rename = "type")]
    ///Expected values are 'mention' and 'link'.
    pub type_: ::std::string::String,
    pub value: ::std::string::String,
}
impl _lex::_rt::LexItem for Entity {
    const URI: &'static str = "app.bsky.feed.post#entity";
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct ReplyRef {
    pub parent: _lex::com::atproto::repo::StrongRef,
    pub root: _lex::com::atproto::repo::StrongRef,
}
impl _lex::_rt::LexItem for ReplyRef {
    const URI: &'static str = "app.bsky.feed.post#replyRef";
}

///Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct TextSlice {
    pub end: u64,
    pub start: u64,
}
impl _lex::_rt::LexItem for TextSlice {
    const URI: &'static str = "app.bsky.feed.post#textSlice";
}
