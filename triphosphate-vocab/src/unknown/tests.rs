use libipld::{cbor::DagCborCodec, codec::assert_roundtrip, ipld, Ipld};

use crate::{Bytes, CidLink};

use super::*;

fn cbor_roundtrip(unknown: Unknown, ipld: Ipld) {
    assert_roundtrip(DagCborCodec, &unknown, &ipld);
}

#[test]
fn primitives_cbor() {
    cbor_roundtrip(Unknown::Integer(101), ipld!(101));
    cbor_roundtrip(Unknown::Integer(-101), ipld!(-101));
    cbor_roundtrip(Unknown::Integer(i64::MIN), ipld!(i64::MIN));
    cbor_roundtrip(Unknown::Integer(i64::MAX), ipld!(i64::MAX));

    cbor_roundtrip(Unknown::Bool(true), ipld!(true));
    cbor_roundtrip(Unknown::Bool(false), ipld!(false));

    cbor_roundtrip(
        Unknown::Bytes(Bytes::new(vec![1, 3, 3, 7])),
        Ipld::Bytes(vec![1, 3, 3, 7]),
    );
    cbor_roundtrip(Unknown::Bytes(Bytes::new(vec![])), Ipld::Bytes(vec![]));

    cbor_roundtrip(Unknown::String(String::new()), ipld!(""));
    cbor_roundtrip(Unknown::String("__test__".to_owned()), ipld!("__test__"));

    cbor_roundtrip(Unknown::List(vec![]), ipld!([]));
    cbor_roundtrip(Unknown::Null, ipld!(null));

    let cid_link =
        CidLink::from_str("bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a").unwrap();
    let cid =
        cid::Cid::try_from("bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a").unwrap();
    cbor_roundtrip(Unknown::Link(cid_link), Ipld::Link(cid));
}
