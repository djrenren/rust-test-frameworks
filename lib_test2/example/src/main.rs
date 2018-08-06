#![feature(custom_test_frameworks)]
#![test_runner(extern::test2::test_main_static)]

use test2::test2;

#[test2]
#[allow_fail]
fn my_test() {
    assert!(false);
}
