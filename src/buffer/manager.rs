use crate::buffer::Buffer;
use crate::error::ParseResult;
use crate::parser::Outcome;
use crate::parser::{ParserCore, Update};

/// Manage the buffering necessary for driving [ParserCore] in an i/o agnostic manner
#[derive(Debug)]
pub struct BufferManager<B>
where
    B: Buffer,
{
    buffer: B,
    rstart: usize,
}

impl<B> Default for BufferManager<B>
where
    B: Default + Buffer,
{
    fn default() -> Self {
        BufferManager::from(B::default())
    }
}

impl<B> From<B> for BufferManager<B>
where
    B: Buffer,
{
    fn from(buffer: B) -> Self {
        BufferManager { buffer, rstart: 0 }
    }
}

impl<B> BufferManager<B>
where
    B: Buffer,
{
    /// Get a writable byte slice for inserting new data
    pub fn get_write_slice(&mut self) -> &mut [u8] {
        let mslice = self.buffer.as_mut();
        &mut mslice[self.rstart..]
    }

    /// Process newly inserted data
    pub fn process_write<P>(
        &mut self,
        parser: P,
        readcnt: usize,
    ) -> ParseResult<Outcome<P, P::Output>, P::Error>
    where
        P: ParserCore<[u8]>,
    {
        let end = self.rstart + readcnt;
        let rslice = &self.buffer.as_ref()[..end];

        let Update { consumed, outcome } = parser.feed(rslice)?;
        let mslice = self.buffer.as_mut();
        mslice.rotate_left(consumed);
        self.rstart = readcnt;

        Ok(outcome)
    }
}
