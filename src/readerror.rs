#[derive(Debug, thiserror::Error)]
pub enum ReadParseError {
    #[error("unexpected input")]
    UnexpectedInput,

    #[error("expected more input")]
    ExpectedMoreInput,

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
