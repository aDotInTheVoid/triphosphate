mod at_uri;
mod regexes;

use std::ops::RangeBounds;
use winnow::{
    error::{ErrMode, ErrorKind, ParserError},
    stream::Range,
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
