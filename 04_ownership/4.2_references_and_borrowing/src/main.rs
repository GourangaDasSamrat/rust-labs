fn main() {
    println!("\n=== REFERENCES AND BORROWING LEARNING EXAMPLES ===\n");

    example_1_basic_references();
    example_2_immutable_references();
    example_3_mutable_references();
    example_4_mutable_reference_restrictions();
    example_5_multiple_immutable_references();
    example_6_mixing_immutable_and_mutable_references();
    example_7_reference_scope_rules();
    example_8_dangling_reference_prevention();
    example_9_references_with_scope_control();
}

fn example_1_basic_references() {
    println!("Example 1: Basic References (Borrowing)");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    println!("s1 still exists and has ownership: '{s1}'\n");

    // Learning notes:
    // When we use &s1, we create a reference that refers to the value of s1 but does not own it.
    // The ampersand (&) represents a reference. This lets us use the value without taking ownership.
    // The String s1 is NOT moved into the function, so we can still use it after the call.
    // The function parameter takes &String instead of String.
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String - it does not have ownership
    s.len()
    // When s goes out of scope here, nothing is dropped because s doesn't own the String.
    // The String is only dropped when the original owner s1 goes out of scope.
}

fn example_2_immutable_references() {
    println!("Example 2: Immutable References");

    let message = String::from("Borrowing is like lending");

    // We can create multiple immutable references to the same value
    let ref1 = &message;
    let ref2 = &message;
    let ref3 = &message;

    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
    println!("ref3: {}", ref3);

    println!("Original message still available: {}\n", message);

    // Learning notes:
    // Multiple immutable references are allowed to the same value because no one can modify the data.
    // All readers are safe since no one can change the value out from under them.
    // These references are borrowed - they don't own the data, they just reference it.
}

fn example_3_mutable_references() {
    println!("Example 3: Mutable References");

    let mut s = String::from("hello");

    // To create a mutable reference, we use &mut
    // The variable must also be declared as mut
    change_string(&mut s);

    println!("After modification: {}\n", s);

    // Learning notes:
    // Mutable references allow us to modify the borrowed value.
    // Both the variable (mut s) and the reference (&mut s) must be mutable.
    // This makes it very clear that the function will modify the borrowed value.
}

fn change_string(some_string: &mut String) {
    // This function borrows s as mutable, allowing modification
    some_string.push_str(", world");
}

fn example_4_mutable_reference_restrictions() {
    println!("Example 4: Mutable Reference Restrictions");

    let mut s = String::from("hello");

    // We can have only ONE mutable reference at a time
    let r1 = &mut s;
    r1.push_str(" first");
    println!("r1: {}", r1);

    // After r1 goes out of scope or is no longer used, we can create another
    let r2 = &mut s;
    r2.push_str(" second");
    println!("r2: {}", r2);

    println!("Final string: {}\n", s);

    // Learning notes:
    // This restriction prevents data races at compile time.
    // A data race occurs when:
    //   1. Two or more pointers access the same data at the same time
    //   2. At least one pointer is writing to the data
    //   3. There's no synchronization mechanism
    // Rust prevents data races by enforcing this rule!
    // Multiple mutable references to the SAME data simultaneously are NOT allowed.
}

fn example_5_multiple_immutable_references() {
    println!("Example 5: Multiple Immutable References Allowed");

    let s = String::from("shared data");

    // We can create as many immutable references as we want
    let reader1 = &s;
    let reader2 = &s;
    let reader3 = &s;
    let reader4 = &s;

    println!("reader1: {}", reader1);
    println!("reader2: {}", reader2);
    println!("reader3: {}", reader3);
    println!("reader4: {}", reader4);

    // All readers coexist peacefully
    println!("Original: {}\n", s);

    // Learning notes:
    // Multiple immutable references are safe because no one can modify the data.
    // All references are read-only, so there's no way for one reader to interfere with another.
    // This allows for safe concurrent reading without any synchronization overhead.
}

fn example_6_mixing_immutable_and_mutable_references() {
    println!("Example 6: Cannot Mix Immutable and Mutable References");

    let mut s = String::from("hello");

    let r1 = &s; // immutable reference
    let r2 = &s; // another immutable reference
    println!("Immutable refs - r1: {}, r2: {}", r1, r2);

    // At this point, r1 and r2 are done being used
    // Now we can create a mutable reference
    let r3 = &mut s; // mutable reference
    r3.push_str(", world");
    println!("Mutable ref - r3: {}\n", r3);

    // Learning notes:
    // We cannot have a mutable reference while immutable references exist and are being used.
    // Users of immutable references expect the value to NOT change while they hold the reference.
    // Once immutable references are no longer used (go out of scope), we can create mutable refs.
    // The key is the SCOPE of the reference - from declaration until last use.
}

fn example_7_reference_scope_rules() {
    println!("Example 7: Reference Scope Rules");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2's scope ends here - after their last use in println!

    // We can now create a mutable reference because r1 and r2 are out of scope
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}\n", r3);

    // Learning notes:
    // A reference's scope starts from its declaration and ends at its LAST USE, not at the end of the block.
    // This is called "Non-Lexical Lifetimes" (NLL) in Rust.
    // The compiler is smart enough to see that r1 and r2 are not used after the println!,
    // so their scope ends there, allowing r3 to be created before the block ends.
}

fn example_8_dangling_reference_prevention() {
    println!("Example 8: Dangling Reference Prevention");

    // The following would NOT compile:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ERROR: s goes out of scope, reference would dangle!
    // }

    // Instead, we return the owned value or use valid references:
    let s = no_dangle();
    println!("Valid return (owned value): {}\n", s);

    // Learning notes:
    // Rust prevents dangling references at compile time.
    // A dangling reference is a pointer to invalid memory (memory that was freed).
    // If a function tries to return a reference to a local variable, the compiler catches it.
    // The solution is to return the owned value directly, not a reference to it.
    // The compiler guarantees: if you have a reference, the data is still valid!
}

fn no_dangle() -> String {
    let s = String::from("hello");
    // Return the owned value, not a reference
    // Ownership is moved to the caller
    s
}

fn example_9_references_with_scope_control() {
    println!("Example 9: Using Scopes to Allow Multiple Mutable References");

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" first");
        println!("First scope: {}", r1);
    } // r1 goes out of scope here

    // Now we can create another mutable reference because r1 is gone
    {
        let r2 = &mut s;
        r2.push_str(" second");
        println!("Second scope: {}", r2);
    } // r2 goes out of scope here

    println!("Final: {}\n", s);

    // Learning notes:
    // We can use curly brackets to create new scopes for variables.
    // This allows us to have multiple mutable references, just not simultaneously.
    // Each reference is scoped to its block, so when the block ends, the reference ends.
    // This is useful when you need to perform sequential mutations on the same data.
}

// Summary of References and Borrowing Rules:
// 1. At any given time, you can have EITHER one mutable reference OR any number of immutable references.
// 2. References must always be valid (Rust prevents dangling references).
// 3. A reference's scope starts at declaration and ends at its last use (not the end of the block).
// 4. The ampersand (&) creates an immutable reference (borrowing).
// 5. The &mut syntax creates a mutable reference.
// 6. Both the variable and the reference must be marked as mutable for mutable references.
// 7. Borrowing allows you to use a value without taking ownership of it.
// 8. The borrowed value is not dropped when the reference goes out of scope (only the original owner drops it).
