use adder;

mod common;

#[test]
fn it_adds_to_six() {
    assert_eq!(6, adder::add_two(4))
}