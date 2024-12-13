use std::io::Read;

use crate::{ParserCore, ReadParseError};

/// The high-level consumer interface to parsers
pub trait Parser: ParserCore {
    fn read_parse<R>(self, mut r: R) -> Result<Self::Output, ReadParseError>
    where
        R: Read,
    {
        use crate::Update::*;
        use ReadParseError::*;

        const BUFSIZE: usize = 1 << 16;

        let mut parser = self;

        let mut heapbuf = Box::new([0u8; BUFSIZE]);
        let buf = heapbuf.as_mut();

        let mut bytes_read = r.read(buf)?;
        while bytes_read > 0 {
            match parser.feed(&buf[..bytes_read]) {
                Pending(next) => {
                    parser = next;
                    bytes_read = r.read(buf)?;
                }
                Parsed(item, consumed) => {
                    assert!(consumed <= bytes_read);

                    let suffix_len = bytes_read - consumed;
                    let suffix_len = if suffix_len == 0 {
                        // Let's read a bit more to see if there is a suffix:
                        r.read(buf)?
                    } else {
                        suffix_len
                    };

                    return if suffix_len > 0 {
                        Err(UnexpectedInput)
                    } else {
                        Ok(item)
                    };
                }
            }
        }

        parser.finalize().ok_or(ExpectedMoreInput)
    }
}

impl<T> Parser for T where T: ParserCore {}
