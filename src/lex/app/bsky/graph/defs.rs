// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct ListItemView {
    pub subject: _lex::app::bsky::actor::defs::ProfileView,
}
impl _lex::_rt::LexItem for ListItemView {
    const URI: &'static str = "app.bsky.graph.defs#listItemView";
}

pub type ListPurpose = ::std::string::String;
impl _lex::_rt::LexItem for ListPurpose {
    const URI: &'static str = "app.bsky.graph.defs#listPurpose";
}

#[derive(::std::fmt::Debug, ::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct ListView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<::std::string::String>,
    pub cid: _lex::_rt::Cid,
    pub creator: _lex::app::bsky::actor::defs::ProfileView,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::std::string::String>,
    #[serde(rename = "descriptionFacets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<_lex::app::bsky::richtext::Facet>>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: _lex::_rt::Datetime,
    pub name: ::std::string::String,
    pub purpose: _lex::app::bsky::graph::defs::ListPurpose,
    pub uri: _lex::_rt::AtUri,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<_lex::app::bsky::graph::defs::ListViewerState>,
}
impl _lex::_rt::LexItem for ListView {
    const URI: &'static str = "app.bsky.graph.defs#listView";
}

#[derive(::std::fmt::Debug, ::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct ListViewBasic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<::std::string::String>,
    pub cid: _lex::_rt::Cid,
    #[serde(rename = "indexedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<_lex::_rt::Datetime>,
    pub name: ::std::string::String,
    pub purpose: _lex::app::bsky::graph::defs::ListPurpose,
    pub uri: _lex::_rt::AtUri,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer: Option<_lex::app::bsky::graph::defs::ListViewerState>,
}
impl _lex::_rt::LexItem for ListViewBasic {
    const URI: &'static str = "app.bsky.graph.defs#listViewBasic";
}

#[derive(::std::fmt::Debug, ::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct ListViewerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
}
impl _lex::_rt::LexItem for ListViewerState {
    const URI: &'static str = "app.bsky.graph.defs#listViewerState";
}

///A list of actors to apply an aggregate moderation action (mute/block) on
#[derive(::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct Modlist;
impl _lex::_rt::LexItem for Modlist {
    const URI: &'static str = "app.bsky.graph.defs#modlist";
}
