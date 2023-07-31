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

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

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
    Subscription(XrpcSubscription),
    CidLink(CidLink),
    Token(Token),
    Union(Union),
    Unknown(Unknown),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Integer {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<StringFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_graphemes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_graphemes: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    known_values: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Object {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nullable: Vec<String>,
    properties: BTreeMap<String, DefKind>, // TODO: Percise Union here

    #[serde(default, rename = "type", skip_serializing_if = "String::is_empty")]
    _type: String, // Hacky hackity hack
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bytes {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CidLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ref {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    r#ref: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_lenght: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
    items: Box<DefKind>, // TODO: We can be more specific here.
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XprcProcedure {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<XrpcParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<XrpcBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<XrpcBody>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    errors: Vec<XrpcError>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
    properties: BTreeMap<String, DefKind>, // TODO: We can be more specific here.

    #[serde(rename = "type")]
    _type: String, // Hacky hackity hack
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Box<DefKind>>, // TODO: Percise union here
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<XrpcParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<XrpcBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<XrpcError>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct XrpcError {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    record: Object,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Boolean {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#const: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Union {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XrpcSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<XrpcParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<XrpcSubscriptionMessage>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    errors: Vec<XrpcError>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XrpcSubscriptionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    schema: Option<Box<DefKind>>, // TODO: Percise union here
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Unknown {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
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

            let lex = match serde_json::from_slice::<Lexicon>(&json) {
                Ok(lex) => lex,
                Err(e) => panic!("can't parse lexicon {i}: {e}"),
            };

            let json_roundtrip = serde_json::to_vec(&lex).unwrap();

            let json: serde_json::Value = serde_json::from_slice(&json).unwrap();
            let json_roundtrip: serde_json::Value =
                serde_json::from_slice(&json_roundtrip).unwrap();

            assert_eq!(
                json,
                json_roundtrip,
                "\nlexicon {i} different",
            );
        }
    }
}
