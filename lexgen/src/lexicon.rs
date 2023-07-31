use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// https://github.com/bluesky-social/atproto/blob/main/packages/lexicon/src/types.ts

// TODO: Versioning
// Needs https://github.com/serde-rs/serde/issues/745 probably.
// https://github.com/serde-rs/serde/pull/2525

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LexiconDoc {
    #[serde(rename = "lexicon")]
    version: u32,

    id: String, // Actually NSID

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    defs: BTreeMap<String, UserType>,
}

/////
// Primitives
/////

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Boolean {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#const: Option<bool>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Unknown {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/////
// From IPLD
/////

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

/////
// References
/////
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ref {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    r#ref: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RefUnion {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed: Option<bool>,
}

/////
// Blob
/////

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

/////
// Complex Types
/////

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_lenght: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
    items: ArrayItem,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ArrayItem {
    // Primitive
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),
    // Ipld
    Bytes(Bytes),
    CidLink(CidLink),
    // Blob
    Blob(Blob),
    // RefVariant
    Ref(Ref),
    Union(RefUnion),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrimitveArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_lenght: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
    items: Primitive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Object {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nullable: Vec<String>,
    properties: BTreeMap<String, ObjectProperty>,

    #[serde(default, rename = "type", skip_serializing_if = "String::is_empty")]
    _type: String, // Hacky hackity hack
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ObjectProperty {
    // RefVariant
    Ref(Ref),
    Union(RefUnion),
    // Ipld
    Bytes(Bytes),
    CidLink(CidLink),
    // Array
    Array(Array),
    // Blob
    Blob(Blob),
    // Primitive
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),
}

/////
// XRPC
/////

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
    properties: BTreeMap<String, ParameterProperty>,

    #[serde(rename = "type")]
    _type: String, // Hacky hackity hack
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ParameterProperty {
    // Primitive
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),

    Array(PrimitveArray),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<XrpcBodySchema>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XrpcSubscriptionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    schema: Option<XrpcBodySchema>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum XrpcBodySchema {
    // RefVariant
    Ref(Ref),
    Union(RefUnion),
    // Object
    Object(Object),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct XrpcError {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct XrpcQuery {
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

//////
// Database
//////

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    record: Object,
}

/////
// Core
/////

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum UserType {
    Record(Record),
    Query(XrpcQuery),
    Procedure(XprcProcedure),
    Subscription(XrpcSubscription),
    Blob(Blob),
    Array(Array),
    Token(Token),
    Object(Object),
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Bytes(Bytes),
    CidLink(CidLink),
    Unknown(Unknown),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum Primitive {
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),
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

            let lex = match serde_json::from_slice::<LexiconDoc>(&json) {
                Ok(lex) => lex,
                Err(e) => panic!("can't parse lexicon {i}: {e}"),
            };

            let json_roundtrip = serde_json::to_vec(&lex).unwrap();

            let json: serde_json::Value = serde_json::from_slice(&json).unwrap();
            let json_roundtrip: serde_json::Value =
                serde_json::from_slice(&json_roundtrip).unwrap();

            assert_eq!(json, json_roundtrip, "\nlexicon {i} different",);
        }
    }
}
