use std::marker::PhantomData;

use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::{ParserCore, Update};

/// Convert the output of parser `P` with fn `F`
#[derive(Debug)]
pub struct MapOutput<P, F, O, B>
where
    B: ?Sized,
    P: ParserCore<B>,
    F: FnOnce(P::Output) -> O,
{
    parser: P,
    map: F,
    phantom: PhantomData<(O, B)>,
}

impl<P, F, O, B> MapOutput<P, F, O, B>
where
    B: ?Sized,
    P: ParserCore<B>,
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

impl<P, F, O, B> Clone for MapOutput<P, F, O, B>
where
    B: ?Sized,
    P: Clone + ParserCore<B>,
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

impl<P, F, O, B> ParserCore<B> for MapOutput<P, F, O, B>
where
    B: ?Sized,
    P: ParserCore<B>,
    F: FnOnce(P::Output) -> O,
{
    type Output = O;
    type Error = P::Error;

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

    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error> {
        let MapOutput { parser, map, .. } = self;

        parser.finalize(buffer).map(|optval| optval.map(map))
    }
}

#[cfg(test)]
mod tests;
