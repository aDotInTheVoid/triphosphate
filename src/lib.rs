pub mod client;
pub mod lex;
pub mod vocab;

mod parsing;

pub trait LexItem: serde::Serialize + serde::de::DeserializeOwned {
    const URI: &'static str;
}

pub(crate) mod rt {
    pub use crate::client::Client;
    pub use crate::vocab::*;
    pub use crate::LexItem;
}
