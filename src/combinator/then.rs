use std::borrow::Cow;
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
    B: ToOwned,
{
    x: Either<X, X::Output>,
    y: Y,
    ph: PhantomData<B>,
}

impl<X, Y, B> Then<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: ToOwned,
{
    /// Create a sequential parser for `x` then `y`
    pub fn new(x: X, y: Y) -> Self {
        use Either::Left;

        Then {
            x: Left(x),
            y,
            ph: PhantomData,
        }
    }
}

impl<X, Y, B> ParserCore<B> for Then<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: ToOwned,
{
    type Output = (X::Output, Y::Output);
    type Error = Either<X::Error, Y::Error>;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error> {
        use std::borrow::Borrow;
        use Cow::{Borrowed, Owned};
        use Either::{Left, Right};
        use Update::{Parsed, Pending};

        let Then { x, y, ph } = self;

        match x {
            Left(xp) => match xp.feed(buffer).map_err_custom(Left)? {
                Pending(xp) => Ok(Pending(Then { x: Left(xp), y, ph })),
                Parsed(xout, Borrowed(xbuf)) => Then {
                    x: Right(xout),
                    y,
                    ph,
                }
                .feed(xbuf),
                Parsed(xout, Owned(xbuf)) => Then {
                    x: Right(xout),
                    y,
                    ph,
                }
                .feed(xbuf.borrow())
                .map_buffer(|cow| Owned(cow.into_owned())),
            },
            Right(xout) => match y.feed(buffer).map_err_custom(Right)? {
                Pending(y) => Ok(Pending(Then {
                    x: Right(xout),
                    y,
                    ph,
                })),
                Parsed(yout, ybuf) => Ok(Parsed((xout, yout), ybuf)),
            },
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        use crate::error::ParseError::ExpectedMoreInput;
        use Either::{Left, Right};

        let Then { x, y, .. } = self;

        let xoutopt = match x {
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
