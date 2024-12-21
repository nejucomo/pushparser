use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserCore, Update};
use crate::sequence::SequenceParser;

/// Collect items emitted from parser `P` into container `C`
#[derive(Debug)]
pub struct Collect<P, C> {
    repeated: P,
    collection: C,
}

impl<P, C> From<P> for Collect<P, C>
where
    C: Default,
{
    fn from(repeated: P) -> Self {
        Collect {
            repeated,
            collection: C::default(),
        }
    }
}

impl<B, P, C> ParserCore<B> for Collect<P, C>
where
    B: ?Sized + BufRef,
    P: SequenceParser<B>,
    C: Extend<P::Item>,
{
    type Output = C;
    type Error = P::Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let Collect {
            repeated,
            mut collection,
        } = self;

        repeated
            .feed(buffer)
            .and_then(|Update { consumed, outcome }| match outcome {
                Next(repeated) => Ok(Update {
                    consumed,
                    outcome: Next(Collect {
                        repeated,
                        collection,
                    }),
                }),
                Parsed(None) => Ok(Update {
                    consumed,
                    outcome: Parsed(collection),
                }),
                Parsed(Some((repeated, item))) => {
                    collection.extend_one(item);
                    let intermediate = Collect {
                        repeated,
                        collection,
                    };
                    let subup = intermediate.feed(buffer.drop_up_to(consumed))?;
                    Ok(Update {
                        consumed: consumed + subup.consumed,
                        outcome: subup.outcome,
                    })
                }
            })
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        let Collect {
            repeated,
            mut collection,
        } = self;

        if let Some(output) = repeated.finalize(buffer)? {
            if let Some((repeated, item)) = output {
                collection.extend_one(item);
                let nextself = Collect {
                    repeated,
                    collection,
                };
                nextself.finalize(buffer.drop_up_to(buffer.len()))
            } else {
                Ok(Some(collection))
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests;
