use std::fmt::Debug;
use std::io::Cursor;

use libipld::cbor::DagCborCodec;
use libipld::codec::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::StringFormat;

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

        let cbor = to_cbor(&t).unwrap();
        let cbor_s = to_cbor(s.to_string()).unwrap();
        assert_eq!(cbor, cbor_s, "different cbor when encoding as string");
        let from_cbor = T::decode(DagCborCodec, &mut Cursor::new(&cbor)).unwrap();

        assert_eq!(t, from_cbor);
    }
}

pub fn to_cbor<T: Encode<DagCborCodec>>(t: T) -> libipld::Result<Vec<u8>> {
    let mut r = Vec::new();
    t.encode(DagCborCodec, &mut r)?;
    Ok(r)
}

pub fn invalids<T: Debug + StringFormat + DeserializeOwned + Decode<DagCborCodec>>(ss: &[&str]) {
    for s in ss {
        T::from_str(s).unwrap_err();

        let cbor = to_cbor(s.to_string()).unwrap();
        T::decode(DagCborCodec, &mut Cursor::new(&cbor)).unwrap_err();

        let json = serde_json::to_string(s).unwrap();
        serde_json::from_str::<T>(&json).unwrap_err();
    }
}
