pub mod lex;
pub mod vocab;

pub trait LexItem: serde::Serialize + serde::de::DeserializeOwned {
    const URI: &'static str;
}
