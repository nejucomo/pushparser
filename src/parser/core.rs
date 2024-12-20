use crate::error::ParseResult;
use crate::parser::Update;

#[cfg(doc)]
use crate::parser::PushParser;

/// The core parser functionality which must be implemented for new parsers
///
/// Often, practical parsers can be implemented by composing parsers provided in this crate with combinator methods on [PushParser]. Otherwise, this trait can be implemented directly.
pub trait ParserCore<B>: Sized
where
    B: ?Sized,
{
    /// The type of parsed value
    type Output;
    /// The custom error type
    type Error;

    /// Feed some input to this parser to produce an update (or error)
    fn feed(self, buffer: &B) -> ParseResult<Update<Self, Self::Output>, Self::Error>;

    /// Indicate to the parser the end of input
    ///
    /// Some parsers produce a value only upon end-of-input, for example the parser that matches any number of `'x'` chars in a string.
    fn finalize(self, buffer: &B) -> ParseResult<Option<Self::Output>, Self::Error>;
}
