use std::{borrow::Cow, str::Utf8Error};

use either::Either;

use crate::error::{ParseResult, ParseResultExt};
use crate::parser::{ParserCore, Update};

/// Wrap any [str] parser into a UTF-8 `[u8]` parser
#[derive(Debug)]
pub struct IntoUtf8Parser<P> {
    textparser: P,
    /// hold partial characters, or intermediate buffers continuing from partial characters
    ///
    /// Whenever a partial character is encountered at the end of an input buffer, it is copied here. Subsequent [ParserCore::feed] calls extend this buffer, which becomes a [Cow::Owned] in the [Update] result.
    straddlebuf: Vec<u8>,
}

impl<P> From<P> for IntoUtf8Parser<P> {
    fn from(textparser: P) -> Self {
        IntoUtf8Parser {
            textparser,
            straddlebuf: vec![],
        }
    }
}

impl<P> ParserCore<[u8]> for IntoUtf8Parser<P>
where
    P: ParserCore<str>,
{
    type Output = P::Output;
    type Error = Either<P::Error, Utf8Error>;

    fn feed(
        self,
        buffer: &[u8],
    ) -> ParseResult<Update<Self, Self::Output, Cow<'_, [u8]>>, Self::Error> {
        use Cow::*;
        use Either::*;
        use Update::*;

        let IntoUtf8Parser {
            textparser,
            mut straddlebuf,
        } = self;

        if straddlebuf.is_empty() {
            let (s, partial) = from_utf8_partial(buffer).map_err(Right)?;
            straddlebuf.extend_from_slice(partial);

            match textparser.feed(s).map_err_custom(Left)? {
                Pending(textparser) => Ok(Pending(IntoUtf8Parser {
                    textparser,
                    straddlebuf,
                })),
                Parsed(output, cow) => Ok(Parsed(
                    output,
                    match cow {
                        Borrowed(textsuffix) => Borrowed(textsuffix.as_bytes()),
                        Owned(textsuffix) => Owned(Vec::from(textsuffix)),
                    },
                )),
            }
        } else {
            straddlebuf.extend_from_slice(buffer);

            let (s, partial) = from_utf8_partial(straddlebuf.as_slice()).map_err(Right)?;
            match textparser.feed(s).map_err_custom(Left)? {
                Pending(textparser) => Ok(Pending(IntoUtf8Parser {
                    textparser,
                    straddlebuf: partial.to_owned(),
                })),
                Parsed(output, cow) => {
                    let mut v = Vec::from(cow.into_owned());
                    v.extend_from_slice(partial);

                    Ok(Parsed(output, Owned(v)))
                }
            }
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        use crate::error::ParseError::ExpectedMoreInput;
        use Either::Left;

        if self.straddlebuf.is_empty() {
            self.textparser.finalize().map_err_custom(Left)
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
