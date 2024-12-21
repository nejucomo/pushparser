use std::fmt::Debug;

use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserBase, PushParser, Update};
use crate::sequence::SequenceParser;

/// Convert a [SequenceParser] into a parser that folds each item into a final output
pub struct Foldl<S, F, A>
where
    S: SequenceParser,
    F: Fn(A, S::Item) -> A,
{
    parser: S,
    acc: A,
    f: F,
}

impl<S, F, A> Foldl<S, F, A>
where
    S: SequenceParser,
    F: Fn(A, S::Item) -> A,
{
    /// Create a new foldl parser
    pub fn new(parser: S, acc: A, f: F) -> Self {
        Foldl { parser, acc, f }
    }
}

impl<S, F, A> Clone for Foldl<S, F, A>
where
    S: Clone + SequenceParser,
    A: Clone,
    F: Clone + Fn(A, S::Item) -> A,
{
    fn clone(&self) -> Self {
        Foldl::new(self.parser.clone(), self.acc.clone(), self.f.clone())
    }
}

impl<S, F, A> ParserBase for Foldl<S, F, A>
where
    S: SequenceParser,
    F: Fn(A, S::Item) -> A,
{
    type Output = A;
    type Error = S::Error;

    fn pending_at_end(self) -> Option<Self::Output> {
        let Foldl { parser, acc, f, .. } = self;

        if let Some(pending) = parser.pending_at_end() {
            if let Some((p, x)) = pending {
                Foldl::new(p, f(acc, x), f).pending_at_end()
            } else {
                None
            }
        } else {
            Some(acc)
        }
    }
}

impl<B, S, F, A> PushParser<B> for Foldl<S, F, A>
where
    B: ?Sized + BufRef,
    S: Debug + SequenceParser + PushParser<B>,
    F: Fn(A, S::Item) -> A,
    A: Debug,
    S::Item: Debug,
{
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
}

impl<S, F, A> Debug for Foldl<S, F, A>
where
    S: Debug + SequenceParser,
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
