/// The result of incremental parsing (when an error is not encountered)
#[derive(Debug)]
pub enum Update<S, X, B> {
    /// All input was consumed and the parser updated its state to `S`; a full output has not yet been parsed
    Pending(S),

    /// The parser successfully parsed an `X` item and returns any unconsumed `B` buffer
    Parsed(X, B),
}
use Update::*;

impl<S, X, B> Update<S, X, B> {
    /// Map the pending state
    pub fn map_pending<F, S2>(self, f: F) -> Update<S2, X, B>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            Pending(s) => Pending(f(s)),
            Parsed(x, b) => Parsed(x, b),
        }
    }

    /// Map the output
    pub fn map_output<F, X2>(self, f: F) -> Update<S, X2, B>
    where
        F: FnOnce(X) -> X2,
    {
        match self {
            Pending(s) => Pending(s),
            Parsed(x, b) => Parsed(f(x), b),
        }
    }

    /// Map the returned buffer
    pub fn map_buffer<F, B2>(self, f: F) -> Update<S, X, B2>
    where
        F: FnOnce(B) -> B2,
    {
        match self {
            Pending(s) => Pending(s),
            Parsed(x, b) => Parsed(x, f(b)),
        }
    }
}

impl<S, X, E, B> Update<S, Result<X, E>, B> {
    /// Convert an [Update] with a [Result] output to a [Result] containing [Update]
    ///
    /// This may be useful after [Update::map_output] if mapped to a [Result].
    pub fn transpose_output(self) -> Result<Update<S, X, B>, E> {
        match self {
            Pending(s) => Ok(Pending(s)),
            Parsed(Ok(x), b) => Ok(Parsed(x, b)),
            Parsed(Err(e), _) => Err(e),
        }
    }
}
