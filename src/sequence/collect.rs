use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserBase, PushParser, Update};
use crate::sequence::SequenceParser;

/// Collect items emitted from [SequenceParser] `P` into container `C`
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

impl<P, C> ParserBase for Collect<P, C>
where
    P: SequenceParser,
    C: Extend<P::Item>,
{
    type Output = C;
    type Error = P::Error;

    fn pending_at_end(self) -> Option<Self::Output> {
        // BUG: strongly suspect an API design flaw around this
        let Collect {
            repeated,
            mut collection,
        } = self;

        if let Some(output) = repeated.pending_at_end() {
            if let Some((repeated, item)) = output {
                collection.extend_one(item);
                let nextself = Collect {
                    repeated,
                    collection,
                };
                nextself.pending_at_end()
            } else {
                Some(collection)
            }
        } else {
            None
        }
    }
}

impl<B, P, C> PushParser<B> for Collect<P, C>
where
    B: ?Sized + BufRef,
    P: SequenceParser + PushParser<B>,
    C: Extend<P::Item>,
{
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
}

#[cfg(test)]
mod tests;
