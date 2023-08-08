// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct View {
    pub record: (),
}
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct ViewBlocked {
    pub uri: _lex::_rt::AtUri,
}
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
pub struct ViewNotFound {
    pub uri: _lex::_rt::AtUri,
}
#[derive(:: serde :: Deserialize, :: serde :: Serialize)]
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