use either::Either;

use crate::error::{ParseResult, ParseResultExt, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

/// Parse two subgrammars in sequence, `X` then `Y`, yielding both of their outputs
#[derive(Debug)]
pub struct Then<X, Y>
where
    X: ParserBase,
    Y: ParserBase,
{
    xporv: Either<X, X::Output>,
    y: Y,
}

impl<X, Y> Then<X, Y>
where
    X: ParserBase,
    Y: ParserBase,
{
    /// Create a sequential parser for `x` then `y`
    pub fn new(x: X, y: Y) -> Self {
        use Either::Left;

        Then { xporv: Left(x), y }
    }
}

impl<X, Y> ParserBase for Then<X, Y>
where
    X: ParserBase,
    Y: ParserBase,
{
    type Output = (X::Output, Y::Output);
    type Error = Either<X::Error, Y::Error>;

    fn pending_at_end(self) -> Option<Self::Output> {
        let Then { xporv, y } = self;

        let xval = xporv
            .map_left(|x| x.pending_at_end())
            .map_right(Some)
            .into_inner()?;

        let yval = y.pending_at_end()?;

        Some((xval, yval))
    }
}

impl<B, X, Y> PushParser<B> for Then<X, Y>
where
    X: PushParser<B>,
    Y: PushParser<B>,
    B: ?Sized,
{
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let Then { xporv, y } = self;

        match xporv {
            Left(xparser) => xparser
                .feed(buffer)
                .map_err_custom(Left)
                .map_outcome(|outcome| match outcome {
                    Next(xparser) => Next(Then {
                        xporv: Left(xparser),
                        y,
                    }),
                    Parsed(xout) => Next(Then {
                        xporv: Right(xout),
                        y,
                    }),
                }),
            Right(xout) => {
                y.feed(buffer)
                    .map_err_custom(Right)
                    .map_outcome(|outcome| match outcome {
                        Next(y) => Next(Then {
                            xporv: Right(xout),
                            y,
                        }),
                        Parsed(yout) => Parsed((xout, yout)),
                    })
            }
        }
    }
}
