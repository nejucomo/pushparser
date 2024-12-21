use std::convert::Infallible;

use test_case::test_case;

use crate::buffer::BufRef;
use crate::error::{ParseResult, ParseResultUpdateExt};
use crate::parser::Outcome::{Next, Parsed};
use crate::parser::{PushParser, Update};
use crate::primitive::Literal;
use crate::sequence::SequenceParser;

#[test_case(
    "Hello",
    "Hello World!",
    Ok(Update { consumed: 5, outcome: Parsed(vec![5]) })
    ; "str_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "HelloHelloHello World!",
    Ok(Update { consumed: 15, outcome: Parsed(vec![5, 5, 5]) })
    ; "str_hellohellohello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Goodbye World!",
    Ok(Update { consumed: 0, outcome: Parsed(vec![]) })
    ; "str_goodbye_world_prefix_hello"
)]
#[test_case(
    b"Hello".as_slice(),
    b"Hello World!".as_slice(),
    Ok(Update { consumed: 5, outcome: Parsed(vec![5]) })
    ; "bytes_hello_world_prefix_hello"
)]
#[test_case(
    "Hello",
    "Hell",
    Ok(Update { consumed: 0, outcome: Next(()) })
    ; "str_hell_prefix_hello"
)]
fn parse_literal_repeated_collect<'a, B>(
    literal: &'a B,
    input: &'a B,
    expected: ParseResult<Update<(), Vec<usize>>, Infallible>,
) where
    B: ?Sized + BufRef + std::fmt::Debug,
{
    let actual = Literal::from(literal)
        .map_output(BufRef::len)
        .repeated()
        .collect()
        .feed(input)
        .map_next(|_| ());

    assert_eq!(actual, expected);
}
