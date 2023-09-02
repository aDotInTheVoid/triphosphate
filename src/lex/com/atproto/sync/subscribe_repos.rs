// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Commit {
    pub blobs: Vec<_lex::_rt::CidLink>,
    ///CAR file containing relevant blocks
    pub blocks: _lex::_rt::Bytes,
    pub commit: _lex::_rt::CidLink,
    pub ops: Vec<_lex::com::atproto::sync::subscribe_repos::RepoOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<_lex::_rt::CidLink>,
    pub rebase: bool,
    pub repo: _lex::_rt::Did,
    ///The rev of the emitted commit
    pub rev: ::std::string::String,
    pub seq: i64,
    ///The rev of the last emitted commit from this repo
    pub since: ::std::string::String,
    pub time: _lex::_rt::Datetime,
    #[serde(rename = "tooBig")]
    pub too_big: bool,
}
impl _lex::_rt::LexItem for Commit {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#commit";
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Handle {
    pub did: _lex::_rt::Did,
    pub handle: _lex::_rt::Handle,
    pub seq: i64,
    pub time: _lex::_rt::Datetime,
}
impl _lex::_rt::LexItem for Handle {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#handle";
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Info {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<::std::string::String>,
    pub name: ::std::string::String,
}
impl _lex::_rt::LexItem for Info {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#info";
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Migrate {
    pub did: _lex::_rt::Did,
    #[serde(rename = "migrateTo")]
    pub migrate_to: ::std::string::String,
    pub seq: i64,
    pub time: _lex::_rt::Datetime,
}
impl _lex::_rt::LexItem for Migrate {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#migrate";
}

///A repo operation, ie a write of a single record. For creates and updates, cid is the record's CID as of this operation. For deletes, it's null.
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct RepoOp {
    pub action: ::std::string::String,
    pub cid: _lex::_rt::CidLink,
    pub path: ::std::string::String,
}
impl _lex::_rt::LexItem for RepoOp {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#repoOp";
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Tombstone {
    pub did: _lex::_rt::Did,
    pub seq: i64,
    pub time: _lex::_rt::Datetime,
}
impl _lex::_rt::LexItem for Tombstone {
    const URI: &'static str = "com.atproto.sync.subscribeRepos#tombstone";
}
