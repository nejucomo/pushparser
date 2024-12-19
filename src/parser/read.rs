use std::io::Read;

use either::Either;

use crate::buffer::BufferManager;
use crate::error::{ParseResult, ParseResultExt};
use crate::parser::Outcome::{Next, Parsed};
use crate::parser::ParserCore;

/// A consumer interface that can parse any sync I/O [std::io::Read] type
///
/// Any [ParserCore] with `[u8]` input is a [ReadParser] by blanket impl.
pub trait ReadParser: ParserCore<[u8]> {
    /// Read `r` to end of file and parse it using a buffer with a default size
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

    /// Read `r` to end of file and parse it using a buffer with the allocated size
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
        use Either::{Left, Right};

        let mut parser = self;
        let mut bufmgr = BufferManager::from(vec![0u8; bufsize]);

        loop {
            let writeslice = bufmgr.get_write_slice();
            let readcnt = r.read(writeslice).map_err(Right)?;
            match bufmgr.process_write(parser, readcnt).map_err_custom(Left)? {
                Next(next) => {
                    parser = next;
                }
                Parsed(output) => {
                    return Ok(output);
                }
            }
        }
    }
}
