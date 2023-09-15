use std::{fmt::Debug, io::Cursor};

use libipld::{
    cbor::{DagCbor, DagCborCodec},
    prelude::Codec,
    DagCbor,
};
use rquickjs::{Context, Exception, FromJs, Function, Object, Runtime};
use serde::{de::DeserializeOwned, Serialize};
use triphosphate::LexItem;

mod bridge;
mod feed_def;
mod post;
mod subscribe_repo;

const BUNDLE_JS: &str = include_str!("../../lexgen/dist/bundle.js");

#[derive(Debug, PartialEq)]
enum ValidationResult {
    Valid,
    Invalid(String),
}

fn validate<T: Serialize + DagCbor>(uri: &str, item: &T) -> ValidationResult {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    let bytes = DagCborCodec.encode(&item).unwrap();

    context.with(|ctx| {
        ctx.eval::<(), _>(BUNDLE_JS).unwrap();

        let validator = ctx
            .globals()
            .as_object()
            .unwrap()
            .get::<_, Object>("triphosphate_bridge")
            .unwrap()
            .get::<_, Function>("default")
            .unwrap();

        let obj_s = serde_json::to_string(item).unwrap();

        dbg!(&obj_s);
        eprintln!("{bytes:x?}");

        let object = ctx.json_parse(obj_s).unwrap();
        let byte_array = ctx.json_parse(serde_json::to_vec(&bytes).unwrap()).unwrap();

        let result = match validator.call::<_, Object>((uri, object, byte_array)) {
            Ok(r) => r,
            Err(e) => {
                if let rquickjs::Error::Exception = e {
                    let exc = ctx.catch();
                    let exc = Exception::from_js(ctx, exc).unwrap();

                    panic!(
                        "exception when calling validation: {}\n{:?}",
                        exc.message().unwrap_or_default(),
                        exc
                    );
                } else {
                    panic!("failed to call validator: {e}")
                }
            }
        };

        if result.get::<_, bool>("success").unwrap() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(
                result
                    .get::<_, Exception>("error")
                    .unwrap()
                    .message()
                    .unwrap(),
            )
        }
    })
}

#[track_caller]
fn check<T: LexItem + PartialEq + Debug>(item: &T) {
    let result = validate(T::URI, item);
    match result {
        ValidationResult::Valid => {}
        ValidationResult::Invalid(err) => panic!("validation failed: {}", err),
    }

    check_cbor_roundtrip(item);
    check_json_str_roundtrip(item);
    check_any_roundtrip(item);
}

fn check_cbor_roundtrip<T: DagCbor + PartialEq + Debug>(item: &T) {
    let bytes = DagCborCodec.encode(item).unwrap();

    let new_item = T::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();

    assert_eq!(item, &new_item);
}

fn check_json_str_roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(item: &T) {
    let json_s = serde_json::to_string(item).unwrap();
    let new_item: T = serde_json::from_str(&json_s).unwrap();

    assert_eq!(item, &new_item);
}

fn check_any_roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(item: &T) {
    let unknown = triphosphate_vocab::to_any(item).unwrap();
    let new_item: T = triphosphate_vocab::from_any(unknown).unwrap();

    assert_eq!(item, &new_item);
}

#[test]
fn different_cbor() {
    #[derive(Serialize, DagCbor)]
    struct Demo {
        #[ipld(rename = "ipld")]
        x: i32,
        b: bool,
    }

    let vresult = validate("foo.bar", &Demo { x: 1, b: true });

    // omFi9WRpcGxkAQ: Map({"b": Bool(true), "ipld": Integer(1)})
    // omFi9WF4AQ:     Map({"b": Bool(true), "x":    Integer(1)})
    assert_eq!(
        vresult,
        ValidationResult::Invalid("CBOR arrays not equal: omFi9WRpcGxkAQ != omFi9WF4AQ".to_owned())
    );
}

#[test]
fn understanding_cbor_macro() {
    // Using a serde rename attribute also applies to the DagCbor name.

    #[derive(Serialize, DagCbor)]
    struct Demo {
        #[serde(rename = "renamed")]
        orig: i32,
    }

    // Indirection so we know the serde impl isn't being used
    struct D2;
    impl libipld::codec::Encode<DagCborCodec> for D2 {
        fn encode<W: std::io::Write>(&self, c: DagCborCodec, w: &mut W) -> anyhow::Result<()> {
            Demo { orig: 42 }.encode(c, w)
        }
    }

    let demo_bytes = DagCborCodec.encode(&D2).unwrap();
    let demo_any = DagCborCodec.decode::<libipld::Ipld>(&demo_bytes).unwrap();

    assert_eq!(demo_any, libipld::ipld!({ "renamed": 42 }));
}
