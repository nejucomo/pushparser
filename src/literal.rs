use std::{borrow::Cow, convert::Infallible};

use crate::{buffer::SplitBuffer, ParseError, ParseResult, ParserCore, Update};

#[derive(Copy, Clone, Debug)]
pub struct Literal<'s, B>
where
    B: ?Sized,
{
    value: &'s B,
    matchcnt: usize,
}

impl<'s, B> From<&'s B> for Literal<'s, B>
where
    B: ?Sized,
{
    fn from(value: &'s B) -> Self {
        Literal { value, matchcnt: 0 }
    }
}

impl<'s, B> ParserCore<B> for Literal<'s, B>
where
    B: ?Sized + SplitBuffer,
{
    type Output = &'s B;
    type Error = Infallible;

    fn feed(
        mut self,
        buffer: &B,
    ) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error> {
        use Cow::Borrowed;
        use Update::*;

        let (_, tomatch) = self.value.split_at(self.matchcnt);

        let mid = std::cmp::min(tomatch.len(), buffer.len());
        let (litprefix, litsuffix) = tomatch.split_at(mid);
        let (bufprefix, bufsuffix) = buffer.split_at(mid);

        if bufprefix == litprefix {
            if litsuffix.is_empty() {
                // We've reached the end of a match:
                Ok(Parsed(self.value, Borrowed(bufsuffix)))
            } else {
                // We haven't seen enough bytes to compare litprefix:
                assert!(bufsuffix.is_empty());
                assert_eq!(bufprefix.len(), buffer.len());
                self.matchcnt += buffer.len();
                Ok(Pending(self))
            }
        } else {
            Err(ParseError::UnexpectedInput)
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
