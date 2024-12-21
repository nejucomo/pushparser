use std::marker::PhantomData;

use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserBase, PushParser, Update};

/// Convert the output of parser `P` with fn `F`
#[derive(Debug)]
pub struct MapOutput<P, F, O>
where
    P: ParserBase,
    F: FnOnce(P::Output) -> O,
{
    parser: P,
    map: F,
    phantom: PhantomData<O>,
}

impl<P, F, O> MapOutput<P, F, O>
where
    P: ParserBase,
    F: FnOnce(P::Output) -> O,
{
    /// Construct a new `MapOutput`
    pub fn new(parser: P, map: F) -> Self {
        MapOutput {
            parser,
            map,
            phantom: PhantomData,
        }
    }
}

impl<P, F, O> Clone for MapOutput<P, F, O>
where
    P: Clone + ParserBase,
    F: Clone + FnOnce(P::Output) -> O,
{
    fn clone(&self) -> Self {
        MapOutput {
            parser: self.parser.clone(),
            map: self.map.clone(),
            phantom: PhantomData,
        }
    }
}

impl<P, F, O> ParserBase for MapOutput<P, F, O>
where
    P: ParserBase,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;

    fn pending_at_end(self) -> Option<Self::Output> {
        self.parser.pending_at_end().map(self.map)
    }
}

impl<B, P, F, O> PushParser<B> for MapOutput<P, F, O>
where
    B: ?Sized,
    P: PushParser<B>,
    F: FnOnce(P::Output) -> O,
{
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let MapOutput {
            parser,
            map,
            phantom,
        } = self;

        parser.feed(buffer).map_outcome(|oc| match oc {
            Next(parser) => Next(MapOutput {
                parser,
                map,
                phantom,
            }),
            Parsed(output) => Parsed(map(output)),
        })
    }
}

#[cfg(test)]
mod tests;
