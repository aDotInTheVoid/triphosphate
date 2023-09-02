use super::check;
use triphosphate::lex::com::atproto::sync::subscribe_repos;
use triphosphate_vocab::{Datetime, Did, StringFormat};

#[test]
fn tombstone() {
    check(&subscribe_repos::Tombstone {
        did: Did::from_str("did:plc:lollmao").unwrap(),
        seq: 101,
        time: Datetime::now(),
    });
}

#[test]
fn repo_op() {
    // check(&subscribe_repos::RepoOp {
    //     action: "ThisIsntAKNownValueLol".to_owned(),
    //     cid: CidLink::from_str("bafyreifi5bqq7og5qxedc5xllono4vlpnfvl4pcbskymzcm5kjmbhgobmu")
    //         .unwrap(),
    //     path: "/etc/hosts".to_owned(),
    // })
}
