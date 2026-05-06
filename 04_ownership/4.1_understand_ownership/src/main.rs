fn main() {
    println!("\n=== Chapter 4: Understanding Ownership ===\n");

    example_1_variable_scope();
    example_2_string_literal_vs_string_type();
    example_3_memory_and_allocation();
    example_4_variables_and_data_interacting_with_move();
    example_5_scope_and_assignment();
    example_6_variables_and_data_interacting_with_clone();
    example_7_stack_only_data_copy();
    example_8_ownership_and_functions();
    example_9_return_values_and_scope();
    example_10_returning_multiple_values_with_tuple();
}

// EXAMPLE 1: VARIABLE SCOPE
// ==============================================================================
// Learn: A scope is the range within a program for which an item is valid.
// The variable s refers to a string literal, where the value of the string is
// hardcoded into the text of the program. The variable is valid from the point
// at which it's declared until the end of the current scope.
fn example_1_variable_scope() {
    println!("1. VARIABLE SCOPE");
    println!("-----------------");

    // Outer scope - s_outer is valid here
    let s_outer = "I'm in outer scope";
    println!("Outer: {}", s_outer);

    {
        // Inner scope - s_inner is only valid within these braces
        let s_inner = "I'm in inner scope";
        println!("Inner: {}", s_inner);
    } // s_inner goes out of scope here and is no longer valid

    // println!("{}", s_inner); // This would cause a compile error!
    println!("Back in outer: {}", s_outer);
    println!();
}

// EXAMPLE 2: STRING LITERAL vs STRING TYPE
// ==============================================================================
// Learn: String literals are immutable and their value is hardcoded into the
// program. The String type is mutable and allocated on the heap, allowing for
// unknown sizes at compile time.
fn example_2_string_literal_vs_string_type() {
    println!("2. STRING LITERAL vs STRING TYPE");
    println!("--------------------------------");

    // String literal - immutable, fixed size, stored in binary
    let s_literal = "hello";
    println!("String literal: {}", s_literal);
    // s_literal.push_str(" world"); // ERROR: can't mutate string literal

    // String type - mutable, allocated on heap, can grow
    let mut s_string = String::from("hello");
    println!("String (before): {}", s_string);

    // We can mutate the String because it's allocated on the heap
    s_string.push_str(", world!");
    println!("String (after): {}", s_string);
    println!();
}

// EXAMPLE 3: MEMORY AND ALLOCATION
// ==============================================================================
// Learn: When a variable goes out of scope, Rust calls a special function
// called `drop`, which returns the memory to the allocator. This happens
// automatically at the closing curly bracket.
fn example_3_memory_and_allocation() {
    println!("3. MEMORY AND ALLOCATION");
    println!("------------------------");

    {
        // s_temp is allocated on the heap here
        let s_temp = String::from("hello");
        println!("s_temp created: {}", s_temp);
    } // s_temp goes out of scope here, drop() is called automatically,
      // and the memory on the heap is freed

    // We can't access s_temp here because it's been dropped
    println!("s_temp has been dropped and memory freed");
    println!();
}

// EXAMPLE 4: VARIABLES AND DATA INTERACTING WITH MOVE
// ==============================================================================
// Learn: When we assign a String to another variable, the ownership moves.
// The first variable is no longer valid, preventing a double free error.
// This is different from integers which implement the Copy trait.
fn example_4_variables_and_data_interacting_with_move() {
    println!("4. VARIABLES AND DATA INTERACTING WITH MOVE");
    println!("-------------------------------------------");

    // Example A: Integer assignment (Copy - both remain valid)
    println!("A. Integer assignment (Copy trait):");
    let x = 5;
    let y = x; // x is copied (not moved) because i32 implements Copy
    println!("x = {}, y = {}", x, y); // Both are valid!

    // Example B: String assignment (Move - first variable becomes invalid)
    println!("\nB. String assignment (Move):");
    let s1 = String::from("hello");
    println!("s1 created: {}", s1);

    let s2 = s1; // s1's value is moved to s2, s1 is no longer valid
    println!("s2 received ownership: {}", s2);
    // println!("{}", s1); // ERROR: s1 has been moved, can't use it

    println!("Note: s1 can no longer be used after being moved to s2");
    println!();
}

// EXAMPLE 5: SCOPE AND ASSIGNMENT
// ==============================================================================
// Learn: When you assign a completely new value to an existing variable,
// Rust will call drop and free the original value's memory immediately.
fn example_5_scope_and_assignment() {
    println!("5. SCOPE AND ASSIGNMENT");
    println!("----------------------");

    let mut s = String::from("hello");
    println!("Initial value: {}", s);

    // Assign a new value to s
    // The old value "hello" is dropped and its memory is freed
    s = String::from("ahoy");
    println!("After reassignment: {}", s);

    // The old "hello" value was automatically freed when we reassigned
    println!();
}

// EXAMPLE 6: VARIABLES AND DATA INTERACTING WITH CLONE
// ==============================================================================
// Learn: If we want to deeply copy the heap data of a String (not just the
// stack data), we can use the clone() method. This is more expensive than
// a move, and the visual indicator (clone) shows something different is
// happening.
fn example_6_variables_and_data_interacting_with_clone() {
    println!("6. VARIABLES AND DATA INTERACTING WITH CLONE");
    println!("--------------------------------------------");

    let s1 = String::from("hello");
    println!("s1 created: {}", s1);

    // Using clone() creates a deep copy of the heap data
    let s2 = s1.clone();
    println!("s2 cloned: {}", s2);

    // Both s1 and s2 are now valid!
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("Both s1 and s2 remain valid after clone()");
    println!();
}

// EXAMPLE 7: STACK-ONLY DATA - COPY
// ==============================================================================
// Learn: Types that implement the Copy trait (like integers) are stored
// entirely on the stack. When assigned to another variable, they are
// trivially copied, and the original remains valid.
fn example_7_stack_only_data_copy() {
    println!("7. STACK-ONLY DATA - COPY");
    println!("-------------------------");

    // All these types implement Copy:
    let x: i32 = 5; // integer
    let y: bool = true; // boolean
    let z: f64 = 3.14; // floating-point
    let c: char = 'A'; // character
    let t: (i32, i32) = (1, 2); // tuple of Copy types

    println!("Types that implement Copy:");
    println!("x (i32) = {}", x);
    println!("y (bool) = {}", y);
    println!("z (f64) = {}", z);
    println!("c (char) = {}", c);
    println!("t (i32, i32) = {:?}", t);

    // Assigning Copy types
    let x2 = x; // x is copied, not moved
    println!("\nAfter assignment:");
    println!("x = {}, x2 = {}", x, x2); // Both are valid!

    println!();
}

// EXAMPLE 8: OWNERSHIP AND FUNCTIONS
// ==============================================================================
// Learn: Passing a variable to a function will move or copy it, just as
// assignment does. String ownership is transferred to the function, while
// Copy types (like i32) remain valid in the caller.
fn example_8_ownership_and_functions() {
    println!("8. OWNERSHIP AND FUNCTIONS");
    println!("---------------------------");

    let s = String::from("hello");
    println!("Before function call: s = {}", s);

    takes_ownership(s); // s's value moves into the function
    // println!("{}", s); // ERROR: s has been moved

    println!("After function call: s has been moved (no longer valid here)");

    let x = 5;
    println!("\nBefore function call: x = {}", x);

    makes_copy(x); // Because i32 implements Copy, x is NOT moved
    println!("After function call: x = {} (still valid!)", x);

    println!();
}

fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: {}", some_string);
} // some_string goes out of scope, drop is called, memory is freed

fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
} // some_integer goes out of scope, nothing special happens

// EXAMPLE 9: RETURN VALUES AND SCOPE
// ==============================================================================
// Learn: Returning values can also transfer ownership. A function can take
// ownership of a value and return it to the caller.
fn example_9_return_values_and_scope() {
    println!("9. RETURN VALUES AND SCOPE");
    println!("--------------------------");

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 received from gives_ownership: {}", s1);

    let s2 = String::from("hello");
    println!("s2 created: {}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved in, return value moves into s3
    // println!("{}", s2); // ERROR: s2 has been moved
    println!("s3 received from takes_and_gives_back: {}", s3);

    println!();
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // Returns and moves ownership to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Returns and moves ownership to the caller
}

// EXAMPLE 10: RETURNING MULTIPLE VALUES WITH TUPLE
// ==============================================================================
// Learn: If we want to let a function use a value but not take ownership,
// we can return multiple values using a tuple. This is tedious but works.
fn example_10_returning_multiple_values_with_tuple() {
    println!("10. RETURNING MULTIPLE VALUES WITH TUPLE");
    println!("----------------------------------------");

    let s1 = String::from("hello");
    println!("Original string: {}", s1);

    let (s2, len) = calculate_length(s1);
    println!("String: {}, Length: {}", s2, len);

    // s2 has ownership of the string now
    println!("We got the string back: {}", s2);

    println!();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // Return both the String and its length
}
