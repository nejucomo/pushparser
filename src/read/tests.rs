use test_case::test_case;

use crate::{Literal, ParseResult, ReadParser};

const STRING_WITH_EMOJI: &str = "Hello 🌏! Nice to meet you.";

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(16)]
#[test_case(1<<14)]
fn literal_bytes_with_utf8_emoji_read_parse_with_bufsize(
    bufsize: usize,
) -> ParseResult<(), std::io::Error> {
    use crate::ParseResultExt;
    use std::convert::Infallible;

    let bytes = STRING_WITH_EMOJI.as_bytes();
    let parser = Literal::from(bytes);

    let outbytes = parser
        .read_parse_with_bufsize::<_, Infallible>(bytes, bufsize)
        .map_err_custom(|ei| ei.right().unwrap())?;
    assert_eq!(outbytes, bytes);

    Ok(())
}
