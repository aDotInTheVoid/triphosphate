use std::io::Cursor;

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use libipld::{cbor::DagCborCodec, prelude::Decode, Ipld};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let bytes = STANDARD_NO_PAD.decode(&arg).unwrap();

    let b = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();

    println!("{}", debug3::pprint(&b));
}
