// ============================================================================
// RUST SLICE TYPE - COMPREHENSIVE LEARNING GUIDE
// ============================================================================
//
// KEY CONCEPT: Slices let you reference a contiguous sequence of elements
// in a collection. A slice is a KIND OF REFERENCE, so it does NOT have ownership.
//
// WHY SLICES? They allow the compiler to ensure that references into a
// collection remain valid, preventing a class of logic errors at compile time.
//
// ============================================================================

// ============================================================================
// PART 1: THE PROBLEM - Why We Need Slices
// ============================================================================

// Problem: Write a function that takes a string of words separated by spaces
// and returns the first word. If no space is found, return the whole string.

// NAIVE APPROACH (WITHOUT SLICES) - Returns index as usize
// This approach has a critical flaw: the index can become invalid
fn first_word_naive(s: &String) -> usize {
    // Convert String to array of bytes
    // Why? Strings are UTF-8 encoded, and we need to search for a space (b' ')
    let bytes = s.as_bytes();

    // iter() returns each element in the collection
    // enumerate() wraps each element as part of a tuple (index, element)
    // This gives us both the index and the element value
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' is byte literal syntax for the space character
        if item == b' ' {
            return i; // Return index of the space
        }
    }

    // No space found, return the length (entire string is one word)
    s.len()
}

// THE BUG WITH NAIVE APPROACH
// The problem with returning just an index:
// - We return a usize value that is separate from the String
// - There's NO CONNECTION between the index and the String data
// - If the String changes, the index becomes meaningless/invalid
// - The compiler won't catch this bug!

fn demonstrate_naive_approach_problem() {
    // NOTE: This logic is incorrect and would be a bug in real code
    let mut s = String::from("hello world");

    // Get index of first word (5, which is the space position)
    let word_index = first_word_naive(&s); // word_index = 5

    // But then we clear the string...
    // s.clear(); // this empties the String, making it equal to ""

    // Now word_index still has the value 5, but:
    // 1. s no longer has any content
    // 2. Index 5 no longer means anything
    // 3. word_index is now TOTALLY INVALID
    // 4. The compiler didn't warn us about this! (This is the bug)

    // If we tried to use word_index with the cleared string,
    // we'd get incorrect or undefined behavior

    println!("Word index (naive approach): {}", word_index);
}

// ============================================================================
// PART 2: SOLUTION - STRING SLICES
// ============================================================================

// A string slice is a REFERENCE to a contiguous sequence of elements in a String.
// Internally, a slice stores:
// 1. A reference to the starting position
// 2. The length of the slice

// STRING SLICE SYNTAX: &s[starting_index..ending_index]
// - starting_index: first position in the slice (INCLUSIVE)
// - ending_index: one MORE than the last position (EXCLUSIVE)
// - Results in a type: &str (string slice)

fn demonstrate_string_slices() {
    // Create a String
    let s = String::from("hello world");

    // Create slices referencing portions of the String
    let hello = &s[0..5]; // "hello" - from index 0 to 4 (5 is exclusive)
    let world = &s[6..11]; // "world" - from index 6 to 10 (11 is exclusive)

    println!("Full string: {}", s); // "hello world"
    println!("Slice 1: {}", hello); // "hello"
    println!("Slice 2: {}", world); // "world"

    // Memory visualization:
    // String s is stored in memory with heap pointer, length, and capacity
    // Slice hello is stored with a pointer to index 0 and length 5
    // Slice world is stored with a pointer to index 6 and length 5
    //
    // Both slices reference the SAME underlying data - no copy is made
    // This is what makes slices memory efficient
}

// ============================================================================
// PART 3: RANGE SYNTAX SHORTCUTS
// ============================================================================

fn demonstrate_range_syntax() {
    let s = String::from("hello");

    // STARTING AT INDEX 0:
    // These two are equivalent:
    let slice1 = &s[0..2]; // Explicit notation
    let slice2 = &s[..2]; // Shorthand - can drop starting 0
    println!("Index 0 shorthand: {} == {}", slice1, slice2);

    // ENDING AT LAST INDEX:
    // These two are equivalent:
    let len = s.len(); // 5
    let slice3 = &s[3..len]; // Explicit notation
    let slice4 = &s[3..]; // Shorthand - can drop ending length
    println!("Last index shorthand: {} == {}", slice3, slice4);

    // ENTIRE STRING:
    // These three are all equivalent:
    let slice5 = &s[0..len];
    let slice6 = &s[..]; // Drop both start and end
    let slice7 = &s[0..s.len()];
    println!("Entire string: {} == {}", slice5, slice6);
}

// ============================================================================
// PART 4: THE IMPROVED SOLUTION - Using String Slices
// ============================================================================

// IMPROVED APPROACH: Returns &str (string slice)
// This ties the returned slice to the underlying String data
// The compiler ensures the reference remains valid
fn first_word(s: &str) -> &str {
    // Convert String to array of bytes to find space character
    let bytes = s.as_bytes();

    // Iterate through bytes to find first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return a slice from start (0) to the space position (i)
            return &s[0..i];
        }
    }

    // No space found, return slice of entire string
    &s[..]
}

// WHY THIS IS BETTER:
// 1. The returned &str is a REFERENCE to the actual data
// 2. It stores both pointer and length information
// 3. The compiler knows about the connection between slice and original String
// 4. Any attempt to invalidate this reference is caught at COMPILE TIME

fn demonstrate_slice_safety() {
    let mut s = String::from("hello world");

    // Get a slice of the first word
    let word = first_word(&s);
    println!("First word (slice): {}", word); // "hello"

    // THIS WILL NOT COMPILE:
    // s.clear(); // ERROR! Cannot borrow s as mutable
    //
    // Compiler error explanation:
    // - word is an immutable borrow of s
    // - clear() requires a mutable borrow of s
    // - We cannot have both immutable and mutable borrows at the same time
    // - This compile-time error prevents the logic bug!
    //
    // Without this check, if we allowed both:
    // - word would point to data that no longer exists
    // - This would be memory unsafety and undefined behavior
    //
    // The compiler forces us to either:
    // 1. Use word before calling clear(), or
    // 2. Not use word after calling clear()

    // This is valid:
    println!("Using word here is safe: {}", word);
    // Now word is no longer used, so we can call clear
    s.clear();
}

// ============================================================================
// PART 5: SECOND WORD FUNCTION EXAMPLE
// ============================================================================

// We can also write functions that return multiple indices
// Using slices makes this much cleaner and safer
fn second_word(s: &str) -> &str {
    // Skip the first word and space
    let bytes = s.as_bytes();
    let mut word_start = 0;
    let mut found_first_word = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first_word {
            // Found end of first word, skip the space
            word_start = i + 1;
            found_first_word = true;
        } else if item == b' ' && found_first_word {
            // Found end of second word
            return &s[word_start..i];
        }
    }

    // Return from word_start to end of string
    if found_first_word {
        &s[word_start..]
    } else {
        "" // No second word
    }
}

// ============================================================================
// PART 6: STRING LITERALS AS SLICES
// ============================================================================

fn demonstrate_string_literals_are_slices() {
    // When you write a string literal, it's NOT a String type
    // It's actually a &str - a slice into the binary
    let s = "Hello, world!";

    // The type of s is &str (string slice)
    // The slice points to a specific location in the compiled binary
    // This is why string literals are immutable - they're slices (&str)

    println!("String literal type: &str");
    println!("String literal value: {}", s);
}

// ============================================================================
// PART 7: FLEXIBLE FUNCTION SIGNATURES WITH STRING SLICES
// ============================================================================

// LESS FLEXIBLE: Takes only &String
fn first_word_string_ref(s: &String) -> &str {
    // This works for String references, but NOT for string literals
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// MORE FLEXIBLE: Takes &str (works with both String and string literals)
fn first_word_slice(s: &str) -> &str {
    // This is the idiomatic Rust way
    // It works with:
    // 1. Slices of Strings: &my_string[0..5]
    // 2. Whole String references: &my_string
    // 3. String literal slices: &"hello world"[0..5]
    // 4. String literals directly: "hello world"
    //
    // This is possible due to "deref coercions" - automatic conversions
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn demonstrate_flexible_function_signature() {
    let my_string = String::from("hello world");

    // first_word_slice works on slices of Strings (partial)
    let word = first_word_slice(&my_string[0..6]);
    println!("Partial slice: {}", word);

    // first_word_slice works on slices of Strings (whole)
    let word = first_word_slice(&my_string[..]);
    println!("Whole slice via ..: {}", word);

    // first_word_slice works on references to String
    // (automatically coerced to &str via deref coercion)
    let word = first_word_slice(&my_string);
    println!("String reference: {}", word);

    let my_string_literal = "hello world";

    // first_word_slice works on slices of string literals (partial)
    let word = first_word_slice(&my_string_literal[0..6]);
    println!("Literal partial slice: {}", word);

    // first_word_slice works on slices of string literals (whole)
    let word = first_word_slice(&my_string_literal[..]);
    println!("Literal whole slice: {}", word);

    // first_word_slice works on string literals directly
    // (no & needed because string literals are already &str)
    let word = first_word_slice(my_string_literal);
    println!("Literal directly: {}", word);

    // NOTE: This flexibility takes advantage of "deref coercions"
    // The compiler automatically converts &String to &str when needed
}

// ============================================================================
// PART 8: OTHER SLICES (NOT JUST STRINGS)
// ============================================================================

fn demonstrate_array_slices() {
    // Slices aren't limited to strings - they work with any collection

    // Regular array
    let a = [1, 2, 3, 4, 5];

    // Slice of the array - just like string slices
    let slice = &a[1..3]; // Elements at indices 1 and 2 (3 is exclusive)

    // The type is &[i32] (slice of i32)
    println!("Array slice: {:?}", slice); // [2, 3]

    // Verify it equals the expected values
    assert_eq!(slice, &[2, 3]);

    // Range syntax works the same way as with strings
    let first_two = &a[0..2]; // [1, 2]
    let from_index_2 = &a[2..]; // [3, 4, 5]
    let first_three = &a[..3]; // [1, 2, 3]
    let entire_array = &a[..]; // [1, 2, 3, 4, 5]

    println!("First two: {:?}", first_two);
    println!("From index 2: {:?}", from_index_2);
    println!("First three: {:?}", first_three);
    println!("Entire array: {:?}", entire_array);
}

// ============================================================================
// PART 9: KEY MEMORY CONCEPTS
// ============================================================================

fn demonstrate_slice_memory_layout() {
    // A slice is stored as TWO pieces of data:
    // 1. A pointer to the first element
    // 2. The length of the slice

    let s = String::from("hello world");

    // When we create a slice:
    let hello = &s[0..5];

    // Memory layout:
    // stack (for 'hello' slice):
    //   - pointer: points to s's data at index 0 (the 'h')
    //   - length: 5 (covers indices 0-4: "hello")
    //
    // heap (s's data):
    //   - "hello world" stored in memory

    // This is memory-safe because:
    // 1. The slice only references valid data
    // 2. The compiler checks borrowing rules
    // 3. The data won't be deallocated while the slice exists

    println!("Slice hello: {}", hello);
    println!("Slice length: {}", hello.len());
}

// ============================================================================
// PART 10: IMPORTANT NOTES ABOUT UTF-8
// ============================================================================

fn demonstrate_utf8_boundary_rules() {
    // IMPORTANT: String slice range indices must occur at valid UTF-8
    // character boundaries.
    //
    // If you try to create a slice in the middle of a multibyte character,
    // your program will PANIC (crash at runtime).
    //
    // Example (would panic if actually run):
    // let s = "你好"; // Chinese characters (each takes multiple bytes in UTF-8)
    // let slice = &s[0..1]; // PANIC! 1 is not a valid UTF-8 boundary

    // This is a safeguard to prevent silent data corruption
    // Rust forces you to be aware of UTF-8 encoding

    let s = "hello"; // ASCII - each character is 1 byte
    let slice = &s[0..3]; // Safe - all ASCII characters

    println!("UTF-8 safe slice: {}", slice);
}

// ============================================================================
// MAIN FUNCTION - Running All Examples
// ============================================================================

fn main() {
    println!("=== RUST SLICE TYPE LEARNING EXAMPLES ===\n");

    println!("1. Demonstrating the problem with naive approach:");
    demonstrate_naive_approach_problem();

    println!("\n2. String slices:");
    demonstrate_string_slices();

    println!("\n3. Range syntax shortcuts:");
    demonstrate_range_syntax();

    println!("\n4. Slice safety (compiler prevents bugs):");
    demonstrate_slice_safety();

    println!("\n5. String literals as slices:");
    demonstrate_string_literals_are_slices();

    println!("\n6. Flexible function signatures:");
    demonstrate_flexible_function_signature();

    println!("\n7. Array slices:");
    demonstrate_array_slices();

    println!("\n8. Slice memory layout:");
    demonstrate_slice_memory_layout();

    println!("\n9. UTF-8 boundary rules:");
    demonstrate_utf8_boundary_rules();

    println!("\n=== END OF LEARNING EXAMPLES ===");
}

// ============================================================================
// SUMMARY OF KEY CONCEPTS
// ============================================================================
//
// 1. WHAT IS A SLICE?
//    - A reference to a contiguous sequence of elements in a collection
//    - Does NOT have ownership
//    - Stores a pointer and length internally
//
// 2. TYPES OF SLICES:
//    - &str: String slice (references part or all of a String or literal)
//    - &[T]: Array/collection slice (generic, works with any collection)
//
// 3. SYNTAX:
//    - &collection[start..end] where end is EXCLUSIVE
//    - Range shortcuts: [..], [start..], [..end], [..]
//
// 4. WHY SLICES ARE IMPORTANT:
//    - Compiler ensures references remain valid
//    - Prevents logic bugs at compile time
//    - More memory efficient than copying data
//    - Enables flexible, idiomatic function signatures
//
// 5. BORROWING RULES WITH SLICES:
//    - Immutable borrow: Can read the data
//    - Cannot modify original while immutable slice exists
//    - Compiler enforces these rules at compile time
//
// 6. STRING LITERALS ARE SLICES:
//    - Type is &str, not String
//    - This is why they're immutable
//    - Point to data in the compiled binary
//
// 7. IDIOMATIC RUST:
//    - Use &str in function parameters, not &String
//    - This accepts both String references and string literals
//    - More flexible and cleaner API
//
// ============================================================================
