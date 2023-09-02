// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Args {
    ///Handle or other identifier supported by the server for the authenticating user.
    pub identifier: ::std::string::String,
    pub password: ::std::string::String,
}

#[derive(
    ::std::fmt::Debug,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::serde::Deserialize,
    ::serde::Serialize,
    ::libipld::DagCbor,
)]
pub struct Responce {
    #[serde(rename = "accessJwt")]
    pub access_jwt: ::std::string::String,
    pub did: _lex::_rt::Did,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<::std::string::String>,
    pub handle: _lex::_rt::Handle,
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: ::std::string::String,
}
