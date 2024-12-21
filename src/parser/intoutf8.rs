use std::str::Utf8Error;

use either::Either::{self, Left, Right};

use crate::error::{ParseResult, ParseResultExt, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

/// Wrap any [str] parser into a UTF-8 `[u8]` parser
#[derive(Debug)]
pub struct IntoUtf8Parser<P>(P);

impl<P> From<P> for IntoUtf8Parser<P> {
    fn from(textparser: P) -> Self {
        IntoUtf8Parser(textparser)
    }
}

impl<P> ParserBase for IntoUtf8Parser<P>
where
    P: ParserBase,
{
    type Output = P::Output;
    type Error = Either<P::Error, Utf8Error>;

    fn pending_at_end(self) -> Option<Self::Output> {
        self.0.pending_at_end()
    }
}

impl<P> PushParser<[u8]> for IntoUtf8Parser<P>
where
    P: PushParser<str>,
{
    fn feed(self, buffer: &[u8]) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        let (s, _) = from_utf8_partial(buffer).map_err(Right)?;
        self.0.feed(s).map_next(IntoUtf8Parser).map_err_custom(Left)
    }
}

fn from_utf8_partial(buf: &[u8]) -> Result<(&str, &[u8]), Utf8Error> {
    use std::str::from_utf8;

    from_utf8(buf).map(|s| (s, [].as_slice())).or_else(|e| {
        if e.error_len().is_none() {
            let (prefix, partial) = buf.split_at(e.valid_up_to());
            let s = from_utf8(prefix).unwrap();
            Ok((s, partial))
        } else {
            Err(e)
        }
    })
}
