use rquickjs::{Context, Exception, Function, Object, Runtime};
use serde::Serialize;

mod bridge;

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

        let result = validator.call::<_, Object>((uri, object)).unwrap();

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
