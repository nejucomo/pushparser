use crate::{ParseError, Update};

pub type ParseResult<T, E> = Result<T, ParseError<E>>;

pub trait ParseResultExt<T, E> {
    fn map_err_custom<F, E2>(self, f: F) -> ParseResult<T, E2>
    where
        F: FnOnce(E) -> E2;
}

impl<T, E> ParseResultExt<T, E> for ParseResult<T, E> {
    fn map_err_custom<F, E2>(self, f: F) -> ParseResult<T, E2>
    where
        F: FnOnce(E) -> E2,
    {
        self.map_err(|e| e.map_custom(f))
    }
}

pub trait ParseUpdateResultExt<S, X, B, E>: ParseResultExt<Update<S, X, B>, E> {
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

impl<S, X, B, E> ParseUpdateResultExt<S, X, B, E> for ParseResult<Update<S, X, B>, E> {
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
