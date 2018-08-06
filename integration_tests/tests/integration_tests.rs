#![feature(custom_test_frameworks)]
#![test_runner(crate::runner)]

#[cfg(test)]
fn runner(ts: &[&i32]) {
    for t in ts {
        if **t == 0 {
            println!("PASSED!");
        } else {
            println!("Failed!");
        }
    }
}

#[test_case]
const TEST_1: i32 = 1;

#[test_case]
const TEST_2: i32 = 0;