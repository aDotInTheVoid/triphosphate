// Code generated by triphosphate lexgen. DO NOT EDIT.
pub mod get_head;

#[allow(unused_imports)]
use super::super::super::_lex;
///Gets the current HEAD CID of a repo.
pub async fn get_head(
    client: &_lex::_rt::Client,
    args: &_lex::com::atproto::sync::get_head::Params,
) -> _lex::_rt::Result<_lex::com::atproto::sync::get_head::Responce> {
    client.do_query("com.atproto.sync.getHead", args).await
}