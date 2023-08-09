use rquickjs::{Context, Exception, Function, Object, Runtime};

const BUNDLE_JS: &str = include_str!("../lexgen/dist/bundle.js");

#[test]
fn run_quickjs() {
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

        let r = validator
            .call::<_, Object>(("app.bsky.feed.post", Object::new(ctx)))
            .unwrap();

        let is_valid = r.get::<_, bool>("success").unwrap();
        assert!(!is_valid);

        let error = r.get::<_, Exception>("error").unwrap();
        assert_eq!(
            error.message().unwrap(),
            r##"Record must have the property "text""##
        );
    });
}
