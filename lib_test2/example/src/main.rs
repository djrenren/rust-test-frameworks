#![feature(test)]
use test2::test2;

#[test2]
#[allow_fail]
fn my_test() {
    assert!(false);
}

fn main() {
    println!("Hello, world!");
}
