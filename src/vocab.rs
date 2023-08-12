use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AtUri;

#[derive(Deserialize, Serialize)]
pub struct Cid;

#[derive(Deserialize, Serialize)]
pub struct Did;

#[derive(Deserialize, Serialize)]
pub struct Handle;

#[derive(Deserialize, Serialize)]
pub struct Uri;

#[derive(Deserialize, Serialize)]
pub struct Blob;

pub type Unknown = serde_json::Value; // TODO

mod datetime;

pub use datetime::Datetime;

pub use crate::LexItem; // TODO: Seperate vocab from _lex::_rt;
