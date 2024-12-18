//! Define and use I/O agnostic "push parsers"
//!
//! A push parser can be incrementally fed input to either produce a parsed value (or error), or to provided an updated parser state. This interface is I/O agnostic and can be used in synchronous I/O APIs, asynchronous I/O APIs, or other contexts where partial parsing is useful (such as interactive user interfaces).
//!
//! The fundamental functionality is implemented with the [ParserCore](crate::parser::ParserCore) trait. Higher level parsers often use the `Parser` (not yet implemented) crate to compose simpler parsers. Consumers typically use [ReadParser](crate::parser::ReadParser) or `AsyncReadParser` (not yet implemented) to parse input. Because `&[u8]` implements [std::io::Read], [ReadParser](crate::parser::ReadParser) can also be used to parse in-memory bytes and strings.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod buffer;
pub mod error;
pub mod parser;
pub mod primitive;

#[cfg(test)]
mod tests;
