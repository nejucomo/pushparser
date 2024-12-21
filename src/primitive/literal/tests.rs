use std::convert::Infallible;

use test_case::test_case;

use crate::buffer::BufRef;
use crate::error::ParseError::UnexpectedInput;
use crate::error::ParseResult;
use crate::parser::Outcome::{Next, Parsed};
use crate::parser::{ParserCore, Update};
use crate::primitive::{literal, Literal};

#[test_case(
    "Hello",
    "Hello World!"
    => matches Ok(Update { consumed: 5, outcome: Parsed("Hello") })
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
    => matches Ok(Update { consumed: 5, outcome: Parsed(_) })
    ; "bytes_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Hell"
    => matches Ok(Update { consumed: 4, outcome: Next(_) })
    ; "str_hell_prefix_hello"
)]
fn parse_literal<'a, B>(
    litval: &'a B,
    input: &'a B,
) -> ParseResult<Update<Literal<'a, B>, &'a B>, Infallible>
where
    B: ?Sized + BufRef,
{
    literal(litval).feed(input)
}
