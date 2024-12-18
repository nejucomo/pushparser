/// Parser errors include universal errors that any parser may emit, plus custom error types
#[derive(Debug, thiserror::Error)]
pub enum ParseError<E> {
    /// The parser encountered unexpected input
    #[error("unexpected input")]
    UnexpectedInput,

    /// The parser was expecting more input
    #[error("expected more input")]
    ExpectedMoreInput,

    /// The parser encountered a custom error
    #[error(transparent)]
    Custom(#[from] E),
}

impl<E> ParseError<E> {
    /// Translate a custom error
    ///
    /// This method is often useful when a higher-level parser needs to adapt an inner parser's error
    pub fn map_custom<F, E2>(self, f: F) -> ParseError<E2>
    where
        F: FnOnce(E) -> E2,
    {
        use ParseError::*;

        match self {
            UnexpectedInput => UnexpectedInput,
            ExpectedMoreInput => ExpectedMoreInput,
            Custom(e) => Custom(f(e)),
        }
    }
}
