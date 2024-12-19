use std::borrow::{Borrow, Cow};
use std::fmt::Debug;

use either::Either::{self, Left, Right};

use crate::buffer::BacktrackBuffer;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::ParserCore;
use crate::parser::Update::{self, Parsed, Pending};

use self::Inner::*;

/// Parse either `X` or `Y` yielding one of their outputs
#[derive(Debug)]
pub struct Or<X, Y, B>(Inner<X, Y, B>)
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BacktrackBuffer,
    B::Owned: Default + Debug;

#[derive(Debug)]
enum Inner<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BacktrackBuffer,
    B::Owned: Default + Debug,
{
    ParsingX { x: X, backtrack: B::Owned, y: Y },
    ParsingY(Y),
}

impl<X, Y, B> Or<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BacktrackBuffer,
    B::Owned: Default + Debug,
{
    /// Create an alternative parser for `x` or `y`
    pub fn new(x: X, y: Y) -> Self {
        Or(Inner::ParsingX {
            x,
            backtrack: B::Owned::default(),
            y,
        })
    }
}

impl<X, Y, B> ParserCore<B> for Or<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BacktrackBuffer,
    B::Owned: Default + Debug,
{
    type Output = Either<X::Output, Y::Output>;
    type Error = Y::Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error> {
        self.0.feed(buffer).map_pending(Or)
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        self.0.finalize()
    }
}

impl<X, Y, B> ParserCore<B> for Inner<X, Y, B>
where
    X: ParserCore<B>,
    Y: ParserCore<B>,
    B: BacktrackBuffer,
    B::Owned: Default + Debug,
{
    type Output = Either<X::Output, Y::Output>;
    type Error = Y::Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error> {
        match self {
            ParsingX {
                x,
                mut backtrack,
                y,
            } => {
                buffer.push_onto(&mut backtrack);
                match x.feed(buffer) {
                    Ok(update) => Ok(update
                        .map_pending(|x| ParsingX { x, backtrack, y })
                        .map_output(Left)),

                    Err(_) => ParsingY(y)
                        .feed(backtrack.borrow())
                        .map_buffer(|cow| Cow::Owned(cow.into_owned())),
                }
            }

            ParsingY(y) => y.feed(buffer).map_pending(ParsingY).map_output(Right),
        }
    }

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error> {
        match self {
            ParsingX { x, backtrack, y } => {
                if let Some(xval) = x.finalize().unwrap_or(None) {
                    Ok(Some(Left(xval)))
                } else {
                    match y.feed(backtrack.borrow())? {
                        Pending(y) => Inner::<X, Y, B>::ParsingY(y).finalize(),
                        Parsed(yval, ybuf) => {
                            if ybuf.is_empty() {
                                Ok(Some(Right(yval)))
                            } else {
                                todo!("unhandled case: Or::finalize() produces ybuf.");
                            }
                        }
                    }
                }
            }
            ParsingY(y) => y.finalize().map(|opt| opt.map(Right)),
        }
    }
}
