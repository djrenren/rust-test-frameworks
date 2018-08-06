#![feature(custom_test_frameworks)]
#![test_runner(crate::my_runner)]

#[cfg(test)]
fn my_runner(ts: &[& dyn Fn(i32) -> String]) {
    println!("Custom Test Framework running {} tests: ", ts.len());
    for t in ts {
        println!("{}", t(0));
    }
}

#[test_case]
fn bar(a: i32) -> String {
    format!("Hello: {}", a)
}
