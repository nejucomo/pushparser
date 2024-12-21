use std::convert::Infallible;

use crate::buffer::BufRef;
use crate::combinator::Optional;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

#[cfg(doc)]
use crate::sequence::SequenceParser;

/// Parse `P` repeatedly as a [SequenceParser] with `Item = P::Output`
#[derive(Debug)]
pub struct Repeated<P>
where
    P: Clone,
{
    template: P,
    current: Optional<P>,
}

impl<P> From<P> for Repeated<P>
where
    P: Clone,
{
    fn from(template: P) -> Self {
        let current = Optional::from(template.clone());
        Repeated { template, current }
    }
}

impl<P> ParserBase for Repeated<P>
where
    P: Clone + ParserBase,
{
    type Output = Option<(Self, P::Output)>;
    type Error = Infallible;

    fn pending_at_end(self) -> Option<Self::Output> {
        let Repeated { template, current } = self;

        current.pending_at_end().map(emit_output(template))
    }
}

impl<P, B> PushParser<B> for Repeated<P>
where
    P: Clone + PushParser<B>,
    B: ?Sized + BufRef,
{
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let Repeated { template, current } = self;

        current.feed(buffer).map_outcome(|oc| match oc {
            Next(current) => Next(Repeated { template, current }),
            Parsed(optout) => Parsed(emit_output(template)(optout)),
        })
    }
}

fn emit_output<P, X>(template: P) -> impl FnOnce(Option<X>) -> Option<(Repeated<P>, X)>
where
    P: Clone,
{
    |optout| optout.map(|x| (Repeated::from(template), x))
}

#[cfg(test)]
mod tests;
