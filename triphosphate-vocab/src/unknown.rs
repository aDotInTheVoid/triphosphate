use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};

/// An unholy fussion of [serde_json::Value] and [libipld::Ipld]
#[derive(Debug, Clone, PartialEq)]
pub enum Unknown {
    Null,
    Bool(bool),
    // ATProto data model doesn't have floats, or ints larget than 51bits.
    Integer(i64),
    String(String),
    Bytes(crate::Bytes),
    List(Vec<Unknown>),
    Map(BTreeMap<String, Unknown>),
    Link(crate::CidLink),
}

mod de;
mod decode;
mod encode;
mod ser;
#[cfg(test)]
mod tests;

pub fn to_unknown<T>(value: T) -> Result<Unknown, serde_json::Error>
where
    T: Serialize,
{
    value.serialize(ser::Serializer)
}

pub fn from_unknown<T>(u: Unknown) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    T::deserialize(u)
}
