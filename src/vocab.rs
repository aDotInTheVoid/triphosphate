use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Cid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Did;

#[derive(Clone, Deserialize, Serialize)]
pub struct Handle;

#[derive(Clone, Deserialize, Serialize)]
pub struct Uri;

#[derive(Clone, Deserialize, Serialize)]
pub struct Blob;

pub type Unknown = serde_json::Value; // TODO

// TODO: Should this be pub?
pub trait StringFormat: Sized {
    fn as_str(&self) -> &str;

    type Error;

    fn from_str(s: &str) -> Result<Self, Self::Error>;
}

macro_rules! serde_impls {
    ($name:path) => {
        impl ::serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                ::serde::Serialize::serialize($crate::vocab::StringFormat::as_str(self), serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        ::std::write!(f, "a string") // TODO: More specific
                    }

                    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                        $crate::vocab::StringFormat::from_str(v).map_err(E::custom)
                    }
                }

                ::serde::Deserializer::deserialize_str(deserializer, Visitor)
            }
        }
    };
}

mod at_uri;
mod datetime;
mod nsid;

pub use at_uri::AtUri;
pub use datetime::Datetime;
pub use nsid::Nsid;
