use rquickjs::{Context, Exception, FromJs, Function, Object, Runtime};
use serde::Serialize;
use triphosphate::LexItem;

mod bridge;
mod post;

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
fn check<T: LexItem>(item: &T) {
    let result = validate(T::URI, item);
    match result {
        ValidationResult::Valid => {}
        ValidationResult::Invalid(err) => panic!("validation failed: {}", err),
    }
}
