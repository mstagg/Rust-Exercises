use adder;

// rust knows to find integration tests in a seperate directory called 'tests'.

// Note integration tests wont run if unit tests are not running. Integration tests
// can also only be incldued in lib projects, not binaries!
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add(2, 2))
}
