use crate::{IntoUtf8Parser, ParserCore};

pub trait Utf8Parser: ParserCore<str> {
    fn into_utf8_parser(self) -> IntoUtf8Parser<Self> {
        IntoUtf8Parser::from(self)
    }
}

impl<T> Utf8Parser for T where T: ParserCore<str> {}
