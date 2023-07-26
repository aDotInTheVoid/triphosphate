use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// https://github.com/bluesky-social/atproto/blob/main/packages/lexicon/src/types.ts

// TODO: Versioning
// Needs https://github.com/serde-rs/serde/issues/745 probably.
// https://github.com/serde-rs/serde/pull/2525

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Lexicon {
    #[serde(rename = "lexicon")]
    version: u32,

    id: String,

    defs: BTreeMap<String, DefKind>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
/**
```bnf
LexUserType =LexRecord
  | LexXrpcQuery
  | LexXrpcProcedure
  | LexXrpcSubscription
  | LexBlob
  | LexArray
  | LexToken
  | LexObject
  | LexBoolean
  | LexInteger
  | LexString
  | LexBytes
  | LexCidLink
  | LexUnknown
```
*/

enum DefKind {
    Array(Array),
    Blob(Blob),
    Bytes(Bytes),
    Boolean(Boolean),
    Integer(Integer),
    Object(Object),
    Procedure(XprcProcedure),
    Query(Query),
    Record(Record),
    Ref(Ref),
    String(LexString),
    Subscription,
    CidLink(CidLink),
    Token(Token),
    Union(Union),
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Integer {
    description: Option<String>,
    default: Option<i64>,
    minimum: Option<i64>,
    maximum: Option<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
enum StringFormat {
    Datetime,
    Uri,
    AtUri,
    Did,
    Handle,
    AtIdentifier,
    Nsid,
    Cid,
    Language,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct LexString {
    format: Option<StringFormat>,
    description: Option<String>,
    default: Option<String>,
    min_length: Option<u64>,
    max_length: Option<u64>,
    min_graphemes: Option<u64>,
    max_graphemes: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Object {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    required: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nullable: Vec<String>,

    // Actually not right, theirs a few less.
    properties: BTreeMap<String, DefKind>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bytes {
    description: Option<String>,
    min_length: Option<u64>,
    max_length: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CidLink {
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ref {
    description: Option<String>,
    r#ref: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blob {
    description: Option<String>,
    max_size: Option<u64>,
    accept: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    description: Option<String>,
    min_lenght: Option<u64>,
    max_length: Option<u64>,
    items: Box<DefKind>, // TODO: We can be more specific here.
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XprcProcedure {
    description: Option<String>,
    parameters: Option<XrpcParameters>,
    input: Option<XrpcBody>,
    output: Option<XrpcBody>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcParameters {
    description: Option<String>,
    required: Option<Vec<String>>,
    properties: BTreeMap<String, DefKind>, // TODO: We can be more specific here.
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcBody {
    description: Option<String>,
    encoding: String,
    schema: Option<Box<DefKind>>, // TODO: Percise union here
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Query {
    description: Option<String>,
    parameters: Option<XrpcParameters>,
    output: Option<XrpcBody>,
    errors: Option<Vec<XrpcError>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct XrpcError {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Record {
    description: Option<String>,
    key: Option<String>,
    record: Object,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Boolean {
    description: Option<String>,
    default: Option<bool>,
    r#const: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Union {
    description: Option<String>,
    refs: Vec<String>,
    closed: Option<bool>,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn all_lexicons() {
        let base_dir = env!("CARGO_MANIFEST_DIR");

        for i in crate::ALL_LEXICONS {
            let json_path = Path::new(base_dir).join("lexicons").join(i);
            let json = fs::read(&json_path).unwrap();

            let lex = serde_json::from_slice::<Lexicon>(&json);

            if let Err(e) = lex {
                panic!("can't read lexicon {i}: {e}");
            }
        }
    }
}
