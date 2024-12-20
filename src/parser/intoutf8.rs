use std::str::Utf8Error;

use either::Either::{self, Left, Right};

use crate::error::{ParseResult, ParseResultExt, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

/// Wrap any [str] parser into a UTF-8 `[u8]` parser
#[derive(Debug)]
pub struct IntoUtf8Parser<P>(P);

impl<P> From<P> for IntoUtf8Parser<P> {
    fn from(textparser: P) -> Self {
        IntoUtf8Parser(textparser)
    }
}

impl<P> ParserCore<[u8]> for IntoUtf8Parser<P>
where
    P: ParserCore<str>,
{
    type Output = P::Output;
    type Error = Either<P::Error, Utf8Error>;

    fn feed(self, buffer: &[u8]) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        let (s, _) = from_utf8_partial(buffer).map_err(Right)?;
        self.0.feed(s).map_next(IntoUtf8Parser).map_err_custom(Left)
    }

    fn finalize(self, buffer: &[u8]) -> ParseResult<Option<Self::Output>, Self::Error> {
        use crate::error::ParseError::ExpectedMoreInput;

        let (s, noise) = from_utf8_partial(buffer).map_err(Right)?;
        if noise.is_empty() {
            self.0.finalize(s).map_err_custom(Left)
        } else {
            Err(ExpectedMoreInput)
        }
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
