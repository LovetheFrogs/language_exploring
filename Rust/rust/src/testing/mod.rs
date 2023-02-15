#[cfg(test)]
mod tests {
    use super::*;
    
    /* Tests in Rust are run with the command cargo test and need to be anotated with the #{test} tag. */
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /** This test will fail. */
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    /** This test should return True, as the larger Rectangle can hold the smaller one. */
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

    /** This test should return False, as the smaller Rectangle can't hold the larger one. */
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    /** This test should pass if function add_two works as intended */
    #[test]
    fn it_adds_two() {
        /* In case of assertion fault, the assert_eq! macro tells us with a message the assertion that failed. This assertion has the form -> assertion failed: (left == right)
         * where left and right are the values passed to the macro. */
        assert_eq!(4, add_two(2));
    }

    /** Assertions can include a custom message to print when they fail. */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeeting did not contain name, value was `{}`",
            result
        );
    }

    /** We can also check if the program will panic! under certain conditions with the #[should_panic] attribute. We can also use the spected parameter to the should_panic attribute
     *  to make it more precise. Note that it should be a substring of the panic message (or the whole String).
     */
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    /** Result<T, E> can also be used in our tests so they don't panic! and return an Err instead. */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /** If a test is passed and something is printed within the test function, nothing will print out to standard output. However, if the test fails, it will print AND show the
     *  error message. Function prints_and_returns_10() should print something and is tested below in this_test_will_pass function. If you want to see the values printed by a
     *  test function, you should call Cargo with the command cargo test -- --show-output instead of cargo test.
     */
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    /** The following three test have been coded to show the commands that can be used to filter out the tests you want to run. Let's have a look at two of these commands:
     *      cargo test              ->  Will run the three of them.
     *      cargo test one_hundred  ->  Will run the test one_hundred()
     *      cargo test add          ->  Will run the first two tests as they are the ones that contain the substring add.
     */
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    /** Sometimes you won't want to execute a test. To avoid this, you can use the #[ignore] attribute below the #[test] one. If we wanted to run JUST the ignored tests, 
     *  we would call the command `cargo test -- --ignored`. If we wanted to execute ALL the tests, we will call the command `cargo test -- --include-ignored`.
     */
    fn expensive_test() {
        // code that takes an hour to run
    }

    /** Thanks to unit tests being in the same file as the functions they test, private functions and method can be tested just by including the use super::* to bring all 
     *  those functions into scope
     */
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

/* Struct Rectangle imported from structs/rectangles/mod.rs file */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/** Function add_two should add two to a value passed as a parameter together */
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}