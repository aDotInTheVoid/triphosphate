use std::collections::BTreeMap;

use camino::Utf8Path;
use lexicon::LexiconDoc;

mod compiller;
mod lexicon;
mod writer;

fn main() {
    let mut map = BTreeMap::new();

    for s in [
        include_str!("../lexicons/app/bsky/actor/defs.json"),
        include_str!("../lexicons/app/bsky/actor/getPreferences.json"),
        include_str!("../lexicons/app/bsky/actor/getProfile.json"),
        // include_str!("../lexicons/app/bsky/actor/getProfiles.json"),
        // include_str!("../lexicons/app/bsky/actor/getSuggestions.json"),
        // include_str!("../lexicons/app/bsky/actor/profile.json"),
        // include_str!("../lexicons/app/bsky/actor/putPreferences.json"),
        // include_str!("../lexicons/app/bsky/actor/searchActors.json"),
        // include_str!("../lexicons/app/bsky/actor/searchActorsTypeahead.json"),
        include_str!("../lexicons/app/bsky/embed/external.json"),
        include_str!("../lexicons/app/bsky/embed/images.json"),
        include_str!("../lexicons/app/bsky/embed/record.json"),
        include_str!("../lexicons/app/bsky/embed/recordWithMedia.json"),
        include_str!("../lexicons/app/bsky/feed/defs.json"),
        // include_str!("../lexicons/app/bsky/feed/describeFeedGenerator.json"),
        // include_str!("../lexicons/app/bsky/feed/generator.json"),
        // include_str!("../lexicons/app/bsky/feed/getActorFeeds.json"),
        // include_str!("../lexicons/app/bsky/feed/getAuthorFeed.json"),
        // include_str!("../lexicons/app/bsky/feed/getFeed.json"),
        // include_str!("../lexicons/app/bsky/feed/getFeedGenerator.json"),
        // include_str!("../lexicons/app/bsky/feed/getFeedGenerators.json"),
        // include_str!("../lexicons/app/bsky/feed/getFeedSkeleton.json"),
        // include_str!("../lexicons/app/bsky/feed/getLikes.json"),
        // include_str!("../lexicons/app/bsky/feed/getPostThread.json"),
        // include_str!("../lexicons/app/bsky/feed/getPosts.json"),
        // include_str!("../lexicons/app/bsky/feed/getRepostedBy.json"),
        // include_str!("../lexicons/app/bsky/feed/getTimeline.json"),
        // include_str!("../lexicons/app/bsky/feed/like.json"),
        include_str!("../lexicons/app/bsky/feed/post.json"),
        // include_str!("../lexicons/app/bsky/feed/repost.json"),
        // include_str!("../lexicons/app/bsky/graph/block.json"),
        include_str!("../lexicons/app/bsky/graph/defs.json"),
        include_str!("../lexicons/app/bsky/graph/follow.json"),
        // include_str!("../lexicons/app/bsky/graph/getBlocks.json"),
        // include_str!("../lexicons/app/bsky/graph/getFollowers.json"),
        // include_str!("../lexicons/app/bsky/graph/getFollows.json"),
        // include_str!("../lexicons/app/bsky/graph/getList.json"),
        // include_str!("../lexicons/app/bsky/graph/getListMutes.json"),
        // include_str!("../lexicons/app/bsky/graph/getLists.json"),
        // include_str!("../lexicons/app/bsky/graph/getMutes.json"),
        // include_str!("../lexicons/app/bsky/graph/list.json"),
        // include_str!("../lexicons/app/bsky/graph/listitem.json"),
        // include_str!("../lexicons/app/bsky/graph/muteActor.json"),
        // include_str!("../lexicons/app/bsky/graph/muteActorList.json"),
        // include_str!("../lexicons/app/bsky/graph/unmuteActor.json"),
        // include_str!("../lexicons/app/bsky/graph/unmuteActorList.json"),
        include_str!("../lexicons/app/bsky/notification/getUnreadCount.json"),
        // include_str!("../lexicons/app/bsky/notification/listNotifications.json"),
        // include_str!("../lexicons/app/bsky/notification/updateSeen.json"),
        include_str!("../lexicons/app/bsky/richtext/facet.json"),
        // include_str!("../lexicons/app/bsky/unspecced/applyLabels.json"),
        // include_str!("../lexicons/app/bsky/unspecced/getPopular.json"),
        // include_str!("../lexicons/app/bsky/unspecced/getPopularFeedGenerators.json"),
        // include_str!("../lexicons/app/bsky/unspecced/getTimelineSkeleton.json"),
        // include_str!("../lexicons/com/atproto/admin/defs.json"),
        // include_str!("../lexicons/com/atproto/admin/disableAccountInvites.json"),
        // include_str!("../lexicons/com/atproto/admin/disableInviteCodes.json"),
        // include_str!("../lexicons/com/atproto/admin/enableAccountInvites.json"),
        // include_str!("../lexicons/com/atproto/admin/getInviteCodes.json"),
        // include_str!("../lexicons/com/atproto/admin/getModerationAction.json"),
        // include_str!("../lexicons/com/atproto/admin/getModerationActions.json"),
        // include_str!("../lexicons/com/atproto/admin/getModerationReport.json"),
        // include_str!("../lexicons/com/atproto/admin/getModerationReports.json"),
        // include_str!("../lexicons/com/atproto/admin/getRecord.json"),
        // include_str!("../lexicons/com/atproto/admin/getRepo.json"),
        // include_str!("../lexicons/com/atproto/admin/rebaseRepo.json"),
        // include_str!("../lexicons/com/atproto/admin/resolveModerationReports.json"),
        // include_str!("../lexicons/com/atproto/admin/reverseModerationAction.json"),
        // include_str!("../lexicons/com/atproto/admin/searchRepos.json"),
        // include_str!("../lexicons/com/atproto/admin/sendEmail.json"),
        // include_str!("../lexicons/com/atproto/admin/takeModerationAction.json"),
        // include_str!("../lexicons/com/atproto/admin/updateAccountEmail.json"),
        // include_str!("../lexicons/com/atproto/admin/updateAccountHandle.json"),
        include_str!("../lexicons/com/atproto/identity/resolveHandle.json"),
        // include_str!("../lexicons/com/atproto/identity/updateHandle.json"),
        include_str!("../lexicons/com/atproto/label/defs.json"),
        // include_str!("../lexicons/com/atproto/label/queryLabels.json"),
        // include_str!("../lexicons/com/atproto/label/subscribeLabels.json"),
        include_str!("../lexicons/com/atproto/moderation/createReport.json"),
        include_str!("../lexicons/com/atproto/moderation/defs.json"),
        // include_str!("../lexicons/com/atproto/repo/applyWrites.json"),
        include_str!("../lexicons/com/atproto/repo/createRecord.json"),
        // include_str!("../lexicons/com/atproto/repo/deleteRecord.json"),
        // include_str!("../lexicons/com/atproto/repo/describeRepo.json"),
        include_str!("../lexicons/com/atproto/repo/getRecord.json"),
        // include_str!("../lexicons/com/atproto/repo/listRecords.json"),
        include_str!("../lexicons/com/atproto/repo/putRecord.json"),
        // include_str!("../lexicons/com/atproto/repo/rebaseRepo.json"),
        include_str!("../lexicons/com/atproto/repo/strongRef.json"),
        // include_str!("../lexicons/com/atproto/repo/uploadBlob.json"),
        // include_str!("../lexicons/com/atproto/server/createAccount.json"),
        // include_str!("../lexicons/com/atproto/server/createAppPassword.json"),
        // include_str!("../lexicons/com/atproto/server/createInviteCode.json"),
        // include_str!("../lexicons/com/atproto/server/createInviteCodes.json"),
        include_str!("../lexicons/com/atproto/server/createSession.json"),
        // include_str!("../lexicons/com/atproto/server/defs.json"),
        // include_str!("../lexicons/com/atproto/server/deleteAccount.json"),
        // include_str!("../lexicons/com/atproto/server/deleteSession.json"),
        // include_str!("../lexicons/com/atproto/server/describeServer.json"),
        // include_str!("../lexicons/com/atproto/server/getAccountInviteCodes.json"),
        // include_str!("../lexicons/com/atproto/server/getSession.json"),
        // include_str!("../lexicons/com/atproto/server/listAppPasswords.json"),
        // include_str!("../lexicons/com/atproto/server/refreshSession.json"),
        // include_str!("../lexicons/com/atproto/server/requestAccountDelete.json"),
        // include_str!("../lexicons/com/atproto/server/requestPasswordReset.json"),
        // include_str!("../lexicons/com/atproto/server/resetPassword.json"),
        // include_str!("../lexicons/com/atproto/server/revokeAppPassword.json"),
        // include_str!("../lexicons/com/atproto/sync/getBlob.json"),
        // include_str!("../lexicons/com/atproto/sync/getBlocks.json"),
        // include_str!("../lexicons/com/atproto/sync/getCheckout.json"),
        include_str!("../lexicons/com/atproto/sync/getCommitPath.json"),
        include_str!("../lexicons/com/atproto/sync/getHead.json"),
        // include_str!("../lexicons/com/atproto/sync/getRecord.json"),
        // include_str!("../lexicons/com/atproto/sync/getRepo.json"),
        // include_str!("../lexicons/com/atproto/sync/listBlobs.json"),
        // include_str!("../lexicons/com/atproto/sync/listRepos.json"),
        // include_str!("../lexicons/com/atproto/sync/notifyOfUpdate.json"),
        // include_str!("../lexicons/com/atproto/sync/requestCrawl.json"),
        // include_str!("../lexicons/com/atproto/sync/subscribeRepos.json"),
    ] {
        let d: LexiconDoc = serde_json::from_str(s).unwrap();

        eprintln!("{}", d.id);

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
