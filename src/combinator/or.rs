use std::fmt::Debug;

use either::Either::{self, Left, Right};

use crate::buffer::BufRef;
use crate::combinator::Backtrack;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

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

impl<B, X, Y> ParserCore<B> for Or<X, Y>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BufRef,
{
    type Output = Either<X::Output, Y::Output>;
    type Error = Y::Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        let Or { xbt, y } = self;

        if let Some(xbt) = xbt {
            if let Some(update) = xbt.feed(buffer).ok() {
                return Ok(update
                    .map_next(|xbt| Or { xbt: Some(xbt), y })
                    .map_output(Left));
            }
        }

        y.feed(buffer)
            .map_next(|y| Or { xbt: None, y })
            .map_output(Right)
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        todo!()
    }
}
