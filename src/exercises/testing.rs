#![allow(unused)]

pub fn start() {}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_eat(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Howdy, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1; got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100; got {}.",
                value
            );
        }
        Guess { value }
    }
}

// use "cargo test" to run all tests in project
#[cfg(test)]
mod tests {
    use super::*;
    // crate for test-only use
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // // tests fail when something in the test function panics
    // #[test]
    // fn another() {
    //     panic!("Make sure this test fails!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 9,
            height: 6,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_eat_smaller_parallel() {
        let larger = Rectangle {
            width: 5,
            height: 3,
        };
        let smaller = Rectangle {
            width: 4,
            height: 1,
        };
        assert!(larger.can_eat(&smaller));
    }
    #[test]
    fn larger_can_eat_smaller_perpendicular() {
        let larger = Rectangle {
            width: 23,
            height: 42,
        };
        let smaller = Rectangle {
            width: 39,
            height: 22,
        };
        assert!(larger.can_eat(&smaller));
    }

    #[test]
    fn smaller_cannot_eat_larger() {
        let larger = Rectangle {
            width: 13,
            height: 2,
        };
        let smaller = Rectangle {
            width: 7,
            height: 3,
        };
        assert!(!smaller.can_eat(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn adds_two_does_something() {
        assert_ne!(3, add_two(3));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // // Custom error message
    // #[test]
    // fn greeting_does_not_contain_name() {
    //     let result = greeting("Carol");
    //     assert!(!result.contains("Carol"), "Don't worry, this assertion is supposed to fail. Greeting did not contain name; value was '{}'", result);
    // }

    // Test that should panic
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_less_than_1() {
        Guess::new(0);
    }

    // Using Result<T,E> in tests
    #[test]
    fn two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four :("))
        }
    }

    // Don't run test by default
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes a long time to run
        let ten_secs = std::time::Duration::from_millis(10000);
        let now = std::time::Instant::now();

        std::thread::sleep(ten_secs);

        assert!(now.elapsed() >= ten_secs);
    }
}
