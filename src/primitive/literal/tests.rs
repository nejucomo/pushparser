use std::{borrow::Cow, convert::Infallible};

use test_case::test_case;

use crate::buffer::SplitBuffer;
use crate::error::ParseError::UnexpectedInput;
use crate::error::ParseResult;
use crate::parser::ParserCore;
use crate::parser::Update::{self, Parsed, Pending};
use crate::primitive::Literal;

#[test_case(
    "Hello",
    "Hello World!"
    => matches Ok(Parsed("Hello", Cow::Borrowed(" World!")))
    ; "str_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Goodbye World!"
    => matches Err(UnexpectedInput)
    ; "str_goodbye_world_prefix_hello"
)]
#[test_case(
    b"Hello".as_slice(),
    b"Hello World!".as_slice()
    => matches Ok(Parsed(_, _))
    ; "bytes_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Hell"
    => matches Ok(Pending(_))
    ; "str_hell_prefix_hello"
)]
fn parse_literal<'a, B>(
    literal: &'a B,
    input: &'a B,
) -> ParseResult<Update<Literal<'a, B>, &'a B, Cow<'a, B>>, Infallible>
where
    B: ?Sized + SplitBuffer,
{
    Literal::from(literal).feed(input)
}
