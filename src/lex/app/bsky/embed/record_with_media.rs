// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(::std::clone::Clone, ::serde::Deserialize, ::serde::Serialize)]
pub struct View {
    pub media: (),
    pub record: _lex::app::bsky::embed::record::View,
}
impl _lex::_rt::LexItem for View {
    const URI: &'static str = "app.bsky.embed.recordWithMedia#view";
}
