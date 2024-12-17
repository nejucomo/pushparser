use std::borrow::Cow;

use crate::{ParseResult, Update};

/// The core parser functionality which must be implemented for new parsers
pub trait ParserCore<B>: Sized
where
    B: ?Sized + ToOwned,
{
    type Output;
    type Error;

    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error>;

    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error>;
}
