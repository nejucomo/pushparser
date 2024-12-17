use std::io::Read;

use either::Either;

use crate::error::ParseError::{ExpectedMoreInput, UnexpectedInput};
use crate::error::{ParseResult, ParseResultExt};
use crate::parser::ParserCore;

pub trait ReadParser: ParserCore<[u8]> {
    fn read_parse<R, E>(
        self,
        r: R,
    ) -> ParseResult<Self::Output, Either<Self::Error, std::io::Error>>
    where
        R: Read,
    {
        const BUFSIZE: usize = 1 << 16;

        self.read_parse_with_bufsize::<R, E>(r, BUFSIZE)
    }

    fn read_parse_with_bufsize<R, E>(
        self,
        r: R,
        bufsize: usize,
    ) -> ParseResult<Self::Output, Either<Self::Error, std::io::Error>>
    where
        R: Read;
}

impl<T> ReadParser for T
where
    T: ParserCore<[u8]>,
{
    fn read_parse_with_bufsize<R, E>(
        self,
        mut r: R,
        bufsize: usize,
    ) -> ParseResult<Self::Output, Either<Self::Error, std::io::Error>>
    where
        R: Read,
    {
        use crate::parser::Update::*;
        use Either::*;

        let mut parser = self;

        let mut heapbuf = vec![0u8; bufsize];
        let buf = heapbuf.as_mut_slice();

        let mut bytes_read = r.read(buf).map_err(Right)?;
        while bytes_read > 0 {
            match parser.feed(&buf[..bytes_read]).map_err_custom(Left)? {
                Pending(next) => {
                    parser = next;
                    bytes_read = r.read(buf).map_err(Right)?;
                }
                Parsed(item, remaining) => {
                    return if remaining.is_empty() {
                        Ok(item)
                    } else {
                        Err(UnexpectedInput)
                    };
                }
            }
        }

        if let Some(output) = parser.finalize().map_err_custom(Left)? {
            Ok(output)
        } else {
            Err(ExpectedMoreInput)
        }
    }
}
