use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
};
use serde::{Deserialize, Serialize};

use crate::StringFormat;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CidLink {
    #[serde(rename = "$link")]
    pub(crate) link: crate::Cid,
}

impl Encode<DagCborCodec> for CidLink {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> libipld::Result<()> {
        self.link.cid.encode(c, w)
    }
}
impl Decode<DagCborCodec> for CidLink {
    fn decode<R: std::io::Read + std::io::Seek>(
        c: DagCborCodec,
        r: &mut R,
    ) -> libipld::Result<Self> {
        let c = cid::Cid::decode(c, r)?;
        Ok(Self {
            link: crate::Cid::from_cid(c),
        })
    }
}

impl CidLink {
    pub fn from_str(s: &str) -> Result<Self, cid::Error> {
        Ok(Self {
            link: crate::Cid::from_str(s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use libipld::DagCbor;

    use super::*;

    // https://atproto.com/specs/data-model#link

    #[derive(Debug, PartialEq, Serialize, Deserialize, DagCbor)]
    struct Demo {
        #[serde(rename = "exampleLink")]
        example_link: CidLink,
    }

    #[test]
    fn json_encode() {
        let demo = Demo {
            example_link: CidLink::from_str(
                "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a",
            )
            .unwrap(),
        };

        let json = serde_json::to_value(demo).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
              "exampleLink": {
                "$link": "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a"
              }
            }
            )
        );
    }

    #[test]
    fn json_decode() {
        let s = r#"
            {
              "exampleLink": {
                "$link": "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a"
              }
            }"#;
        let demo: Demo = serde_json::from_str(s).unwrap();

        assert_eq!(
            demo,
            Demo {
                example_link: CidLink::from_str(
                    "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a",
                )
                .unwrap(),
            }
        );
    }

    #[test]
    fn cbor() {
        let demo = Demo {
            example_link: CidLink::from_str(
                "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a",
            )
            .unwrap(),
        };

        let cid = cid::Cid::try_from("bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a")
            .unwrap();

        libipld::codec::assert_roundtrip(
            DagCborCodec,
            &demo,
            &libipld::ipld!({
                "exampleLink": cid,
            }),
        )
    }
}
