use crate::combinator::Then;
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
}
