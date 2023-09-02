use winnow::combinator::alt;
use winnow::Parser;

use crate::parsing;

use super::{Did, Handle, StringFormat};

#[derive(Debug, Clone, PartialEq)]
pub enum AtIdentifier {
    Handle(Handle),
    Did(Did),
}

impl StringFormat for AtIdentifier {
    fn as_str(&self) -> &str {
        match self {
            AtIdentifier::Handle(h) => h.as_str(),
            AtIdentifier::Did(d) => d.as_str(),
        }
    }

    type Error = super::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        // TODO: This feels ugly, we shouldn't be allowed to construct these here.
        let mut parser = alt((
            parsing::handle.map(|s| Self::Handle(Handle(s.to_owned()))),
            parsing::did.map(|s| Self::Did(Did(s.to_owned()))),
        ));

        parser.parse(s).map_err(|_| super::ParseError(()))
    }
}

// TODO: Tests
