use crate::buffer::BufRef;
use crate::combinator::{MapOutput, Optional, Or, Repeated, Then};
use crate::parser::ParserCore;

/// The primary composition interface for push parsers
pub trait PushParser<B>: ParserCore<B>
where
    B: ?Sized,
{
    /// Convert this output once parsed
    fn map_output<F, O>(self, f: F) -> MapOutput<Self, F, O, B>
    where
        F: FnOnce(Self::Output) -> O,
    {
        MapOutput::new(self, f)
    }

    /// Parse `self` then `next` in sequence, yielding `(Self::Output, P::Output)`
    fn then<P>(self, next: P) -> Then<Self, P, B>
    where
        P: ParserCore<B>,
    {
        Then::new(self, next)
    }

    /// Parse either `self` or `alternative`, yielding `Either<Self::Output, P::Output>`
    fn or<P>(self, alternative: P) -> Or<Self, P>
    where
        P: ParserCore<B>,
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

impl<B, P> PushParser<B> for P
where
    B: ?Sized,
    P: ParserCore<B>,
{
}
