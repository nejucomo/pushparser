mod error;
mod parsercore;
mod read;
mod result;
mod update;

pub use crate::error::ParseError;
pub use crate::parsercore::ParserCore;
pub use crate::read::ReadParser;
pub use crate::result::{ParseResult, ParseResultExt, ParseUpdateResultExt};
pub use crate::update::Update;

#[cfg(test)]
mod tests;
