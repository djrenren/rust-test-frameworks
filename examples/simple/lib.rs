extern crate simple_framework;

#[test = "simple_framework"]
fn foo(a: i32) -> i32 {
    a
}

#[test = "simple_framework"]
fn bar(a: i32) -> i32 {
    a
}

fn main() {
    println!("hey");
}
