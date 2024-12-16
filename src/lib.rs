mod core;
mod error;
mod literal;
mod read;
mod result;
mod update;

pub use crate::core::ParserCore;
pub use crate::error::ParseError;
pub use crate::literal::{CmpPrefix, Literal};
pub use crate::read::ReadParser;
pub use crate::result::{ParseResult, ParseResultExt, ParseUpdateResultExt};
pub use crate::update::Update;

#[cfg(test)]
mod tests;
