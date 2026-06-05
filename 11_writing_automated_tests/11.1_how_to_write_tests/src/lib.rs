/// Demonstrates a simple const function for addition.
/// Const functions can be evaluated at compile time.
#[allow(clippy::pedantic, clippy::arithmetic_side_effects)]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Example struct for testing struct methods and instance comparison.
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    /// Tests if this rectangle can hold another rectangle.
    /// Returns true only if both dimensions are strictly greater.
    const fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Demonstrates string formatting in functions.
/// Used to test result assertions with custom messages.
#[allow(clippy::pedantic)]
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

/// Demonstrates validation and panic testing.
/// The `new()` method validates input bounds and panics on invalid values.
#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(clippy::pedantic, clippy::nursery, clippy::panic)]
impl Guess {
    /// Creates a new Guess with bounds validation (1-100).
    /// Panics if value is outside the valid range.
    pub fn new(value: i32) -> Guess {
        const MIN_VALUE: i32 = 1;
        const MAX_VALUE: i32 = 100;

        if value < MIN_VALUE {
            panic!("Guess value must be greater than or equal to {MIN_VALUE}, got {value}.");
        }
        if value > MAX_VALUE {
            panic!("Guess value must be less than or equal to {MAX_VALUE}, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to avoid repeating Rectangle creation (DRY principle).
    /// Returns a test pair: (larger rectangle, smaller rectangle)
    fn create_test_rectangles() -> (Rectangle, Rectangle) {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        (larger, smaller)
    }

    /// Tests basic const function arithmetic.
    /// Demonstrates: #[test] attribute for unit tests.
    #[test]
    fn adder() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// Tests struct method with positive case.
    /// Demonstrates: assert! macro for boolean conditions.
    #[test]
    fn larger_can_hold_smaller() {
        let (larger, smaller) = create_test_rectangles();
        assert!(larger.can_hold(&smaller));
    }

    /// Tests struct method with negative case.
    /// Demonstrates: assert! with negation (!).
    #[test]
    fn smaller_cannot_hold_larger() {
        let (larger, smaller) = create_test_rectangles();
        assert!(!smaller.can_hold(&larger));
    }

    /// Tests string output with custom assertion message.
    /// Demonstrates: assert! with custom failure message, .`contains()` string method.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    /// Tests that panic occurs with expected message.
    /// Demonstrates: #[`should_panic`] attribute for panic testing.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
