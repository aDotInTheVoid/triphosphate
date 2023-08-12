// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(:: std :: clone :: Clone, :: serde :: Deserialize, :: serde :: Serialize)]
pub struct View {
    pub record: (),
}
impl _lex::_rt::LexItem for View {
    const URI: &'static str = "app.bsky.embed.record#view";
}
#[derive(:: std :: clone :: Clone, :: serde :: Deserialize, :: serde :: Serialize)]
pub struct ViewBlocked {
    pub author: _lex::app::bsky::feed::defs::BlockedAuthor,
    pub blocked: bool,
    pub uri: _lex::_rt::AtUri,
}
impl _lex::_rt::LexItem for ViewBlocked {
    const URI: &'static str = "app.bsky.embed.record#viewBlocked";
}
#[derive(:: std :: clone :: Clone, :: serde :: Deserialize, :: serde :: Serialize)]
pub struct ViewNotFound {
    #[serde(rename = "notFound")]
    pub not_found: bool,
    pub uri: _lex::_rt::AtUri,
}
impl _lex::_rt::LexItem for ViewNotFound {
    const URI: &'static str = "app.bsky.embed.record#viewNotFound";
}
#[derive(:: std :: clone :: Clone, :: serde :: Deserialize, :: serde :: Serialize)]
pub struct ViewRecord {
    pub author: _lex::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: _lex::_rt::Cid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<()>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: _lex::_rt::Datetime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<()>,
    pub uri: _lex::_rt::AtUri,
    pub value: _lex::_rt::Unknown,
}
impl _lex::_rt::LexItem for ViewRecord {
    const URI: &'static str = "app.bsky.embed.record#viewRecord";
}
