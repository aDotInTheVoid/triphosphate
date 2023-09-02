// https://github.com/ipld/libipld/blob/8478d6d66576636b9970cb3b00a232be7a88ea42/dag-cbor/src/encode.rs#L274-L288

use libipld::cbor::encode::write_null;
use libipld::cbor::DagCborCodec;
use libipld::codec::Encode;

use crate::Any;

impl Encode<DagCborCodec> for Any {
    fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> libipld::Result<()> {
        match self {
            Any::Null => write_null(w),
            Any::Bool(b) => b.encode(c, w),
            Any::Integer(i) => i.encode(c, w),
            Any::Bytes(b) => b.encode(c, w),
            Any::String(s) => s.encode(c, w),
            Any::List(l) => l.encode(c, w),
            Any::Map(m) => m.encode(c, w),
            Any::Link(l) => l.encode(c, w),
        }
    }
}
