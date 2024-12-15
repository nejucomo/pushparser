#[derive(Debug, thiserror::Error)]
pub enum ParseError<E> {
    #[error("unexpected input")]
    UnexpectedInput,

    #[error("expected more input")]
    ExpectedMoreInput,

    #[error(transparent)]
    Custom(#[from] E),
}

impl<E> ParseError<E> {
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
