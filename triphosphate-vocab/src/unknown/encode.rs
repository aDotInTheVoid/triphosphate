// https://github.com/ipld/libipld/blob/8478d6d66576636b9970cb3b00a232be7a88ea42/dag-cbor/src/encode.rs#L274-L288

use libipld::cbor::encode::write_null;
use libipld::cbor::DagCborCodec;
use libipld::codec::Encode;

use crate::Unknown;

impl Encode<DagCborCodec> for Unknown {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> libipld::Result<()> {
        match self {
            Unknown::Null => write_null(w),
            Unknown::Bool(b) => b.encode(c, w),
            Unknown::Integer(i) => i.encode(c, w),
            Unknown::Bytes(b) => b.encode(c, w),
            Unknown::String(s) => s.encode(c, w),
            Unknown::List(l) => l.encode(c, w),
            Unknown::Map(m) => m.encode(c, w),
            Unknown::Link(l) => l.encode(c, w),
        }
    }
}
