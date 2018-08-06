# rust-test-frameworks
A home for experiments with custom test frameworks in rust

All examples require a build from the `custom-test-frameworks` branch from:
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
#![test_runner(crate::my_runner)]

fn my_runner(ts: &[&Fn(i32) -> bool]) {
  //...
}

#[test_case]
fn foo(a: i32) -> bool {

}
```

`test_case` performs basic aggregation and the `test_runner` crate attribute specifies
what function will receive the tests.

