/*
OVERVIEW:
─────────
Enums (enumerations) allow you to define a type by enumerating its possible variants.
- Structs: Group related fields together (like Rectangle with width & height)
- Enums: Define a value as one of a possible set of values

KEY DIFFERENCE:
An enum value can ONLY be ONE of its variants at any given time.
This is why enums are perfect for cases like IP addresses (IPv4 XOR IPv6).

*/

// PART 1: BASIC ENUM DEFINITION

// Simple enum without associated data
// Each variant (V4, V6) is a possible value of IpAddrKind
enum IpAddrKind {
    V4,  // An IP address variant for IPv4
    V6,  // An IP address variant for IPv6
}


// PART 2: ENUM INSTANCES


// Creating instances of enum variants using :: (double colon)
// Both values are of the SAME type: IpAddrKind
fn create_enum_instances() {
    let four = IpAddrKind::V4;   // First variant
    let six = IpAddrKind::V6;    // Second variant

    // Both have the same type!
    route_simple(&four);
    route_simple(&six);
}

// Function that accepts ANY IpAddrKind variant
fn route_simple(_ip_kind: &IpAddrKind) {
    // Note: Since both V4 and V6 are the same type, this function works for both
}


// PART 3: ENUM + STRUCT APPROACH (Less ideal, showing why enums are better)


// One way to associate data: enum + struct (verbose approach)
enum IpAddrKind_V2 {
    V4,
    V6,
}

struct IpAddr_Old {
    kind: IpAddrKind_V2,
    address: String,
}

fn enum_with_struct() {
    let home = IpAddr_Old {
        kind: IpAddrKind_V2::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr_Old {
        kind: IpAddrKind_V2::V6,
        address: String::from("::1"),
    };

    // This approach requires matching on the enum AND accessing the struct field
    // It's more verbose than directly putting data in the enum variant
    let _ = (home, loopback);
}


// PART 4: ENUMS WITH ASSOCIATED DATA (Better approach!)


// Each enum variant can hold associated data directly
// This is more concise than enum + struct!
// Variants can have different types and amounts of data
#[derive(Debug)]
enum IpAddr {
    V4(String),  // V4 variant holds a String
    V6(String),  // V6 variant holds a String
}

fn enum_with_data_simple() {
    // The enum variant names also act as constructor functions!
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // No need for a separate struct - data is directly in the enum
    let _ = (home, loopback);
}


// PART 5: ENUMS WITH DIFFERENT DATA TYPES FOR EACH VARIANT (ENUM SUPERPOWER!)


// This is where enums shine over structs!
// IPv4: always 4 numeric components (0-255)
// IPv6: can be represented as a single String
// Structs can't do this, but enums can!

#[derive(Debug)]
enum IpAddr_Better {
    V4(u8, u8, u8, u8),  // Four u8 values for IPv4 octets
    V6(String),           // Single String for IPv6 (different type!)
}

fn enum_mixed_types() {
    let home = IpAddr_Better::V4(127, 0, 0, 1);
    let loopback = IpAddr_Better::V6(String::from("::1"));

    // Each variant stores different types and amounts of data!
    // This flexibility is impossible with structs alone
    let _ = (home, loopback);
}


// PART 6: COMPLEX ENUM WITH MULTIPLE VARIANT TYPES


// Message enum demonstrates all the ways variants can store data
#[derive(Debug)]
enum Message {
    Quit,                              // No associated data (unit-like)
    Move { x: i32, y: i32 },          // Named fields (struct-like)
    Write(String),                     // Single value (tuple-like)
    ChangeColor(i32, i32, i32),       // Multiple values (tuple-like)
}

// Note: These struct equivalents would be separate types:
//   struct QuitMessage;
//   struct MoveMessage { x: i32, y: i32 }
//   struct WriteMessage(String);
//   struct ChangeColorMessage(i32, i32, i32);
// Problem: Each struct is a DIFFERENT TYPE!
// You'd need an outer enum anyway, so Message enum is cleaner.

fn message_enum() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 128),
    ];

    // All messages are the SAME type despite different data!
    for msg in messages {
        let _ = msg;
    }
}


// PART 7: DEFINING METHODS ON ENUMS (impl)


// Just like structs, you can implement methods on enums!

impl Message {
    // A method that handles each variant differently
    fn call(&self) {
        match self {
            Message::Quit => {},
            Message::Move { x, y } => {
                let _ = (x, y);
                // Access named fields from the Move variant
            }
            Message::Write(s) => {
                let _ = s;
                // Access the String from the Write variant
            }
            Message::ChangeColor(r, g, b) => {
                let _ = (r, g, b);
                // Access the three i32 values from ChangeColor variant
            }
        }
    }

    // Another method that returns data based on the variant
    fn get_description(&self) -> String {
        match self {
            Message::Quit => String::from("User quit"),
            Message::Move { x, y } => format!("Moving to position ({}, {})", x, y),
            Message::Write(text) => format!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => format!("Color RGB({}, {}, {})", r, g, b),
        }
    }
}

fn enum_methods() {
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::ChangeColor(200, 100, 50);

    m1.call();
    m2.call();

    // Methods on enums work just like on structs
    let desc1 = m1.get_description();
    let desc2 = m2.get_description();

    let _ = (desc1, desc2);
}


// PART 8: THE OPTION ENUM (Standard Library)


/*
PROBLEM: How to express "something or nothing"?
─────────
Many languages have NULL:
  - NULL means "no value"
  - But it's often called the "Billion Dollar Mistake" by Tony Hoare (2009)
  - Leads to crashes: trying to use null as a valid value

RUST'S SOLUTION: Option<T>
──────────────
Instead of NULL, Rust uses an enum:

    enum Option<T> {
        None,    // No value
        Some(T), // A value of type T
    }

KEY BENEFIT: Type Safety!
  - You can't accidentally treat Option<T> as T
  - Compiler forces you to handle both cases
  - Prevents null pointer errors at compile time!
  - This is a MAJOR Rust advantage

Option is SO useful it's in the prelude (no import needed)
You can use Some and None without Option:: prefix
*/

fn option_enum() {
    // Some examples with different types
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;

    // Type inference works with Some:
    // Rust knows some_number is Option<i32> from the value 5
    // But for None, you must explicitly state the type

    let _ = (some_number, some_char, absent_number);

    // WHY OPTION IS BETTER THAN NULL:
    // This won't compile:
    //     let x: i8 = 5;
    //     let y: Option<i8> = Some(5);
    //     let sum = x + y;  // ERROR! Can't add i8 + Option<i8>
    //
    // ERROR: cannot add `Option<i8>` to `i8`
    //
    // You MUST extract the value from Some before using it!
    // This forces you to handle the "nothing" case.
    // The compiler prevents null pointer bugs!
}

// Using Option to demonstrate type safety
fn demonstrating_option_safety() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This would NOT compile (demonstrating Rust's type safety):
    // let sum = x + y;  // ERROR!

    // Instead, you must extract the value using match:
    let sum = match y {
        Some(value) => x + value,  // Extract and use the value
        None => x,                  // Handle the None case
    };

    // Pattern matching with Option (using match)
    let value = Some(42);
    match value {
        Some(num) => {
            let _ = num;
            // num is guaranteed to be a valid i32 here
        }
        None => {
            // This code only runs if value is None
        }
    }

    let _ = sum;
}


// PART 9: GENERIC TYPE PARAMETERS - <T>


/*
What is <T>?
────────────
<T> is a GENERIC TYPE PARAMETER
  - T is a placeholder for any type
  - Option<i32> means T = i32
  - Option<String> means T = String
  - Option<bool> means T = bool

So Option<T> can work with ANY type!
We'll learn more about generics in the full chapter.

Benefits of Generics:
  - Write code once, works with many types
  - Type-safe: Compiler checks each concrete type
  - No runtime cost (monomorphization)
*/

fn generic_type() {
    // Each concrete type makes a different Option type:
    let int_value: Option<i32> = Some(42);      // Option<i32>
    let string_value: Option<String> = Some(String::from("hello")); // Option<String>
    let bool_value: Option<bool> = Some(true);  // Option<bool>

    // None still requires explicit type annotation:
    let no_value: Option<i32> = None;

    let _ = (int_value, string_value, bool_value, no_value);

    // All are different types in the type system, but share the same pattern
}


// PART 10: PRACTICAL ROUTING EXAMPLE WITH BETTER IPADDR


#[derive(Debug)]
enum IpAddr_Final {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr_Final {
    // Route function that handles both IPv4 and IPv6
    fn route(&self) {
        match self {
            IpAddr_Final::V4(a, b, c, d) => {
                // Each component is a u8
                let _ = (a, b, c, d);
            }
            IpAddr_Final::V6(addr) => {
                // addr is a String reference
                let _ = addr;
            }
        }
    }

    // Another useful method - type checking
    fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr_Final::V4(_, _, _, _))
    }
}

fn routing_example() {
    let home = IpAddr_Final::V4(127, 0, 0, 1);
    let loopback = IpAddr_Final::V6(String::from("::1"));

    home.route();
    loopback.route();

    let _is_v4 = home.is_ipv4();  // true
    let _is_v4_2 = loopback.is_ipv4();  // false
}


// PART 11: PATTERN MATCHING WITH MATCH EXPRESSION


/*
The match expression is THE primary way to work with enums.
It ensures you handle every possible variant.

Syntax:
    match value {
        variant1 => { /* handle variant1 */ }
        variant2 => { /* handle variant2 */ }
        // ... etc
    }

Key Benefits:
  - Exhaustive: Compiler forces you to handle all variants
  - Destructuring: Extract data from variants automatically
  - Pattern matching: More powerful than simple if-else
*/

fn match_patterns() {
    // Using match to handle all variants
    let addr = IpAddr_Final::V4(192, 168, 1, 1);

    match addr {
        IpAddr_Final::V4(a, b, c, d) => {
            // Automatically destructured!
            let _ = (a, b, c, d);
        }
        IpAddr_Final::V6(s) => {
            let _ = s;
        }
    }

    // Match can also return values
    let formatted = match addr {
        IpAddr_Final::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
        IpAddr_Final::V6(s) => s,
    };

    let _ = formatted;
}


// PART 12: WORKING WITH OPTION IN PRACTICE


// Real-world use: A function that might fail
fn find_first_matching(haystack: &[i32], needle: i32) -> Option<usize> {
    for (i, &val) in haystack.iter().enumerate() {
        if val == needle {
            return Some(i);  // Found it!
        }
    }
    None  // Not found
}

fn option_practical() {
    let numbers = vec![10, 20, 30, 40, 50];

    // Result is Option<usize>
    let result = find_first_matching(&numbers, 30);

    // Handle the Option result
    match result {
        Some(index) => {
            // We have a valid index
            let _ = index;
        }
        None => {
            // Value was not found
        }
    }

    // Alternative: if let syntax (simpler for single variant)
    if let Some(index) = result {
        let _ = index;
    }
}


// MAIN FUNCTION - DEMONSTRATES ALL ENUM CONCEPTS


fn main() {
    // PART 1: Basic enum instances
    create_enum_instances();

    // PART 2: Enum + struct approach
    enum_with_struct();

    // PART 3: Simple enum with data
    enum_with_data_simple();

    // PART 4: Enums with different data types
    enum_mixed_types();

    // PART 5: Complex message enum
    message_enum();

    // PART 6: Enum methods
    enum_methods();

    // PART 7: Option enum
    option_enum();

    // PART 8: Option type safety
    demonstrating_option_safety();

    // PART 9: Generic types
    generic_type();

    // PART 10: Practical routing
    routing_example();

    // PART 11: Pattern matching
    match_patterns();

    // PART 12: Option practical example
    option_practical();

    /*
    KEY TAKEAWAYS ABOUT RUST ENUMS:

    1. ENUM BASICS:
       - Enums define a value as ONE of a set of possible variants
       - Create instances with EnumName::VariantName
       - All variants are the same type

    2. ASSOCIATED DATA:
       - Each variant can hold different types and amounts of data
       - This is more flexible than structs for these use cases
       - Variants act as constructor functions

    3. COMPLEX VARIANTS:
       - Unit-like: Quit (no data)
       - Struct-like: Move { x, y } (named fields)
       - Tuple-like: Write(String) or ChangeColor(r, g, b)
       - Mix and match in the same enum!

    4. ENUM METHODS:
       - Use impl just like with structs
       - Methods can match on self to handle different variants
       - Enables safe, organized code

    5. OPTION<T>:
       - The standard library's way of handling "something or nothing"
       - Replaces null pointers (which are called a "billion dollar mistake")
       - Forces compiler checking: you MUST handle the None case
       - This prevents null pointer bugs - a major Rust advantage!

    6. PATTERN MATCHING WITH MATCH:
       - The primary way to work with enums
       - Compiler ensures you handle all variants
       - Automatically destructures data from variants
       - Can be exhaustive or use _ for default cases

    7. TYPE SAFETY:
       - Option<T> and T are different types
       - You can't accidentally mix them
       - The compiler prevents entire classes of bugs
       - This is core to Rust's safety philosophy

    */
}
