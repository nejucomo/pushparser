use std::{borrow::Cow, convert::Infallible};

use crate::error::{ParseError::UnexpectedInput, ParseResult};
use crate::parser::Update::{self, Pending};
use crate::{buffer::Buffer, parser::ParserCore};

pub fn end() -> End {
    End
}

#[derive(Debug)]
pub struct End;

impl<B> ParserCore<B> for End
where
    B: ?Sized + Buffer,
{
    type Output = End;
    type Error = Infallible;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error> {
        if buffer.is_empty() {
            Ok(Pending(Self))
        } else {
            Err(UnexpectedInput)
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        Ok(Some(End))
    }
}

#[cfg(test)]
mod tests;