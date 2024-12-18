//! [ParseError] and related utilities
mod perror;
mod resultext;
mod resupext;

pub use self::perror::ParseError;
pub use self::resultext::ParseResultExt;
pub use self::resupext::ParseResultUpdateExt;

/// A type alias to shorten the [ParseError::Custom] type
pub type ParseResult<T, E> = Result<T, ParseError<E>>;
