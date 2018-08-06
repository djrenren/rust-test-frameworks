#![feature(custom_test_frameworks)]
#![test_runner(crate::runner)]

#[cfg(test)]
fn runner(ts: &[&Fn() -> i32]) {
    for t in ts {
        if t() == 0 {
            println!("PASSED!");
        } else {
            println!("Failed!");
        }
    }
}

#[test_case]
fn test1() -> i32 {
    1
}

#[test_case]
fn test2() -> i32 {
    0
}
