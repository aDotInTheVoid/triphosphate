macro_rules! lazy_regex {
    ($($name:ident = $reg:literal;)*) => {
        $(
            pub fn $name() -> &'static ::regex::Regex {
                static RE: ::std::sync::OnceLock<::regex::Regex> = ::std::sync::OnceLock::new();
                RE.get_or_init(|| ::regex::Regex::new($reg).unwrap())
            }
        )*
    };
}

lazy_regex! {
    // *IMPORTANT*: These must all start with ^, but must not end with $.

    // https://github.com/bluesky-social/atproto/blob/ea9d96e3a44119ca6189e7a3989a1bd9b54989a9/packages/identifier/src/handle.ts#L77
    handle = r#"^([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?"#;
    // https://github.com/bluesky-social/atproto/blob/ea9d96e3a44119ca6189e7a3989a1bd9b54989a9/packages/identifier/src/did.ts#L49
    did = r#"^did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]"#;
    // https://github.com/bluesky-social/atproto/blob/ea9d96e3a44119ca6189e7a3989a1bd9b54989a9/packages/nsid/src/index.ts#L100
    nsid = r#"^[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(\.[a-zA-Z]([a-zA-Z]{0,61}[a-zA-Z])?)"#;
}
