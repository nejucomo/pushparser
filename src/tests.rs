use std::convert::Infallible;

use test_case::test_case;

use crate::error::{ParseResult, ParseResultExt};
use crate::parser::{ByteParser, TextParser};
use crate::primitive::Literal;

const STRING_WITH_EMOJI: &str = "Hello 🌏! Nice to meet you.";

/// Exercises `[Literal]<'_, [u8]>` and [ByteParser::parse_reader_with_bufsize]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(16)]
#[test_case(1<<14)]
fn literal_bytes_with_utf8_emoji_parse_reader_with_bufsize(
    bufsize: usize,
) -> ParseResult<(), std::io::Error> {
    let bytes = STRING_WITH_EMOJI.as_bytes();
    let parser = Literal::from(bytes);

    let outbytes = parser
        .parse_reader_with_bufsize::<_, Infallible>(bytes, bufsize)
        .map_err_custom(|ei| ei.right().unwrap())?;
    assert_eq!(outbytes, bytes);

    Ok(())
}

/// Exercises `[Literal]<'_, str>`, [Utf8Parser::into_utf8_parser], and [ByteParser::parse_reader_with_bufsize]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(16)]
#[test_case(1<<14)]
fn literal_str_with_utf8_emoji_into_utf8_parser_parse_reader_with_bufsize(
    bufsize: usize,
) -> ParseResult<(), std::io::Error> {
    let parser = Literal::from(STRING_WITH_EMOJI);

    let inbytes = STRING_WITH_EMOJI.as_bytes();
    let outstr = parser
        .into_utf8_parser()
        .parse_reader_with_bufsize::<_, Infallible>(inbytes, bufsize)
        .map_err_custom(|ei| ei.right().unwrap())?;

    assert_eq!(outstr.as_bytes(), inbytes);
    assert_eq!(outstr, STRING_WITH_EMOJI);

    Ok(())
}
