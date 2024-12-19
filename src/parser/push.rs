use crate::combinator::{Or, Then};
use crate::parser::ParserCore;

/// The primary composition interface for push parsers
pub trait PushParser<B>: ParserCore<B>
where
    B: ToOwned,
{
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
}
