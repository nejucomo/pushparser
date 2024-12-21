use std::convert::Infallible;

use crate::buffer::BufRef;
use crate::combinator::Optional;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

/// Parse `P` repeatedly
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

impl<P, B> ParserCore<B> for Repeated<P>
where
    P: Clone + ParserCore<B>,
    B: ?Sized + BufRef,
{
    type Output = Option<(Self, P::Output)>;
    type Error = Infallible;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let Repeated { template, current } = self;

        current.feed(buffer).map_outcome(|oc| match oc {
            Next(current) => Next(Repeated { template, current }),
            Parsed(optout) => Parsed(emit_output(template)(optout)),
        })
    }

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        let Repeated { template, current } = self;

        current
            .finalize(buffer)
            .map(|optoptout| optoptout.map(emit_output(template)))
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
