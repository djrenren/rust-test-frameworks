// Enable the feature
#![feature(custom_test_frameworks)]

// Set the test runner
#![test_runner(extern::test2::test_main_static)]

// Import the macro
use test2::test2;

// Enjoy
#[test2]
#[allow_fail]
fn my_test() {
    assert!(false);
}
