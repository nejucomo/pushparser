use std::fmt::Debug;
use std::marker::PhantomData;

use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserCore, Update};

/// Convert the output of parser `P` with fn `F`
pub struct Foldl<P, F, A, X, B>
where
    B: ?Sized + BufRef,
    P: ParserCore<B, Output = Option<(P, X)>>,
    F: Fn(A, X) -> A,
{
    parser: P,
    acc: A,
    f: F,
    phantom: PhantomData<(X, B)>,
}

impl<P, F, A, X, B> Foldl<P, F, A, X, B>
where
    B: ?Sized + BufRef,
    P: ParserCore<B, Output = Option<(P, X)>>,
    F: Fn(A, X) -> A,
{
    /// Create a new foldl parser
    pub fn new(parser: P, acc: A, f: F) -> Self {
        Foldl {
            parser,
            acc,
            f,
            phantom: PhantomData,
        }
    }
}

impl<P, F, A, X, B> Clone for Foldl<P, F, A, X, B>
where
    B: ?Sized + BufRef,
    P: Clone + ParserCore<B, Output = Option<(P, X)>>,
    A: Clone,
    F: Clone + Fn(A, X) -> A,
{
    fn clone(&self) -> Self {
        Foldl::new(self.parser.clone(), self.acc.clone(), self.f.clone())
    }
}

impl<P, F, A, X, B> ParserCore<B> for Foldl<P, F, A, X, B>
where
    B: ?Sized + BufRef,
    P: Debug + ParserCore<B, Output = Option<(P, X)>>,
    F: Fn(A, X) -> A,
    A: Debug,
    X: Debug,
{
    type Output = A;
    type Error = P::Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let Foldl { parser, acc, f, .. } = self;

        parser
            .feed(buffer)
            .and_then(|Update { consumed, outcome }| match outcome {
                Next(parser) => Ok(Update {
                    consumed,
                    outcome: Next(Foldl::new(parser, acc, f)),
                }),
                Parsed(None) => Ok(Update {
                    consumed,
                    outcome: Parsed(acc),
                }),
                Parsed(Some((parser, x))) => {
                    let intermediate = Foldl::new(parser, f(acc, x), f);
                    let subup = intermediate.feed(buffer.drop_up_to(consumed))?;
                    Ok(Update {
                        consumed: consumed + subup.consumed,
                        outcome: subup.outcome,
                    })
                }
            })
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        let Foldl { parser, acc, f, .. } = self;

        if let Some(Some((p, x))) = parser.finalize(buffer)? {
            return Foldl::new(p, f(acc, x), f).finalize(buffer.drop_up_to(buffer.len()));
        }
        Ok(Some(acc))
    }
}

impl<P, F, A, X, B> Debug for Foldl<P, F, A, X, B>
where
    B: ?Sized + BufRef,
    P: Debug + ParserCore<B, Output = Option<(P, X)>>,
    F: Fn(A, X) -> A,
    A: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Foldl")
            .field("parser", &self.parser)
            .field("acc", &self.acc)
            .field("f", &"...")
            .finish()
    }
}

#[cfg(test)]
mod tests;
