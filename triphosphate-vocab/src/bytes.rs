use libipld::cbor::DagCborCodec;
use libipld::codec::{Decode, Encode};

mod base64;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bytes {
    #[serde(rename = "$bytes", with = "base64")]
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl Encode<DagCborCodec> for Bytes {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> libipld::Result<()> {
        // Need to do this to pick up the bytestring specialization
        <[u8] as Encode<DagCborCodec>>::encode(&self.bytes, c, w)
    }
}
impl Decode<DagCborCodec> for Bytes {
    fn decode<R: std::io::Read + std::io::Seek>(
        c: DagCborCodec,
        r: &mut R,
    ) -> libipld::Result<Self> {
        // Need to do this to pick up the bytestring specialization

        let bytes = <Box<[u8]> as Decode<DagCborCodec>>::decode(c, r)?;

        Ok(Self {
            bytes: bytes.into(),
        })
    }
}

// https://github.com/serde-rs/serde/issues/661#issuecomment-269858463

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use libipld::Ipld;

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, libipld::DagCbor)]
    struct Demo {
        #[serde(rename = "exampleBytes")]
        example_bytes: Bytes,
    }

    #[test]
    fn decode_json() {
        let s = r#"
            {
              "exampleBytes": {
                "$bytes": "nFERjvLLiw9qm45JrqH9QTzyC2Lu1Xb4ne6+sBrCzI0"
              }
            }"#;

        let eg: Demo = serde_json::from_str(&s).unwrap();

        assert_eq!(
            eg,
            Demo {
                example_bytes: Bytes::new(vec![
                    156, 81, 17, 142, 242, 203, 139, 15, 106, 155, 142, 73, 174, 161, 253, 65, 60,
                    242, 11, 98, 238, 213, 118, 248, 157, 238, 190, 176, 26, 194, 204, 141
                ])
            }
        );
    }

    #[test]
    fn encode_json() {
        let b = Demo {
            example_bytes: Bytes::new(vec![
                156, 81, 17, 142, 242, 203, 139, 15, 106, 155, 142, 73, 174, 161, 253, 65, 60, 242,
                11, 98, 238, 213, 118, 248, 157, 238, 190, 176, 26, 194, 204, 141,
            ]),
        };

        let json = serde_json::to_value(&b).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
              "exampleBytes": {
                "$bytes": "nFERjvLLiw9qm45JrqH9QTzyC2Lu1Xb4ne6+sBrCzI0"
              }
            })
        );
    }

    #[test]
    fn cbor() {
        let some_bytes: Vec<u8> = b"this will work first time, garenteed".to_vec();

        let demo = Demo {
            example_bytes: Bytes::new(some_bytes.clone()),
        };

        let mut map = BTreeMap::new();
        map.insert("exampleBytes".to_owned(), Ipld::Bytes(some_bytes));

        libipld::codec::assert_roundtrip(DagCborCodec, &demo, &Ipld::Map(map));
    }
}
