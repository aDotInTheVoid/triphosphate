mod at_uri;
mod regexes;

use std::ops::RangeBounds;
use winnow::{
    error::{ErrMode, ErrorKind, ParserError},
    stream::Range,
    token::take_while,
    PResult, Parser,
};

// Easy Parsers
pub fn did<'i>(input: &mut &'i str) -> PResult<&'i str> {
    len_is(regex(regexes::did()), ..=(2 * 1024)).parse_next(input)
}
pub fn handle<'i>(input: &mut &'i str) -> PResult<&'i str> {
    len_is(regex(regexes::handle()), ..=253).parse_next(input)
}
pub fn nsid<'i>(input: &mut &'i str) -> PResult<&'i str> {
    len_is(regex(regexes::nsid()), ..=(253 + 1 + 63)).parse_next(input)
}
fn rkey<'i>(input: &mut &'i str) -> PResult<&'i str> {
    take_while(
        1..=512,
        |c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '~' | '_' ),
    )
    .verify(|s: &str| s != "." && s != "..")
    .parse_next(input)
}
pub use at_uri::at_uri;

// Combinators
fn regex<E: for<'a> ParserError<&'a str>>(regex: &regex::Regex) -> impl Parser<&str, &str, E> {
    let inner = |input: &mut &str| -> PResult<(), E> {
        match regex.find(input) {
            Some(m) => {
                assert_eq!(m.start(), 0);
                *input = &input[m.end()..];
                Ok(())
            }
            None => Err(ErrMode::from_error_kind(input, ErrorKind::Tag)),
        }
    };

    inner.recognize()
}

fn len_is<'input, E: for<'a> ParserError<&'a str>>(
    p: impl Parser<&'input str, &'input str, E>,
    l: impl Into<Range>,
) -> impl Parser<&'input str, &'input str, E> {
    let l = l.into();
    p.verify(move |s: &str| l.contains(&s.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://atproto.com/specs/record-key#examples
    #[test]
    fn rkey_valid() {
        for s in ["3jui7kd54zh2y", "self", "example.com", "~1.2-3_", "dHJ1ZQ"] {
            let rkey = rkey.parse(s).unwrap();
            assert_eq!(rkey, s)
        }
    }

    #[test]
    fn rkey_invalid() {
        for s in [
            r##"literal:self"##,
            r##"alpha/beta"##,
            r##"."##,
            r##".."##,
            r##"#extra"##,
            r##"@handle"##,
            r##"any space"##,
            r##"any+space"##,
            r##"number[3]"##,
            r##"number(3)"##,
            r##""quote""##,
            r##"pre:fix"##,
            r##"dHJ1ZQ=="##,
        ] {
            rkey.parse(s).unwrap_err();
        }
    }
}
