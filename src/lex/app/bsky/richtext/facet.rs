// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
///A text segment. Start is inclusive, end is exclusive. Indices are for utf8-encoded strings.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct ByteSlice {
    #[serde(rename = "byteEnd")]
    pub byte_end: u64,
    #[serde(rename = "byteStart")]
    pub byte_start: u64,
}
impl _lex::_rt::LexItem for ByteSlice {
    const URI: &'static str = "app.bsky.richtext.facet#byteSlice";
}

///A facet feature for links.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Link {
    pub uri: _lex::_rt::Uri,
}
impl _lex::_rt::LexItem for Link {
    const URI: &'static str = "app.bsky.richtext.facet#link";
}

///A facet feature for actor mentions.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Mention {
    pub did: _lex::_rt::Did,
}
impl _lex::_rt::LexItem for Mention {
    const URI: &'static str = "app.bsky.richtext.facet#mention";
}

///A hashtag.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Tag {
    pub tag: ::std::string::String,
}
impl _lex::_rt::LexItem for Tag {
    const URI: &'static str = "app.bsky.richtext.facet#tag";
}
