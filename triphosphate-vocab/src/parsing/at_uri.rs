/*!
<https://atproto.com/specs/at-uri-scheme#restricted-at-uri-syntax>

```bnf
AT-URI        = "at://" AUTHORITY [ "/" COLLECTION [ "/" RKEY ] ]

AUTHORITY     = HANDLE | DID
COLLECTION    = NSID
RKEY          = RECORD-KEY
```
*/

use winnow::{
    combinator::{alt, opt},
    token::tag,
    PResult, Parser,
};

use super::{did, handle, nsid, rkey};

pub fn at_uri<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let authority = alt((handle, did));

    (tag("at://"), authority, opt(('/', nsid, opt(('/', rkey)))))
        .recognize()
        .parse_next(input)
}
