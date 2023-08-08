use std::collections::BTreeMap;

use camino::Utf8Path;
use lexicon::LexiconDoc;

mod compiller;
mod lexicon;
mod writer;

fn main() {
    let mut map = BTreeMap::new();

    for s in [
        include_str!("../lexicons/app/bsky/actor/defs.json"),
        include_str!("../lexicons/app/bsky/embed/external.json"),
        include_str!("../lexicons/app/bsky/embed/images.json"),
        include_str!("../lexicons/app/bsky/embed/record.json"),
        include_str!("../lexicons/app/bsky/embed/recordWithMedia.json"),
        include_str!("../lexicons/app/bsky/feed/post.json"),
        include_str!("../lexicons/app/bsky/graph/defs.json"),
        include_str!("../lexicons/com/atproto/repo/strongRef.json"),
    ] {
        let d: LexiconDoc = serde_json::from_str(s).unwrap();

        eprintln!("{}", d.id);

        let m = compiller::lower_lexicon(d);

        for (k, v) in m {
            compiller::insert_new(&mut map, k, v);
        }
    }

    writer::write_to(
        &Utf8Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join("src")
            .join("lex"),
        &map,
    );
}
