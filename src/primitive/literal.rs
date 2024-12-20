use std::convert::Infallible;

use crate::error::ParseError::{ExpectedMoreInput, UnexpectedInput};
use crate::error::ParseResult;
use crate::parser::Update;
use crate::{buffer::BufRef, parser::ParserCore};

/// Construct a [Literal] which parses input which exactly matches its value
pub fn literal<B>(value: &B) -> Literal<'_, B> {
    Literal::from(value)
}

/// A [Literal] parses input which exactly matches its value
#[derive(Copy, Debug)]
pub struct Literal<'s, B>
where
    B: ?Sized,
{
    value: &'s B,
    matchcnt: usize,
}

impl<B> Clone for Literal<'_, B>
where
    B: ?Sized,
{
    fn clone(&self) -> Self {
        Literal {
            value: self.value,
            matchcnt: self.matchcnt,
        }
    }
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
    B: ?Sized + BufRef,
{
    type Output = &'s B;
    type Error = Infallible;

    fn feed(mut self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let (_, tomatch) = self.value.split_at(self.matchcnt);

        let consumed = std::cmp::min(tomatch.len(), buffer.len());
        let (litprefix, litsuffix) = tomatch.split_at(consumed);
        let (bufprefix, bufsuffix) = buffer.split_at(consumed);

        if bufprefix == litprefix {
            if litsuffix.is_empty() {
                // We've reached the end of a match:
                Ok(Update {
                    consumed,
                    outcome: Parsed(self.value),
                })
            } else {
                // We haven't seen enough bytes to compare litprefix.
                // So we should have read the whole buffer:
                assert!(bufsuffix.is_empty());
                let consumed = bufprefix.len();
                assert_eq!(consumed, buffer.len());

                self.matchcnt += consumed;
                Ok(Update {
                    consumed,
                    outcome: Next(self),
                })
            }
        } else {
            Err(UnexpectedInput)
        }
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        assert!(buffer.is_empty());
        Err(ExpectedMoreInput)
    }
}

#[cfg(test)]
mod tests;
