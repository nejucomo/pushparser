use std::convert::Infallible;

use test_case::test_case;

use crate::buffer::BufRef;
use crate::combinator::Optional;
use crate::error::ParseResult;
use crate::parser::Outcome::{Next, Parsed};
use crate::parser::{ParserCore, PushParser, Update};
use crate::primitive::Literal;

#[test_case(
    "Hello",
    "Hello World!"
    => matches Ok(Update { consumed: 5, outcome: Parsed(Some("Hello")) })
    ; "str_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Goodbye World!"
    => matches Ok(Update { consumed: 0, outcome: Parsed(None) })
    ; "str_goodbye_world_prefix_hello"
)]
#[test_case(
    b"Hello".as_slice(),
    b"Hello World!".as_slice()
    => matches Ok(Update { consumed: 5, outcome: Parsed(Some(_)) })
    ; "bytes_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Hell"
    => matches Ok(Update { consumed: 0, outcome: Next(_) })
    ; "str_hell_prefix_hello"
)]
fn parse_literal<'a, B>(
    literal: &'a B,
    input: &'a B,
) -> ParseResult<Update<Optional<Literal<'a, B>>, Option<&'a B>>, Infallible>
where
    B: ?Sized + BufRef,
{
    Literal::from(literal).optional().feed(input)
}
