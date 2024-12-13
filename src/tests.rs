mod xstar;

use crate::{Parser, ReadParseError};

use self::xstar::XStarParser;

#[test]
fn xstar_success() -> Result<(), ReadParseError> {
    let parser = XStarParser::default();
    let input = b"xxxxx";
    let xcount = parser.read_parse(input.as_slice())?;
    assert_eq!(xcount, input.len());
    Ok(())
}
