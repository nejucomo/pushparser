use crate::error::{ParseResult, ParseResultExt};
use crate::parser::{Outcome, Update};

/// A trait with a [`ParseResult`]`<`[`Update`]`, ...>` impl which delegates to useful [Update] methods
pub trait ParseResultUpdateExt<S, X, E>: ParseResultExt<Update<S, X>, E> {
    /// Map the pending state of an [Update]
    fn map_next<F, S2>(self, f: F) -> ParseResult<Update<S2, X>, E>
    where
        F: FnOnce(S) -> S2;

    /// Map the output of an [Update]
    fn map_output<F, X2>(self, f: F) -> ParseResult<Update<S, X2>, E>
    where
        F: FnOnce(X) -> X2;

    /// Map the outcome of an [Update]
    fn map_outcome<F, S2, X2>(self, f: F) -> ParseResult<Update<S2, X2>, E>
    where
        F: FnOnce(Outcome<S, X>) -> Outcome<S2, X2>;
}

impl<S, X, E> ParseResultUpdateExt<S, X, E> for ParseResult<Update<S, X>, E> {
    fn map_next<F, S2>(self, f: F) -> ParseResult<Update<S2, X>, E>
    where
        F: FnOnce(S) -> S2,
    {
        self.map(|up| up.map_next(f))
    }

    fn map_output<F, X2>(self, f: F) -> ParseResult<Update<S, X2>, E>
    where
        F: FnOnce(X) -> X2,
    {
        self.map(|up| up.map_output(f))
    }

    /// Map the outcome of an [Update]
    fn map_outcome<F, S2, X2>(self, f: F) -> ParseResult<Update<S2, X2>, E>
    where
        F: FnOnce(Outcome<S, X>) -> Outcome<S2, X2>,
    {
        self.map(|up| up.map_outcome(f))
    }
}
