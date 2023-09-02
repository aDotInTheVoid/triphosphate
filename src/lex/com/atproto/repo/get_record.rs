// Code generated by triphosphate lexgen. DO NOT EDIT.

#[allow(unused_imports)]
use super::super::super::super::_lex;
pub struct Params {
    ///The CID of the version of the record. If not specified, then return the most recent version.
    pub cid: Option<_lex::_rt::Cid>,
    ///The NSID of the record collection.
    pub collection: _lex::_rt::Nsid,
    ///The handle or DID of the repo.
    pub repo: _lex::_rt::AtIdentifier,
    ///The key of the record.
    pub rkey: ::std::string::String,
}
impl _lex::_rt::AsParams for Params {
    fn as_params(&self) -> Vec<(&'static str, String)> {
        let mut r: Vec<(&'static str, String)> = Vec::with_capacity(3usize);
        if let Some(cid) = &self.cid {
            r.push(("cid", _lex::_rt::StringFormat::as_str(cid).to_owned()));
        }
        {
            let collection = &self.collection;
            r.push((
                "collection",
                _lex::_rt::StringFormat::as_str(collection).to_owned(),
            ));
        }
        {
            let repo = &self.repo;
            r.push(("repo", _lex::_rt::StringFormat::as_str(repo).to_owned()));
        }
        {
            let rkey = &self.rkey;
            r.push(("rkey", rkey.clone()));
        }
        r
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<_lex::_rt::Cid>,
    pub uri: _lex::_rt::AtUri,
    pub value: _lex::_rt::Any,
}
