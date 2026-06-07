fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    This test intentionally fails to demonstrate test output.
    By default, Rust test output is suppressed for passing tests.
    Use `cargo test -- --show-output` to capture and display println! output
    and other standard output from ALL tests (passing and failing).
    This is useful for debugging test logic or seeing function behavior.

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);  // This fails because prints_and_returns_10 always returns 10, not 5
    }
    */

    // To run only tests whose name contains "add_two", use: `cargo test add_two`
    // Test filtering helps when you want to focus on a specific test or subset
    // without waiting for the entire test suite to complete.
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // The #[ignore] attribute marks this test to be skipped during normal test runs.
    // This is useful for slow, expensive, or work-in-progress tests.
    // To run ONLY ignored tests, use: `cargo test -- --ignored`
    // To run ALL tests including ignored ones, use: `cargo test -- --include-ignored`
    #[test]
    #[ignore]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
