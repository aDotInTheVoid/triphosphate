use std::{fmt::Debug, io::Cursor};

use libipld::cbor::{DagCbor, DagCborCodec};
use rquickjs::{Context, Exception, FromJs, Function, Object, Runtime};
use serde::{de::DeserializeOwned, Serialize};
use triphosphate::LexItem;

mod bridge;
mod post;
mod subscribe_repo;

const BUNDLE_JS: &str = include_str!("../../lexgen/dist/bundle.js");

#[derive(Debug, PartialEq)]
enum ValidationResult {
    Valid,
    Invalid(String),
}

fn validate<T: Serialize>(uri: &str, item: &T) -> ValidationResult {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

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

        let object = ctx.json_parse(serde_json::to_vec(item).unwrap()).unwrap();

        let result = match validator.call::<_, Object>((uri, object)) {
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
    check_unknown_roundtrip(item);
}

fn check_cbor_roundtrip<T: DagCbor + PartialEq + Debug>(item: &T) {
    let mut bytes: Vec<_> = Vec::new();
    item.encode(DagCborCodec, &mut bytes).unwrap();

    let new_item = T::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();

    assert_eq!(item, &new_item);
}

fn check_json_str_roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(item: &T) {
    let json_s = serde_json::to_string(item).unwrap();
    let new_item: T = serde_json::from_str(&json_s).unwrap();

    assert_eq!(item, &new_item);
}

fn check_unknown_roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(item: &T) {
    let unknown = triphosphate_vocab::to_any(item).unwrap();
    let new_item: T = triphosphate_vocab::from_any(unknown).unwrap();

    assert_eq!(item, &new_item);
}
