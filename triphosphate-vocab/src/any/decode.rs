// https://github.com/ipld/libipld/blob/8478d6d66576636b9970cb3b00a232be7a88ea42/dag-cbor/src/decode.rs#L344

use std::io::{Read, Seek};

use libipld::cbor::cbor::{self, MajorKind};
use libipld::cbor::decode::{
    read_bytes, read_link, read_list, read_major, read_map, read_str, read_uint,
};
use libipld::cbor::error::{UnexpectedCode, UnknownTag};
use libipld::cbor::DagCborCodec as DagCbor;
use libipld::codec::Decode;

impl Decode<DagCbor> for super::Any {
    fn decode<R: Read + Seek>(_: DagCbor, r: &mut R) -> libipld::Result<Self> {
        let major = read_major(r)?;
        let ipld = match major.kind() {
            MajorKind::UnsignedInt => Self::Integer(read_uint(r, major)? as i64),
            MajorKind::NegativeInt => Self::Integer(-1 - read_uint(r, major)? as i64),
            MajorKind::ByteString => {
                let len = read_uint(r, major)?;
                Self::Bytes(crate::Bytes::new(read_bytes(r, len)?))
            }
            MajorKind::TextString => {
                let len = read_uint(r, major)?;
                Self::String(read_str(r, len)?)
            }
            MajorKind::Array => {
                let len = read_uint(r, major)?;
                Self::List(read_list(r, len)?)
            }
            MajorKind::Map => {
                let len = read_uint(r, major)?;
                Self::Map(read_map(r, len)?)
            }
            MajorKind::Tag => {
                let value = read_uint(r, major)?;
                if value == 42 {
                    let cid = read_link(r)?;
                    let link = crate::Cid::from_cid(cid);
                    Self::Link(crate::CidLink { link })
                } else {
                    return Err(UnknownTag(value).into());
                }
            }
            MajorKind::Other => match major {
                cbor::FALSE => Self::Bool(false),
                cbor::TRUE => Self::Bool(true),
                cbor::NULL => Self::Null,
                // F32 => Self::Float(read_f32(r)? as f64),
                // F64 => Self::Float(read_f64(r)?),
                m => return Err(UnexpectedCode::new::<Self>(m.into()).into()),
            },
        };
        Ok(ipld)
    }
}
