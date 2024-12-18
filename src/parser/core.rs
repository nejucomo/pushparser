use std::borrow::Cow;

use crate::error::ParseResult;
use crate::parser::Update;

/// The core parser functionality which must be implemented for new parsers
pub trait ParserCore<B>: Sized
where
    B: ?Sized + ToOwned,
{
    /// The type of parsed value
    type Output;
    /// The custom error type
    type Error;

    /// Feed some input to this parser to produce an update (or error)
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output, Cow<'_, B>>, Self::Error>;

    /// Indicate to the parser the end of input
    ///
    /// Some parsers produce a value only upon end-of-input, for example the parser that matches any number of `'x'` chars in a string.
    fn finalize(self) -> ParseResult<Option<Self::Output>, Self::Error>;
}
