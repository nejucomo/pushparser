use std::fmt::Debug;

use either::Either::{self, Left, Right};

use crate::buffer::BufRef;
use crate::combinator::Backtrack;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

/// Parse either `X` or `Y` yielding one of their outputs
#[derive(Debug)]
pub struct Or<X, Y> {
    xbt: Option<Backtrack<X>>,
    y: Y,
}

impl<X, Y> Or<X, Y> {
    /// Construct a new alternative parser
    pub fn new(x: X, y: Y) -> Self {
        Or {
            xbt: Some(Backtrack::from(x)),
            y,
        }
    }
}

impl<X, Y> ParserBase for Or<X, Y>
where
    X: ParserBase,
    Y: ParserBase,
{
    type Output = Either<X::Output, Y::Output>;
    type Error = Y::Error;

    fn pending_at_end(self) -> Option<Self::Output> {
        let Or { xbt, y } = self;
        xbt.and_then(|x| x.pending_at_end().map(Left))
            .or_else(|| y.pending_at_end().map(Right))
    }
}

impl<B, X, Y> PushParser<B> for Or<X, Y>
where
    X: PushParser<B>,
    Y: PushParser<B>,
    B: BufRef,
{
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        let Or { xbt, y } = self;

        if let Some(xbt) = xbt {
            if let Ok(update) = xbt.feed(buffer) {
                return Ok(update
                    .map_next(|xbt| Or { xbt: Some(xbt), y })
                    .map_output(Left));
            }
        }

        y.feed(buffer)
            .map_next(|y| Or { xbt: None, y })
            .map_output(Right)
    }
}
