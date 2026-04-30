// RUST DATA TYPES - COMPREHENSIVE LEARNING EXAMPLES

// Every value in Rust is of a certain data type. Rust is statically typed,
// meaning the compiler must know all variable types at compile time.

fn main() {
    println!("\n-- TYPE ANNOTATION EXAMPLE --");
    type_annotation_example();

    println!("\n-- SCALAR TYPES: INTEGERS --");
    integer_types_example();
    integer_literals_example();

    println!("\n-- INTEGER OVERFLOW HANDLING --");
    integer_overflow_example();

    println!("\n-- SCALAR TYPES: FLOATING-POINT --");
    floating_point_example();

    println!("\n-- NUMERIC OPERATIONS --");
    numeric_operations_example();

    println!("\n-- SCALAR TYPES: BOOLEAN --");
    boolean_type_example();

    println!("\n-- SCALAR TYPES: CHARACTER --");
    character_type_example();

    println!("\n-- COMPOUND TYPES: TUPLES --");
    tuple_example();
    tuple_destructuring_example();
    tuple_indexing_example();
    unit_type_example();

    println!("\n-- COMPOUND TYPES: ARRAYS --");
    array_example();
    array_type_annotation_example();
    array_initialization_shorthand_example();
    array_element_access_example();
    array_months_example();

    println!("\n-- ARRAY BOUNDS CHECKING --");
    safe_array_access_example();
}

// TYPE ANNOTATION EXPLANATION

fn type_annotation_example() {
    // WITHOUT type annotation, Rust can't infer what numeric type we want
    // This would cause a compile error: error[E0284]: type annotations needed
    // let guess = "42".parse().expect("Not a number!");

    // WITH type annotation, the compiler knows exactly what we want
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed guess: {} (type: u32)", guess);

    // Another example with a different numeric type
    let decimal_number: f64 = "3.14".parse().expect("Not a valid number!");
    println!("Parsed decimal: {} (type: f64)", decimal_number);
}

// INTEGER TYPES - Understanding Signed vs Unsigned

fn integer_types_example() {
    // SIGNED INTEGERS (can be negative): i8, i16, i32, i64, i128, isize
    // Range: −(2^(n−1)) to 2^(n−1) − 1
    let signed_8bit: i8 = -128; // Smallest i8 value
    let signed_8bit_max: i8 = 127; // Largest i8 value
    println!("i8 range: {} to {}", signed_8bit, signed_8bit_max);

    let signed_32bit: i32 = -2147483648; // Smallest i32 value
    let signed_32bit_max: i32 = 2147483647; // Largest i32 value
    println!("i32 range: {} to {}", signed_32bit, signed_32bit_max);

    // UNSIGNED INTEGERS (always positive): u8, u16, u32, u64, u128, usize
    // Range: 0 to 2^n − 1
    let unsigned_8bit: u8 = 0; // Smallest u8 value
    let unsigned_8bit_max: u8 = 255; // Largest u8 value
    println!("u8 range: {} to {}", unsigned_8bit, unsigned_8bit_max);

    let unsigned_32bit: u32 = 4294967295; // Largest u32 value
    println!("u32 max: {}", unsigned_32bit);

    // ARCHITECTURE-DEPENDENT: isize and usize depend on your computer's architecture
    // 64 bits on 64-bit systems, 32 bits on 32-bit systems
    // Primarily used for indexing collections
    let collection_index: usize = 0;
    println!("Array index type (usize): {}", collection_index);

    // Note: This would cause an error because -10 doesn't fit in a u8
    // let invalid: u8 = -10;  // ❌ Error: unsigned integers can't be negative
}

// INTEGER LITERALS - Different ways to write numbers

fn integer_literals_example() {
    // DECIMAL - standard base 10 numbers
    let decimal = 98_222;
    println!("Decimal: {}", decimal); // Underscore is a visual separator for readability

    // HEXADECIMAL - base 16 (prefix: 0x)
    let hexadecimal = 0xff;
    println!("Hexadecimal: 0xff = {}", hexadecimal);

    // OCTAL - base 8 (prefix: 0o)
    let octal = 0o77;
    println!("Octal: 0o77 = {}", octal);

    // BINARY - base 2 (prefix: 0b)
    let binary = 0b1111_0000;
    println!("Binary: 0b1111_0000 = {}", binary);

    // BYTE - u8 only (prefix: b)
    let byte = b'A'; // ASCII value of 'A'
    println!("Byte: b'A' = {} (ASCII value)", byte);

    // TYPE SUFFIX - explicitly specify the type in the literal
    let typed_int = 57u8; // This is a u8 (5 bytes)
    println!("With type suffix: 57u8 = {}", typed_int);

    // VISUAL SEPARATORS - underscores make large numbers readable
    let one_million = 1_000_000;
    println!("One million with separators: {}", one_million);
}

// INTEGER OVERFLOW - What happens when you exceed the type's limits

fn integer_overflow_example() {
    // In DEBUG mode: Rust panics (exits with error)
    // In RELEASE mode (--release): Rust uses two's complement wrapping

    // Methods to explicitly handle overflow:

    // 1. wrapping_* methods - silently wraps around
    let wrapped = (255u8).wrapping_add(1);
    println!("wrapping_add: 255 + 1 = {} (wrapped)", wrapped);

    // 2. checked_* methods - returns Option (Some or None)
    let checked = (255u8).checked_add(1);
    match checked {
        Some(result) => println!("checked_add succeeded: {}", result),
        None => println!("checked_add returned None (overflow detected)"),
    }

    // 3. overflowing_* methods - returns value AND boolean flag
    let (result, overflowed) = (255u8).overflowing_add(1);
    println!(
        "overflowing_add: result={}, overflowed={}",
        result, overflowed
    );

    // 4. saturating_* methods - clamps to min/max value
    let saturated = (255u8).saturating_add(1); // Stays at 255
    println!("saturating_add: 255 + 1 = {} (stays at max)", saturated);
}

// FLOATING-POINT TYPES - Numbers with decimal points

fn floating_point_example() {
    // DEFAULT FLOATING-POINT TYPE is f64 (64-bit)
    // More precision than f32, roughly same speed on modern CPUs
    let x = 2.0; // f64 (type inferred from literal)
    println!("Default float: {} (inferred as f64)", x);

    // EXPLICIT f32 TYPE ANNOTATION (32-bit)
    let y: f32 = 3.0;
    println!("Explicit f32: {}", y);

    // f64 provides more precision
    let precise: f64 = 3.141592653589793;
    println!("f64 precision: {}", precise);

    // Both types follow IEEE-754 standard
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let not_a_number = f64::NAN;
    println!(
        "Special f64 values: inf={}, -inf={}, nan={}",
        infinity, neg_infinity, not_a_number
    );
}

// NUMERIC OPERATIONS - Basic math operations

fn numeric_operations_example() {
    // ADDITION
    let sum = 5 + 10;
    println!("Addition: 5 + 10 = {}", sum);

    // SUBTRACTION
    let difference = 95.5 - 4.3;
    println!("Subtraction: 95.5 - 4.3 = {}", difference);

    // MULTIPLICATION
    let product = 4 * 30;
    println!("Multiplication: 4 * 30 = {}", product);

    // DIVISION
    let quotient = 56.7 / 32.2;
    println!("Division (float): 56.7 / 32.2 = {}", quotient);

    // INTEGER DIVISION TRUNCATES toward zero
    let integer_division = 5 / 3;
    println!(
        "Integer division: 5 / 3 = {} (truncates toward zero)",
        integer_division
    );

    let negative_division = -5 / 3;
    println!(
        "Negative division: -5 / 3 = {} (truncates toward zero)",
        negative_division
    );

    // REMAINDER (modulo)
    let remainder = 43 % 5;
    println!("Remainder: 43 % 5 = {}", remainder);

    let remainder_negative = -5 % 3;
    println!("Negative remainder: -5 % 3 = {}", remainder_negative);
}

// BOOLEAN TYPE - true or false

fn boolean_type_example() {
    // IMPLICIT TYPE (inferred as bool)
    let t = true;
    println!("Inferred boolean: {}", t);

    // EXPLICIT TYPE ANNOTATION
    let f: bool = false;
    println!("Explicit boolean: {}", f);

    // Booleans are 1 byte in size
    println!("Size of bool: {} byte(s)", std::mem::size_of::<bool>());

    // Booleans are commonly used in conditionals
    let number = 3;
    let condition = number < 5; // Expression that evaluates to a bool
    println!("Is 3 < 5? {}", condition);
}

// CHARACTER TYPE - Single Unicode character

fn character_type_example() {
    // SIMPLE ASCII CHARACTER
    let c = 'z';
    println!("Simple char: '{}'", c);

    // EXPLICIT TYPE ANNOTATION
    let z: char = 'ℤ'; // Mathematical symbol
    println!("Unicode char (mathematical): '{}'", z);

    // EMOJI - valid Unicode character
    let heart_eyed_cat = '😻';
    println!("Emoji char: '{}'", heart_eyed_cat);

    // Important: char uses SINGLE QUOTES, not double quotes
    // Double quotes are for strings, single quotes are for chars
    let single_quote_char: char = 'a';
    // let double_quote_str = "a";  // This would be a &str (string), not a char

    // UNICODE RANGE
    // Valid: U+0000 to U+D7FF and U+E000 to U+10FFFF
    let accented_letter = 'é'; // Accented letter
    println!("Accented letter: '{}'", accented_letter);

    let chinese_char = '中'; // Chinese character
    println!("Chinese character: '{}'", chinese_char);

    let greek_char = 'Ω'; // Greek letter
    println!("Greek letter: '{}'", greek_char);

    // Size of char type (always 4 bytes for Unicode)
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}

// TUPLE TYPE - Grouping multiple values of different types

fn tuple_example() {
    // Tuples have FIXED LENGTH - once created, they can't grow or shrink
    // Each position can have a different type

    // IMPLICIT TYPE INFERENCE
    let tup = (500, 6.4, 1);
    println!("Tuple without annotation: {:?}", tup);

    // EXPLICIT TYPE ANNOTATION
    let tup_annotated: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple with annotation: {:?}", tup_annotated);

    // MIXED TYPES
    let mixed: (i32, f64, bool, char) = (42, 3.14, true, 'a');
    println!("Mixed types tuple: {:?}", mixed);
}

// TUPLE DESTRUCTURING - Breaking a tuple into individual variables

fn tuple_destructuring_example() {
    let tup = (500, 6.4, 1);
    println!("Original tuple: {:?}", tup);

    // DESTRUCTURE using pattern matching
    let (x, y, z) = tup;
    println!("After destructuring: x={}, y={}, z={}", x, y, z);

    // You can ignore values with underscore
    let (first, _, third) = tup;
    println!(
        "Using _ to ignore middle value: first={}, third={}",
        first, third
    );

    // With type annotations during destructuring
    let coordinates: (f64, f64, f64) = (3.5, 2.1, 8.9);
    let (x_coord, y_coord, z_coord) = coordinates;
    println!("\nCoordinates: x={}, y={}, z={}", x_coord, y_coord, z_coord);
}

// TUPLE INDEXING - Accessing individual elements by index

fn tuple_indexing_example() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", x);

    // ACCESS by index using dot notation
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("x.0={}, x.1={}, x.2={}", five_hundred, six_point_four, one);

    // Note: First index is 0 (like most programming languages)
    // Out-of-bounds access causes a compile error
}

// UNIT TYPE - Empty tuple

fn unit_type_example() {
    // UNIT TYPE is written as ()
    let unit: () = ();
    println!("Unit value: {:?}", unit);

    // Functions that don't return a value implicitly return the unit type
    let result = function_returns_nothing();
    println!("Function with no return statement returns: {:?}", result);
}

fn function_returns_nothing() -> () {
    println!("This function implicitly returns the unit type");
    // No explicit return - returns unit type ()
}

// ARRAY TYPE - Fixed-length collection of same-type elements

fn array_example() {
    // ARRAY with inferred type
    let a = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);

    // Arrays: fixed length, stack allocation, all same type
    // Unlike vectors, you cannot dynamically add or remove elements
}

// ARRAY TYPE ANNOTATIONS - Explicit type and length

fn array_type_annotation_example() {
    // SYNTAX: [type; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array with annotation: {:?}", a);

    let string_array: [&str; 3] = ["hello", "world", "rust"];
    println!("String array: {:?}", string_array);

    // Type annotation helps catch errors at compile time
}

// ARRAY INITIALIZATION - Shorthand for repeated values

fn array_initialization_shorthand_example() {
    // SHORTHAND: [value; length]
    let a = [3; 5];
    println!("Shorthand array [3; 5]: {:?}", a);

    let days_in_week = [1; 7];
    println!("Days in week: {:?}", days_in_week);

    // With explicit type annotation
    let zeros: [u8; 10] = [0; 10];
    println!("10 zeros: {:?}", zeros);
}

// ARRAY ELEMENT ACCESS - Getting individual elements by index

fn array_element_access_example() {
    let a = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);

    // ACCESS elements using index notation
    let first = a[0];
    let second = a[1];
    println!("a[0]={}, a[1]={}", first, second);

    // Iterating over all elements
    println!("\nAll elements:");
    for (index, value) in a.iter().enumerate() {
        println!("  a[{}] = {}", index, value);
    }
}

// ARRAY MONTHS EXAMPLE - Real-world fixed-size array use

fn array_months_example() {
    // REAL-WORLD EXAMPLE: Month names (always 12)
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    for (index, month) in months.iter().enumerate() {
        println!("Month {}: {}", index + 1, month);
    }
}

// SAFE ARRAY ACCESS - Bounds checking at runtime

fn safe_array_access_example() {
    let a = [10, 20, 30, 40, 50];
    println!("Array: {:?}", a);
    println!("Length: {}", a.len());

    // VALID ACCESS
    println!("Valid: a[0]={}, a[4]={}", a[0], a[4]);

    // If user input exceeds bounds, Rust panics at runtime
    // This demonstrates Rust's memory safety principles

    // Safely check before accessing
    let index = 2;
    if index < a.len() {
        println!("Safe pattern: a[{}]={}", index, a[index]);
    }

    // Using get() method for safe access (returns Option)
    match a.get(0) {
        Some(value) => println!("get(0): {}", value),
        None => println!("Out of bounds"),
    }

    match a.get(10) {
        Some(value) => println!("get(10): {}", value),
        None => println!("get(10): safely handled (out of bounds)"),
    }
}
