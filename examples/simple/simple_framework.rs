pub fn test_main_static(tests: &[&Fn(i32)->(i32)]) {
    for t in tests {
        println!("{}", t(3));
    }
}