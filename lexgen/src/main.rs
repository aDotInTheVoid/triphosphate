use std::collections::BTreeMap;

use camino::Utf8Path;
use lexicon::LexiconDoc;

mod compiller;
mod lexicon;
mod writer;

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

fn main() {
    let mut map = BTreeMap::new();

    for s in [
        include_str!("../lexicons/app/bsky/feed/post.json"),
        include_str!("../lexicons/com/atproto/repo/strongRef.json"),
    ] {
        let d: LexiconDoc = serde_json::from_str(s).unwrap();
        let m = compiller::lower_lexicon(d);

        for (k, v) in m {
            compiller::insert_new(&mut map, k, v);
        }
    }

    writer::write_to(
        &Utf8Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join("src")
            .join("lex"),
        &map,
    );
}
