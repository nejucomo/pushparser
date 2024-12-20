use std::convert::Infallible;

use crate::buffer::BufRef;
use crate::combinator::Backtrack;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

/// Attempt to parse `P`, or else yield `None`
#[derive(Debug)]
pub struct Optional<P>(Backtrack<P>);

impl<P> From<P> for Optional<P> {
    fn from(parser: P) -> Self {
        Optional(Backtrack::from(parser))
    }
}

impl<B, P> ParserCore<B> for Optional<P>
where
    B: ?Sized + BufRef,
    P: ParserCore<B>,
{
    type Output = Option<P::Output>;
    type Error = Infallible;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::Parsed;

        Ok(self
            .0
            .feed(buffer)
            .map_next(Optional)
            .map_output(Some)
            .unwrap_or(Update {
                consumed: 0,
                outcome: Parsed(None),
            }))
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        Ok(self.0.finalize(buffer).unwrap_or(None).map(Some))
    }
}

#[cfg(test)]
mod tests;
