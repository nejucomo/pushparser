use crate::error::{ParseResult, ParseResultExt};
use crate::parser::Update;

pub trait ParseResultUpdateExt<S, X, B, E>: ParseResultExt<Update<S, X, B>, E> {
    fn map_pending<F, S2>(self, f: F) -> ParseResult<Update<S2, X, B>, E>
    where
        F: FnOnce(S) -> S2;

    fn map_output<F, X2>(self, f: F) -> ParseResult<Update<S, X2, B>, E>
    where
        F: FnOnce(X) -> X2;

    fn map_buffer<F, B2>(self, f: F) -> ParseResult<Update<S, X, B2>, E>
    where
        F: FnOnce(B) -> B2;
}

impl<S, X, B, E> ParseResultUpdateExt<S, X, B, E> for ParseResult<Update<S, X, B>, E> {
    fn map_pending<F, S2>(self, f: F) -> ParseResult<Update<S2, X, B>, E>
    where
        F: FnOnce(S) -> S2,
    {
        self.map(|up| up.map_pending(f))
    }

    fn map_output<F, X2>(self, f: F) -> ParseResult<Update<S, X2, B>, E>
    where
        F: FnOnce(X) -> X2,
    {
        self.map(|up| up.map_output(f))
    }

    fn map_buffer<F, B2>(self, f: F) -> ParseResult<Update<S, X, B2>, E>
    where
        F: FnOnce(B) -> B2,
    {
        self.map(|up| up.map_buffer(f))
    }
}