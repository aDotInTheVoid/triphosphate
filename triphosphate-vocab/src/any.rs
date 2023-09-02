use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};

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
