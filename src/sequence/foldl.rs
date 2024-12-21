use std::fmt::Debug;
use std::marker::PhantomData;

use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserCore, Update};
use crate::sequence::SequenceParser;

/// Convert the output of parser `P` with fn `F`
pub struct Foldl<S, F, A, B>
where
    B: ?Sized + BufRef,
    S: SequenceParser<B>,
    F: Fn(A, S::Item) -> A,
{
    parser: S,
    acc: A,
    f: F,
    phantom: PhantomData<(S::Item, B)>,
}

impl<S, F, A, B> Foldl<S, F, A, B>
where
    B: ?Sized + BufRef,
    S: SequenceParser<B>,
    F: Fn(A, S::Item) -> A,
{
    /// Create a new foldl parser
    pub fn new(parser: S, acc: A, f: F) -> Self {
        Foldl {
            parser,
            acc,
            f,
            phantom: PhantomData,
        }
    }
}

impl<S, F, A, B> Clone for Foldl<S, F, A, B>
where
    B: ?Sized + BufRef,
    S: Clone + SequenceParser<B>,
    A: Clone,
    F: Clone + Fn(A, S::Item) -> A,
{
    fn clone(&self) -> Self {
        Foldl::new(self.parser.clone(), self.acc.clone(), self.f.clone())
    }
}

impl<S, F, A, B> ParserCore<B> for Foldl<S, F, A, B>
where
    B: ?Sized + BufRef,
    S: Debug + SequenceParser<B>,
    F: Fn(A, S::Item) -> A,
    A: Debug,
    S::Item: Debug,
{
    type Output = A;
    type Error = S::Error;

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

impl<S, F, A, B> Debug for Foldl<S, F, A, B>
where
    B: ?Sized + BufRef,
    S: Debug + SequenceParser<B>,
    F: Fn(A, S::Item) -> A,
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
