use crate::error::ParseResult;
use crate::parser::Outcome;
use crate::parser::{ParserCore, Update};

/// Manage the buffering necessary for driving [ParserCore] in an i/o agnostic manner
#[derive(Debug)]
pub struct BufferManager {
    buffer: Vec<u8>,
    rstart: usize,
}

impl From<Vec<u8>> for BufferManager {
    fn from(buffer: Vec<u8>) -> Self {
        BufferManager { buffer, rstart: 0 }
    }
}

impl BufferManager {
    /// Get a writable byte slice for inserting new data
    pub fn get_write_slice(&mut self) -> &mut [u8] {
        if self.rstart == self.buffer.len() {
            // The parser is using the entire buffer for storage, so let's grow for new input:
            self.buffer.resize(self.buffer.len() * 2, 0);
        }
        &mut self.buffer[self.rstart..]
    }

    /// Process newly inserted data
    ///
    /// # Diagram
    ///
    /// ```text
    ///             rstart-+           +-end
    ///                    | _readcnt_ |
    ///                    v/         \v
    ///        +-----------+-----------+--------+
    /// buffer | prev-kept | new       | uninit |
    ///        +-----------+----+------+--------+
    /// rslice |    consumed    | kept |
    ///        +------+---------+------+--------+
    /// rotate | kept | uninit                  |
    ///        +------+-------------------------+
    /// ```
    pub fn process_write<P>(
        &mut self,
        parser: P,
        readcnt: usize,
    ) -> ParseResult<Outcome<P, P::Output>, P::Error>
    where
        P: ParserCore<[u8]> + std::fmt::Debug,
        P::Output: std::fmt::Debug,
        P::Error: std::fmt::Debug,
    {
        use crate::error::ParseError::ExpectedMoreInput;
        use Outcome::Parsed;

        let end = self.rstart + readcnt;
        let rslice = &self.buffer[..end];

        if readcnt == 0 {
            let optoutput = parser.finalize(rslice)?;
            let output = optoutput.ok_or(ExpectedMoreInput)?;
            Ok(Parsed(output))
        } else {
            let Update { consumed, outcome } = parser.feed(rslice)?;

            self.buffer.rotate_left(consumed);
            self.rstart = end - consumed;

            Ok(outcome)
        }
    }
}
