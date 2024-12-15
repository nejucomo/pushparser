use std::io::Read;

use crate::{ParseError, ParseResult, ParserCore};

pub trait ReadParser: ParserCore<[u8]> {
    fn read_parse<R, E>(self, r: R) -> ParseResult<Self::Output, Self::Error>
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
    ) -> ParseResult<Self::Output, Self::Error>
    where
        R: Read;
}

impl<T> ReadParser for T
where
    T: ParserCore<[u8]>,
    T::Error: From<std::io::Error>,
{
    fn read_parse_with_bufsize<R, E>(
        self,
        mut r: R,
        bufsize: usize,
    ) -> ParseResult<Self::Output, Self::Error>
    where
        R: Read,
    {
        use crate::Update::*;
        use ParseError::*;

        let mut parser = self;

        let mut heapbuf = vec![0u8; bufsize];
        let buf = heapbuf.as_mut_slice();

        let mut bytes_read = r.read(buf).map_err(Self::Error::from)?;
        while bytes_read > 0 {
            match parser.feed(&buf[..bytes_read])? {
                Pending(next) => {
                    parser = next;
                    bytes_read = r.read(buf).map_err(Self::Error::from)?;
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

        if let Some(output) = parser.finalize()? {
            Ok(output)
        } else {
            Err(ExpectedMoreInput)
        }
    }
}

#[cfg(test)]
mod tests;
