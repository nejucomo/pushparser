//! Define and use I/O agnostic "push parsers" via the [PushParser] trait
//!
//! A [PushParser] can be incrementally fed input to either produce a parsed value (or error), or to provided an updated parser state. This interface is I/O agnostic and can be used in synchronous I/O APIs, asynchronous I/O APIs, in other contexts where partial parsing is useful (such as interactive user interfaces), and where parsing very large inputs. All of these distinct use cases can use the same parser definition, which improves code reuse and interoperability across applications.
//!
//! A fundamental trade-off is that parsed values may not refer to input data, which is useful for creating "zero-copy" parsers. A zero-copy parser can produce a parse result that refers to the in-memory input to avoid copying input data.
//!
//! # Consumers
//!
//! ## Parsing complete I/O sources
//!
//! Code which needs a parsed result typically calls [ByteParser::parse_reader] or `ByteParser::parse_async_reader` (not yet implemented). A parser over `&str` input provides [TextParser::into_utf8_parser] to convert to a [ByteParser].
//!
//! ## Incremental parsing
//!
//! Some applications need direct access to incremental parsing, such as in a user interface which is attempting to parse user input as it is written, or those with data sources which don't fit neatly into the [ByteParser] methods. Also, some applications may use different kinds of input such as sequence of application specific tokens and parsers over those. All of these cases need to use [ParserCore] directly:
//!
//! The fundamental incremental parsing functionality comes from two methods, the first of which is [ParserCore::feed]. [ParserCore::feed] produces a [Result] which either signifies a parse error or an [Update]. Updates indicate how many input elements were consumed and whether a [ParserCore::Output] was parsed or the parser has a newly updated state.
//!
//! The second funamental parsing method is and [ParserCore::finalize] which signals to the parser no more input is coming. It must either produce a value, or signal a parse error (typically indicating that it expected more input).
//!
//! ### Memory Management
//!
//! Consumer code which is using [ParserCore] directly must maintain certain invariants on the input buffer:
//!
//! 1. Any items which aren't consumed by a call to [ParserCore::feed] _must remain present starting at index 0_ of the next call to any [ParserCore] method.
//! 2. Each call to [ParserCore::feed] must have new input items with respect to the previous call. This implies if a buffer is full and [ParserCore::feed] consumes 0 items, the buffer must be extended to a larger size.
//! 3. When there is no more input, [ParserCore::finalize] must be called a single time on the remaining buffer (which may be empty).
//!
//! The [BufferManager] type simplifies management of this buffer.
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
use crate::{
    buffer::BufferManager,
    parser::{ByteParser, ParserCore, PushParser, TextParser, Update},
};

#[cfg(test)]
mod tests;
