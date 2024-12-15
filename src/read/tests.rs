use test_case::test_case;

#[ignore]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(16)]
#[test_case(1<<14)]
fn read_parse_with_bufsize(bufsize: usize) {
    todo!("implement a test with bufsize {bufsize}");
}
