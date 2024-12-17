use crate::parser::{IntoUtf8Parser, ParserCore};

pub trait TextParser: ParserCore<str> {
    fn into_utf8_parser(self) -> IntoUtf8Parser<Self> {
        IntoUtf8Parser::from(self)
    }
}

impl<T> TextParser for T where T: ParserCore<str> {}
