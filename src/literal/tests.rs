use std::{borrow::Cow, convert::Infallible};

use test_case::test_case;

use crate::{
    buffer::SplitBuffer,
    Literal,
    ParseError::UnexpectedInput,
    ParseResult, ParserCore,
    Update::{self, Parsed, Pending},
};

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
