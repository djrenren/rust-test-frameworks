# rust-test-frameworks
A home for experiments with custom test frameworks in rust

All examples require master from:
https://github.com/djrenren/rust


# High Level Architecture
The goal is to allow user-defined crates to fulfill tests.

Today we mark tests like so:

```rust
#[test]
fn foo() {
  //...
}
```

These tests are aggreggated and run by rust's internal libtest framework.
If we want custom reporting or different execution behavior we're stuck.
Now we can write something like so:

```rust
#[test="myframework"]
fn foo(a: i32) -> bool {

}
```

`myframework` is a crate that contains a public `fn test_main_static` at the top level.
`test_main_static` accepts a reference to an array of references to tests like so:

```rust
fn test_main_static(tests: &[&Fn(i32)->(i32)]) {
}
```
