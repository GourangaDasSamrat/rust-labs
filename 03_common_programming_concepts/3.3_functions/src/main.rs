fn main() {
    // Example 1: Basic function definition and calling
    another_function();

    // Example 2: Function with single parameter
    function_with_parameter(5);

    // Example 3: Function with multiple parameters
    print_labeled_measurement(5, 'h');

    // Example 4: Statements (do not return values)
    let y = 6; // This is a statement
    let x = {
        let y = 3;
        y + 1 // Expression without semicolon - returns 4
    };

    // Example 5: Function that returns a value (implicit return)
    let result1 = five(); // Returns 5

    // Example 6: Function with parameter and return value
    let result2 = plus_one(5); // Returns 6

    // Example 7: Function with explicit return
    let result3 = add_two(3); // Returns 5

    // Example 8: Function that uses early return
    let result4 = check_number(10);

    // Example 9: Function returning tuple
    let (a, b) = get_pair();

    // Example 10: Nested function calls
    let nested_result = plus_one(five()); // five() returns 5, plus_one(5) returns 6
}

// Example 1: Basic function definition
fn another_function() {
    let message = "Another function.";
}

// Example 2: Function with single parameter
fn function_with_parameter(x: i32) {
    let y = x * 2;
}

// Example 3: Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    // Function body with multiple parameters
}

// Example 5: Function with return value (expression without semicolon)
fn five() -> i32 {
    5 // No semicolon - this is an expression, returns 5
}

// Example 6: Function with parameter and return value
fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon - expression returns the value
}

// Example 7: Function with explicit return keyword
fn add_two(x: i32) -> i32 {
    return x + 2; // Early return with explicit return keyword
}

// Example 8: Function with conditional return
fn check_number(num: i32) -> i32 {
    if num > 5 {
        return num * 2;
    }
    num
}

// Example 9: Function returning tuple
fn get_pair() -> (i32, i32) {
    (10, 20) // Expression returns a tuple
}

// Example 10: Function with multiple statements and expression
fn complex_calculation(x: i32, y: i32) -> i32 {
    let sum = x + y; // Statement
    let product = x * y; // Statement
    sum + product // Expression - final line without semicolon returns value
}

// Example 11: Function showing statement vs expression difference
fn statement_vs_expression() {
    let y = {
        let x = 3;
        x + 1 // Expression block evaluates to 4
    };
    // y is now 4
}

// Example 12: Function with no return value (returns unit type)
fn returns_nothing() {
    let x = 5;
    // No return value - implicitly returns ()
}
