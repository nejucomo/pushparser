use crate::error::ParseResult;

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
