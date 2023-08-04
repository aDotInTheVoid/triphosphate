// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[doc = "Deprecated: use facets instead."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct Entity {
    pub index: _lex::app::bsky::feed::post::TextSlice,
    #[serde(rename = "type")]
    #[doc = "Expected values are 'mention' and 'link'."]
    pub type_: ::std::string::String,
    pub value: ::std::string::String,
}
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct ReplyRef {
    pub parent: _lex::com::atproto::repo::StrongRef,
    pub root: _lex::com::atproto::repo::StrongRef,
}
#[doc = "Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct TextSlice {
    pub end: (),
    pub start: (),
}
