use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// https://github.com/bluesky-social/atproto/blob/main/packages/lexicon/src/types.ts

// TODO: Versioning
// Needs https://github.com/serde-rs/serde/issues/745 probably.
// https://github.com/serde-rs/serde/pull/2525

/// Represents a single `.json` file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LexiconDoc {
    #[serde(rename = "lexicon")]
    pub version: u32,

    pub id: String, // Actually NSID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub defs: BTreeMap<String, UserType>,
}

/////
// Primitives
/////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Boolean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#const: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Integer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum StringFormat {
    AtIdentifier,
    AtUri,
    Cid,
    Datetime,
    Did,
    Handle,
    Language,
    Nsid,
    Uri,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LexString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StringFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_graphemes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_graphemes: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_values: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Unknown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/////
// From IPLD
/////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bytes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CidLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/////
// References
/////
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ref {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub r#ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RefUnion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
}

/////
// Blob
/////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<String>>,
}

/////
// Complex Types
/////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lenght: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    pub items: ArrayItem,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ArrayItem {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrimitveArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lenght: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    pub items: Primitive,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Object {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nullable: Vec<String>,
    pub properties: BTreeMap<String, ObjectProperty>,

    #[serde(default, rename = "type", skip_serializing_if = "String::is_empty")]
    _type: String, // Hacky hackity hack
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ObjectProperty {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    pub properties: BTreeMap<String, ParameterProperty>,

    #[serde(rename = "type")]
    _type: String, // Hacky hackity hack
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ParameterProperty {
    // Primitive
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),

    Array(PrimitveArray),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<XrpcBodySchema>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct XrpcSubscriptionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: Option<XrpcBodySchema>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum XrpcBodySchema {
    // RefVariant
    Ref(Ref),
    Union(RefUnion),
    // Object
    Object(Object),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcError {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XrpcQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<XrpcParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<XrpcBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<XrpcError>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XprcProcedure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<XrpcParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<XrpcBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<XrpcBody>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<XrpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct XrpcSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<XrpcParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<XrpcSubscriptionMessage>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<XrpcError>,
}

//////
// Database
//////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub record: Object,
}

/////
// Core
/////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum UserType {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Primitive {
    Boolean(Boolean),
    Integer(Integer),
    String(LexString),
    Unknown(Unknown),
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    // fd -e json | sd "(.*)" '"$1",'
    const ALL_LEXICONS: &[&str] = &[
        "app/bsky/actor/defs.json",
        "app/bsky/actor/getPreferences.json",
        "app/bsky/actor/getProfile.json",
        "app/bsky/actor/getProfiles.json",
        "app/bsky/actor/getSuggestions.json",
        "app/bsky/actor/profile.json",
        "app/bsky/actor/putPreferences.json",
        "app/bsky/actor/searchActors.json",
        "app/bsky/actor/searchActorsTypeahead.json",
        "app/bsky/embed/external.json",
        "app/bsky/embed/images.json",
        "app/bsky/embed/record.json",
        "app/bsky/embed/recordWithMedia.json",
        "app/bsky/feed/defs.json",
        "app/bsky/feed/describeFeedGenerator.json",
        "app/bsky/feed/generator.json",
        "app/bsky/feed/getActorFeeds.json",
        "app/bsky/feed/getAuthorFeed.json",
        "app/bsky/feed/getFeed.json",
        "app/bsky/feed/getFeedGenerator.json",
        "app/bsky/feed/getFeedGenerators.json",
        "app/bsky/feed/getFeedSkeleton.json",
        "app/bsky/feed/getLikes.json",
        "app/bsky/feed/getPostThread.json",
        "app/bsky/feed/getPosts.json",
        "app/bsky/feed/getRepostedBy.json",
        "app/bsky/feed/getTimeline.json",
        "app/bsky/feed/like.json",
        "app/bsky/feed/post.json",
        "app/bsky/feed/repost.json",
        "app/bsky/graph/block.json",
        "app/bsky/graph/defs.json",
        "app/bsky/graph/follow.json",
        "app/bsky/graph/getBlocks.json",
        "app/bsky/graph/getFollowers.json",
        "app/bsky/graph/getFollows.json",
        "app/bsky/graph/getList.json",
        "app/bsky/graph/getListMutes.json",
        "app/bsky/graph/getLists.json",
        "app/bsky/graph/getMutes.json",
        "app/bsky/graph/list.json",
        "app/bsky/graph/listitem.json",
        "app/bsky/graph/muteActor.json",
        "app/bsky/graph/muteActorList.json",
        "app/bsky/graph/unmuteActor.json",
        "app/bsky/graph/unmuteActorList.json",
        "app/bsky/notification/getUnreadCount.json",
        "app/bsky/notification/listNotifications.json",
        "app/bsky/notification/updateSeen.json",
        "app/bsky/richtext/facet.json",
        "app/bsky/unspecced/applyLabels.json",
        "app/bsky/unspecced/getPopular.json",
        "app/bsky/unspecced/getPopularFeedGenerators.json",
        "app/bsky/unspecced/getTimelineSkeleton.json",
        "com/atproto/admin/defs.json",
        "com/atproto/admin/disableAccountInvites.json",
        "com/atproto/admin/disableInviteCodes.json",
        "com/atproto/admin/enableAccountInvites.json",
        "com/atproto/admin/getInviteCodes.json",
        "com/atproto/admin/getModerationAction.json",
        "com/atproto/admin/getModerationActions.json",
        "com/atproto/admin/getModerationReport.json",
        "com/atproto/admin/getModerationReports.json",
        "com/atproto/admin/getRecord.json",
        "com/atproto/admin/getRepo.json",
        "com/atproto/admin/rebaseRepo.json",
        "com/atproto/admin/resolveModerationReports.json",
        "com/atproto/admin/reverseModerationAction.json",
        "com/atproto/admin/searchRepos.json",
        "com/atproto/admin/sendEmail.json",
        "com/atproto/admin/takeModerationAction.json",
        "com/atproto/admin/updateAccountEmail.json",
        "com/atproto/admin/updateAccountHandle.json",
        "com/atproto/identity/resolveHandle.json",
        "com/atproto/identity/updateHandle.json",
        "com/atproto/label/defs.json",
        "com/atproto/label/queryLabels.json",
        "com/atproto/label/subscribeLabels.json",
        "com/atproto/moderation/createReport.json",
        "com/atproto/moderation/defs.json",
        "com/atproto/repo/applyWrites.json",
        "com/atproto/repo/createRecord.json",
        "com/atproto/repo/deleteRecord.json",
        "com/atproto/repo/describeRepo.json",
        "com/atproto/repo/getRecord.json",
        "com/atproto/repo/listRecords.json",
        "com/atproto/repo/putRecord.json",
        "com/atproto/repo/rebaseRepo.json",
        "com/atproto/repo/strongRef.json",
        "com/atproto/repo/uploadBlob.json",
        "com/atproto/server/createAccount.json",
        "com/atproto/server/createAppPassword.json",
        "com/atproto/server/createInviteCode.json",
        "com/atproto/server/createInviteCodes.json",
        "com/atproto/server/createSession.json",
        "com/atproto/server/defs.json",
        "com/atproto/server/deleteAccount.json",
        "com/atproto/server/deleteSession.json",
        "com/atproto/server/describeServer.json",
        "com/atproto/server/getAccountInviteCodes.json",
        "com/atproto/server/getSession.json",
        "com/atproto/server/listAppPasswords.json",
        "com/atproto/server/refreshSession.json",
        "com/atproto/server/requestAccountDelete.json",
        "com/atproto/server/requestPasswordReset.json",
        "com/atproto/server/resetPassword.json",
        "com/atproto/server/revokeAppPassword.json",
        "com/atproto/sync/getBlob.json",
        "com/atproto/sync/getBlocks.json",
        "com/atproto/sync/getCheckout.json",
        "com/atproto/sync/getCommitPath.json",
        "com/atproto/sync/getHead.json",
        "com/atproto/sync/getRecord.json",
        "com/atproto/sync/getRepo.json",
        "com/atproto/sync/listBlobs.json",
        "com/atproto/sync/listRepos.json",
        "com/atproto/sync/notifyOfUpdate.json",
        "com/atproto/sync/requestCrawl.json",
        "com/atproto/sync/subscribeRepos.json",
    ];

    #[test]
    fn all_lexicons() {
        let base_dir = env!("CARGO_MANIFEST_DIR");

        for i in ALL_LEXICONS {
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
