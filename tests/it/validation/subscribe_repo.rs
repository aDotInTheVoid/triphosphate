use super::check;
use triphosphate::lex::com::atproto::sync::subscribe_repos;
use triphosphate_vocab::{Bytes, CidLink, Datetime, Did, Handle, StringFormat};

#[test]
fn commit() {
    check(&subscribe_repos::Commit {
        blobs: vec![CidLink::from_str(
            "bafyreiduvn7qni7isoz34ixyevrwxra3y6oquxwlrdezpytx6etjwap66m",
        )
        .unwrap()],
        blocks: Bytes::new(vec![1, 2, 3, 4]),
        commit: CidLink::from_str("bafyreictjt53axaaw4chauhdebrwyorzjdowyaj22qxnj7q2rrw2rxr7zq")
            .unwrap(),
        ops: vec![subscribe_repos::RepoOp {
            action: "something_or_other".to_owned(),
            cid: CidLink::from_str("bafyreicgtrn37hysggzvok4pujehyabmvnru34intfdordvwc2spgafkvm")
                .unwrap(),
            path: "some bath".to_owned(),
        }],
        prev: None,
        rebase: false,
        repo: Did::from_str("did:whatyoucanputwhateverhere:lol").unwrap(),
        rev: "Final-Final-Final-v2-withfix".to_owned(),
        seq: 1010101,
        since: "Hello World".to_owned(),
        time: Datetime::now(),
        too_big: false,
    })
}

#[test]
fn handle() {
    check(&subscribe_repos::Handle {
        did: Did::from_str("did:plc:lollmao").unwrap(),
        handle: Handle::from_str("lies.atproto.com").unwrap(),
        seq: 42069,
        time: Datetime::now(),
    })
}

#[test]
fn info() {
    check(&subscribe_repos::Info {
        message: Some("Are you informed".to_owned()),
        name: "Yes you are".to_owned(),
    })
}

#[test]
fn migrate() {
    check(&subscribe_repos::Migrate {
        did: Did::from_str("did:plc:xxx").unwrap(),
        migrate_to: "the future".to_owned(),
        seq: 10000001,
        time: Datetime::now(),
    })
}

#[test]
fn repo_op() {
    check(&subscribe_repos::RepoOp {
        action: "ThisIsntAKNownValueLol".to_owned(),
        cid: CidLink::from_str("bafyreifi5bqq7og5qxedc5xllono4vlpnfvl4pcbskymzcm5kjmbhgobmu")
            .unwrap(),
        path: "/etc/hosts".to_owned(),
    })
}

#[test]
fn tombstone() {
    check(&subscribe_repos::Tombstone {
        did: Did::from_str("did:plc:lollmao").unwrap(),
        seq: 101,
        time: Datetime::now(),
    });
}
