use adder;  // each file in the *tests* directory is a 
            // separate crate, so need to bring in

mod common; // add common module to use it from any of the 
            // integration test files as a module.

// don't need to add cfg() here, as Cargo treats the tests 
// directory specially and compiles files in this directory
// only when we run 'cargo test'.
//
#[test]
fn it_adds_to_six() {
    common::setup();    // declared to improve test output
    assert_eq!(6, adder::add_two(4))
}

