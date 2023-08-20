pub mod client;
pub mod lex;

pub trait LexItem: serde::Serialize + serde::de::DeserializeOwned {
    const URI: &'static str;
}

pub trait LexRecord: LexItem {
    const NSID: triphosphate_vocab::Nsid;
}

pub trait AsParams {
    fn as_params(&self) -> Vec<(&'static str, String)>;
}

pub(crate) mod rt {
    pub use crate::client::Client;
    pub use crate::{AsParams, LexItem, LexRecord};
    pub use triphosphate_vocab::*;

    // TODO: Error handling.
    pub use anyhow::Result;
}
