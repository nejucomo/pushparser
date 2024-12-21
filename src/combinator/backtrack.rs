use crate::buffer::BufRef;
use crate::error::ParseResult;
use crate::parser::{ParserBase, PushParser, Update};

/// A [Backtrack] parser holds onto all of the input buffer until the inner parser completes
#[derive(Debug)]
pub struct Backtrack<P> {
    parser: P,
    offset: usize,
}

impl<P> From<P> for Backtrack<P> {
    fn from(parser: P) -> Self {
        Backtrack { parser, offset: 0 }
    }
}

impl<P> ParserBase for Backtrack<P>
where
    P: ParserBase,
{
    type Output = P::Output;
    type Error = P::Error;

    fn pending_at_end(self) -> Option<Self::Output> {
        self.parser.pending_at_end()
    }
}

impl<B, P> PushParser<B> for Backtrack<P>
where
    B: ?Sized + BufRef,
    P: PushParser<B>,
{
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error> {
        use crate::parser::Outcome::{Next, Parsed};

        let Backtrack { parser, offset } = self;
        let subbuf = buffer.drop_up_to(offset);
        let Update { consumed, outcome } = parser.feed(subbuf)?;
        match outcome {
            Next(parser) => Ok(Update {
                consumed: 0,
                outcome: Next(Backtrack {
                    parser,
                    offset: offset + consumed,
                }),
            }),
            Parsed(output) => Ok(Update {
                consumed: offset + consumed,
                outcome: Parsed(output),
            }),
        }
    }
}
