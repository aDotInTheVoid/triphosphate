use std::collections::BTreeMap;

use libipld::Ipld;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Cid, CidLink};

/// An unholy fussion of [serde_json::Value] and [libipld::Ipld]
#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    Null,
    Bool(bool),
    // ATProto data model doesn't have floats, or ints larget than 51bits.
    Integer(i64),
    String(String),
    Bytes(crate::Bytes),
    List(Vec<Any>),
    Map(BTreeMap<String, Any>),
    Link(crate::CidLink),
}

mod de;
mod decode;
mod encode;
mod ser;
#[cfg(test)]
mod tests;

pub fn to_any<T>(value: T) -> Result<Any, serde_json::Error>
where
    T: Serialize,
{
    value.serialize(ser::Serializer)
}

pub fn from_any<T>(u: Any) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    T::deserialize(u)
}

pub fn ipld_to_any(ipld: Ipld) -> Any {
    match ipld {
        Ipld::Null => Any::Null,
        Ipld::Bool(b) => Any::Bool(b),
        Ipld::Integer(i) => {
            if let Ok(i) = i64::try_from(i) {
                Any::Integer(i)
            } else {
                panic!("ATProto doesn't suport integers as large as {i}")
            }
        }
        Ipld::Float(_) => panic!("ATProto data model doens't support floats"),
        Ipld::String(s) => Any::String(s),
        Ipld::Bytes(bytes) => Any::Bytes(crate::Bytes { bytes }),
        Ipld::List(l) => Any::List(l.into_iter().map(ipld_to_any).collect()),
        Ipld::Map(m) => Any::Map(m.into_iter().map(|(x, y)| (x, ipld_to_any(y))).collect()),
        Ipld::Link(l) => Any::Link(CidLink {
            link: Cid::from_cid(l),
        }),
    }
}
