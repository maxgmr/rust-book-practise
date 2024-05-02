#![allow(unused)]

pub fn start() {}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// use "cargo test" to run all tests in project
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // tests fail when something in the test function panics
    #[test]
    fn another() {
        panic!("Make sure this test fails!");
    }
}
