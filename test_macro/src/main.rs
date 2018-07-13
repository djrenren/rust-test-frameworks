#![feature(test)]
use libtest_macro::fancy_test;

#[fancy_test]
#[allow_fail]
fn ex1() {
    println!("foo")
}

fn main() {
    println!("Hello, world! {:?}", ex1);
}
