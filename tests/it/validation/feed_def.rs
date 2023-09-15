use triphosphate::lex::app::bsky::feed::defs::ViewerThreadState;

use super::check;

#[test]
fn viewer_thread_state() {
    check(&ViewerThreadState {
        can_reply: Some(true),
    });
    check(&ViewerThreadState {
        can_reply: Some(false),
    });
    check(&ViewerThreadState { can_reply: None });
}
