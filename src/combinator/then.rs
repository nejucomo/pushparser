use std::marker::PhantomData;

use either::Either;

use crate::error::{ParseResult, ParseResultExt, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

/// Parse two subgrammars in sequence, `X` then `Y`, yielding both of their outputs
#[derive(Debug)]
pub struct Then<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
{
    xporv: Either<X, X::Output>,
    y: Y,
    ph: PhantomData<B>,
}

impl<X, Y, B> Then<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
{
    /// Create a sequential parser for `x` then `y`
    pub fn new(x: X, y: Y) -> Self {
        use Either::Left;

        Then {
            xporv: Left(x),
            y,
            ph: PhantomData,
        }
    }
}

impl<X, Y, B> ParserCore<B> for Then<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
{
    type Output = (X::Output, Y::Output);
    type Error = Either<X::Error, Y::Error>;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};
        use Either::{Left, Right};

        let Then { xporv, y, ph } = self;

        match xporv {
            Left(xparser) => xparser
                .feed(buffer)
                .map_err_custom(Left)
                .map_outcome(|outcome| match outcome {
                    Next(xparser) => Next(Then {
                        xporv: Left(xparser),
                        y,
                        ph,
                    }),
                    Parsed(xout) => Next(Then {
                        xporv: Right(xout),
                        y,
                        ph,
                    }),
                }),
            Right(xout) => {
                y.feed(buffer)
                    .map_err_custom(Right)
                    .map_outcome(|outcome| match outcome {
                        Next(y) => Next(Then {
                            xporv: Right(xout),
                            y,
                            ph,
                        }),
                        Parsed(yout) => Parsed((xout, yout)),
                    })
            }
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        use crate::error::ParseError::ExpectedMoreInput;
        use Either::{Left, Right};

        let Then { xporv, y, .. } = self;

        let xoutopt = match xporv {
            Left(xp) => xp.finalize().map_err_custom(Left)?,
            Right(xout) => Some(xout),
        };

        let youtopt = y.finalize().map_err_custom(Right)?;

        match (xoutopt, youtopt) {
            // Both finalize to a value, so we finalize to a value:
            (Some(x), Some(y)) => Ok(Some((x, y))),
            // X finalized to None, so Y's result is irrelevant:
            (None, _) => Ok(None),
            // Because X was a value, a lack of Y value is an error:
            (Some(_), None) => Err(ExpectedMoreInput),
        }
    }
}
