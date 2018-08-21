#![feature(custom_test_frameworks)]
#![test_runner(crate::runner)]

use std::env;
#[cfg(test)] use std::io::{self, Write};
use std::process::{Command, exit};

// A test that "fails" if it returns false
type SillyTest = Fn() -> bool;

// Looks at argv to see if we're the master runner or the subordinate runner
// If no args are supplied we're the master
// Otherwise the parameter provided is the index of the test to run
pub fn runner(tests: &[&SillyTest]) {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        master_main(tests);
    } else {
        sub_main(
            tests,
            args[1].parse().unwrap());
    }
}

// Runs each test in a separate process
fn master_main(tests: &[&SillyTest]) {
    let curr_exe = env::current_exe().expect("Couldn't get current exe... sorry?");

    for i in 0..tests.len() {
        let output = Command::new(&curr_exe)
                        .arg(&i.to_string())
                        .output().expect("failed to execute child process");
        if output.status.success() {
            print!(".");
        } else {
            print!("f");
            println!("\n{}", ::std::str::from_utf8(&output.stdout).unwrap());
            println!("\n{}", ::std::str::from_utf8(&output.stderr).unwrap());
        }
    }
}

// Runs a specific test
fn sub_main(tests: &[&SillyTest], idx: usize) {
    if !tests[idx]() {
        exit(1);
    }
    exit(0);
}

#[test_case]
fn foo() -> bool {
    io::stdout().lock().write(b"Should be swallowed!\n").unwrap();
    true
}

#[test_case]
fn foo() -> bool {
    io::stdout().lock().write(b"Is properly reported\n").unwrap();
    false
}