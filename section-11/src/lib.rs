/*
TESTING
Tests are Rust function that verify that non-test functions are working as intended.
Body of these test functions tipically includes three actions:
- Set up needed data,
- Run the code that needs to be tested,
- Assert the results are what you expect.
*/

/*
`cargo test` accepts two types of arguments, (1) is for `test` command and (2) is for resulting test binary.
These two can be seperated by `--` sign.
`cargo test --help` displays options you can use with `cargo test`,
`cargo test -- --help` displays the options you can use after the seperation.
*/

/*
When multiple tests are run, by default they run in parallel using threads. Because of this, test cases must have isolated states meaning they don't depend eachother.

Thread count can be changed to override this behavior by `cargo test -- --test-threads=1`.
This way total time will be longer but we can overcome the problems such as overriding test data.
*/

/*
By default when test cases are run only message on assertion will be display if provided on passed tests. Anything that prints to console before will not print.
On failures, anything can be printed to screen.
If we wanted to see prints of passed tests, we could call test with `cargo test -- --show-output`.
*/

/*
If we want to only run one specific test, we can give the name of it after command `cargo test [some_test_fn_name]`
Any test function did not get run will be displayed on result section as `(num of tests that didn't run) filtered out`
*/

/*
We can also run multiple tests with a given string. Let's say we have three tests, two test function's names that includes `add`. If we run `cargo test add` it will run these two cases and filter out other one.
*/

/*
If by any reason we want to skip a test case, we can add `ignore` attribute to skip this test case on normal case run.
If we only want to run tests that was ignored normall we can do it by `cargo test -- --ignored`. This way cargo will only run tests that are marked as ignored.
*/

/*
By convention, each unit test are in the same file as they are testing. For that reason it is necessary to put `#[cfg(test)]` to start of testing module. That attribute tells rust to only compile and run that part of the code when developer runs `cargo build`.
`cfg` means `configuration`
*/

/*
Integration tests are entirely external to your library. They use your code just like any external code would use it. For that reason integration test file are located in another folder.It is called `tests` directory and it is sibling with `src` directory.
Unline in unit tests, we don't need to add `cfg` attribute to start of modules since rust will only compile and run anything under `tests` folder when `cargo test` is called.
Each test file under `tests` folder is a seperate crate.
For code examples please see: https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests
*/

use core::panic;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value == 32 {
            panic!("Guess value cannot be 32")
        }

        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // We can have non test functions inside of testing module,
    // a function that may help us to set up data etc.

    // Values that are being compared with `assert_eq!` or `assert_ne!` must implement `PartialEq` and `Debug` traits.

    // This is called an attribute.
    // By adding this we change a function to test function.
    #[test]
    fn exploration() {
        // this compares given two values to be equal or not
        assert_eq!(add_two(2), 4)
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 50,
            height: 50,
        };

        let smaller = Rectangle {
            width: 25,
            height: 30,
        };

        // This tests if given value is evaluated to true
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 50,
            height: 50,
        };

        let smaller = Rectangle {
            width: 25,
            height: 30,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // We can add custom failure message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    // this means that accepted result of this test is if it panics
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // we can also test if it panics with specific panic message
    #[should_panic(expected = "cannot be 32")]
    fn cannot_be_32() {
        Guess::new(32);
    }

    // This is also a valid case of testing values with `Result<T,E>` type
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four!"))
        }
    }
}
