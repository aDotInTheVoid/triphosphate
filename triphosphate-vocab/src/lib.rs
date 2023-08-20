use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Uri;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blob;

pub type Unknown = serde_json::Value; // TODO

// TODO: Should this be pub?
pub trait StringFormat: Sized {
    fn as_str(&self) -> &str;

    type Error;

    fn from_str(s: &str) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub struct ParseError(());
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse NSID")
    }
}
impl std::error::Error for ParseError {}

mod at_identifer;
mod at_uri;
mod cid;
mod datetime;
mod did;
mod handle;
mod language;
mod nsid;
mod parsing;

pub use self::cid::Cid;
pub use at_identifer::AtIdentifier;
pub use at_uri::AtUri;
pub use datetime::Datetime;
pub use did::Did;
pub use handle::Handle;
pub use language::Language;
pub use nsid::Nsid;

macro_rules! serde_impls {
    ($($name:path)*) => {$(
        impl ::serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                ::serde::Serialize::serialize($crate::StringFormat::as_str(self), serializer)
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
                        $crate::StringFormat::from_str(v).map_err(E::custom)
                    }
                }

                ::serde::Deserializer::deserialize_str(deserializer, Visitor)
            }
        }
    )*};
}

serde_impls! {
    AtIdentifier
    AtUri
    Cid
    Datetime
    Did
    Handle
    Language
    Nsid
}
