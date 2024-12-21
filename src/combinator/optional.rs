use std::convert::Infallible;

use crate::buffer::BufRef;
use crate::combinator::Backtrack;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

/// Attempt to parse `P`, or else yield `None`
#[derive(Debug)]
pub struct Optional<P>(Backtrack<P>);

impl<P> From<P> for Optional<P> {
    fn from(parser: P) -> Self {
        Optional(Backtrack::from(parser))
    }
}

impl<P> ParserBase for Optional<P>
where
    P: ParserBase,
{
    type Output = Option<P::Output>;
    type Error = Infallible;

    fn pending_at_end(self) -> Option<Self::Output> {
        self.0.pending_at_end().map(Some)
    }
}

impl<B, P> PushParser<B> for Optional<P>
where
    B: ?Sized + BufRef,
    P: PushParser<B>,
{
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
}

#[cfg(test)]
mod tests;
