use crate::error::{ParseError, ParseResult};

/// The base parser type defines an output value, custom error, and end-of-input behavior
pub trait ParserBase: Sized {
    /// The type of parsed value
    type Output;
    /// The custom error type
    type Error;

    /// Signal to the parser the end of input
    ///
    /// Some parsers can return a pending value; whereas for others this is [ParseError]
    fn end_of_input(self) -> ParseResult<Self::Output, Self::Error> {
        self.pending_at_end().ok_or(ParseError::ExpectedMoreInput)
    }

    /// Convert the parser to a pending output if possible at the end of input.
    ///
    /// Some parsers produce a value only upon end-of-input, for example the parser that matches any number of `'x'` chars in a string. If this returns `None` this always signals the parser was expecting more input for a complete result.
    /// This method is typically for implementors, and consumer code typically calls [ParserBase::end_of_input].
    fn pending_at_end(self) -> Option<Self::Output>;
}
