use crate::buffer::BufRef;
use crate::combinator::{MapOutput, Optional, Or, Repeated, Then};
use crate::error::ParseResult;
use crate::parser::{ParserBase, Update};

/// The primary composition interface for push parsers
pub trait PushParser<B>: ParserBase
where
    B: ?Sized,
{
    /// Feed a buffer reference to the parser to produce an update
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error>;

    /// Convert this output once parsed
    fn map_output<F, O>(self, f: F) -> MapOutput<Self, F, O>
    where
        F: FnOnce(Self::Output) -> O,
    {
        MapOutput::new(self, f)
    }

    /// Parse `self` then `next` in sequence, yielding `(Self::Output, P::Output)`
    fn then<P>(self, next: P) -> Then<Self, P>
    where
        P: PushParser<B>,
    {
        Then::new(self, next)
    }

    /// Parse either `self` or `alternative`, yielding `Either<Self::Output, P::Output>`
    fn or<P>(self, alternative: P) -> Or<Self, P>
    where
        P: PushParser<B>,
    {
        Or::new(self, alternative)
    }

    /// Attempt to parse `self`, or else proceed successfully without consuming anything, yielding `Option<Self::Output>`
    fn optional(self) -> Optional<Self>
    where
        B: BufRef,
    {
        Optional::from(self)
    }

    /// Parse `self` multiple times
    fn repeated(self) -> Repeated<Self>
    where
        Self: Clone,
    {
        Repeated::from(self)
    }
}
