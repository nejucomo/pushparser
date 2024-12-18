use crate::parser::{IntoUtf8Parser, ParserCore};

/// A [TextParser] natively parses strings and can be adapted to parse UTF-8 bytes
pub trait TextParser: ParserCore<str> {
    /// Convert this `str` parser to one that consumed `[u8]` UTF-8 input
    fn into_utf8_parser(self) -> IntoUtf8Parser<Self> {
        IntoUtf8Parser::from(self)
    }
}

impl<T> TextParser for T where T: ParserCore<str> {}
