//! Define and use I/O agnostic "push parsers" via the [PushParser] trait
//!
//! A [PushParser] can be incrementally fed input via [ParserCore::feed] to either produce a parsed value (or error), or to provided an updated parser state (see [Update]). This interface is I/O agnostic and can be used in synchronous I/O APIs, asynchronous I/O APIs, or other contexts where partial parsing is useful (such as interactive user interfaces).
//!
//! The fundamental functionality is implemented with the [ParserCore] trait. Higher level parsers often use the [PushParser] trait to compose simpler parsers. Consumers typically use [ReadParser] or `AsyncReadParser` (not yet implemented) to parse input. Because `&[u8]` implements [std::io::Read], [ReadParser] can also be used to parse in-memory bytes and strings.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![feature(extend_one)]

pub mod buffer;
pub mod combinator;
pub mod error;
pub mod parser;
pub mod primitive;
pub mod sequence;

#[cfg(doc)]
use crate::parser::{ParserCore, PushParser, ReadParser, Update};

#[cfg(test)]
mod tests;
