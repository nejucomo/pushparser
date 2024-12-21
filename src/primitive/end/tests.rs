use test_case::test_case;

use crate::parser::PushParser;
use crate::primitive::end;

#[test_case("" => true)]
#[test_case("foo" => false)]
#[test_case(" " => false)]
#[test_case("\n" => false)]
#[test_case("\0" => false)]
fn parse(input: &str) -> bool {
    end().feed(input).is_ok()
}
