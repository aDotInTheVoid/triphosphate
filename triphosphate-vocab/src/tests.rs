use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;

use libipld::cbor::DagCborCodec;
use libipld::codec::{Codec, Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Any, StringFormat};

#[track_caller]
pub fn valids<
    T: StringFormat
        + Serialize
        + DeserializeOwned
        + Encode<DagCborCodec>
        + Decode<DagCborCodec>
        + PartialEq
        + Debug,
>(
    ss: &[&str],
) {
    for s in ss {
        let t: T = T::from_str(s).expect(&format!("failed to parse {s:?}"));

        assert_eq!(t.as_str(), *s);

        let json = serde_json::to_string(&t).unwrap();
        let json_s = serde_json::to_string(s).unwrap();
        assert_eq!(json, json_s);
        let from_json: T = serde_json::from_str(&json).unwrap();

        assert_eq!(t, from_json);

        let cbor = DagCborCodec.encode(&t).unwrap();
        let cbor_s = DagCborCodec.encode(&s.to_string()).unwrap();
        assert_eq!(cbor, cbor_s, "different cbor when encoding as string");
        let from_cbor = T::decode(DagCborCodec, &mut Cursor::new(&cbor)).unwrap();

        assert_eq!(t, from_cbor);
    }
}

pub fn invalids<T: Debug + StringFormat + DeserializeOwned + Decode<DagCborCodec>>(ss: &[&str]) {
    for s in ss {
        T::from_str(s).unwrap_err();

        let cbor = DagCborCodec.encode(&s.to_string()).unwrap();
        T::decode(DagCborCodec, &mut Cursor::new(&cbor)).unwrap_err();

        let json = serde_json::to_string(s).unwrap();
        serde_json::from_str::<T>(&json).unwrap_err();
    }
}

#[test]
fn any_macro() {
    assert_eq!(any!(10), Any::Integer(10));
    assert_eq!(any!(true), Any::Bool(true));

    assert_eq!(
        any!({
            "x": 1,
            "y": "2"
        }),
        Any::Map({
            let mut m = BTreeMap::new();
            m.insert("x".into(), Any::Integer(1));
            m.insert("y".into(), Any::String("2".into()));
            m
        })
    )
}
