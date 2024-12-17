mod perror;
mod resultext;
mod resupext;

pub use self::perror::ParseError;
pub use self::resultext::ParseResultExt;
pub use self::resupext::ParseResultUpdateExt;

pub type ParseResult<T, E> = Result<T, ParseError<E>>;
