use std::convert::Infallible;

use crate::buffer::BufRef;
use crate::error::{ParseError::UnexpectedInput, ParseResult};
use crate::parser::{ParserCore, Update};

/// Construct the [End] parser, which only succeeds on an empty end of input
pub fn end() -> End {
    End
}

/// The [End] parser only succeeds on an empty end of input
#[derive(Debug)]
pub struct End;

impl<B> ParserCore<B> for End
where
    B: ?Sized + BufRef,
{
    type Output = End;
    type Error = Infallible;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::Next;

        check_empty(buffer)?;
        Ok(Update {
            consumed: 0,
            outcome: Next(Self),
        })
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        check_empty(buffer)?;
        Ok(Some(End))
    }
}

fn check_empty<B, E>(buffer: &B) -> ParseResult<(), E>
where
    B: ?Sized + BufRef,
{
    if buffer.is_empty() {
        Ok(())
    } else {
        Err(UnexpectedInput)
    }
}

#[cfg(test)]
mod tests;
