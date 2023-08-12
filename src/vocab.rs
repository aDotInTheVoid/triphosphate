use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct AtUri;

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

mod datetime;

pub use datetime::Datetime;
