//! ================================================================================
//! LEARNING FILE: DEFINING AND INSTANTIATING STRUCTS IN RUST
//! ================================================================================
//!
//! This file demonstrates all key concepts from "The Rust Programming Language"
//! Chapter 5: Defining and Instantiating Structs.
//!
//! KEY CONCEPTS:
//! 1. Basic struct definition and instantiation
//! 2. Accessing fields with dot notation
//! 3. Mutability of instances
//! 4. Field init shorthand syntax
//! 5. Struct update syntax
//! 6. Tuple structs
//! 7. Unit-like structs
//! 8. Ownership considerations
//! 9. Debug trait for printing
//! 10. Practical examples with Rectangle
//! ================================================================================

// ========================
// 1. BASIC STRUCT DEFINITION
// ========================
//
// A struct is a custom data type that lets you name and group multiple related values.
// - Similar to tuples, but with named fields (more flexible than relying on order)
// - Each field has a name and type
// - Struct names describe the significance of the grouped data
//
// Example: User struct that stores account information
struct User {
    active: bool,       // Is the account active?
    username: String,   // Username (owned String, not &str - we'll explain why)
    email: String,      // Email address (owned String)
    sign_in_count: u64, // Number of times user signed in
}

// ===========================
// 2. BASIC INSTANTIATION
// ===========================
//
// To create an instance:
// 1. Use the struct name followed by curly brackets {}
// 2. Specify key: value pairs for each field
// 3. Order doesn't matter (unlike tuples!)
// 4. Use dot notation (.) to access fields
//
fn basic_instantiation_example() {
    // Create an instance of User
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Access fields using dot notation
    println!("User email: {}", user1.email);
    println!("User active: {}", user1.active);
}

// ===============================
// 3. MUTATING STRUCT INSTANCES
// ===============================
//
// To modify fields:
// 1. The ENTIRE instance must be declared with mut
// 2. Rust doesn't allow marking only certain fields as mutable
// 3. Use dot notation to assign new values
//
fn mutable_instance_example() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Modify email field using dot notation
    user1.email = String::from("newemail@example.com");
    println!("Updated email: {}", user1.email);

    // We can't do this in Rust:
    // let user2 = User { ... }; // Would need "let mut" to modify any field
}

// ====================================
// 4. FUNCTIONS RETURNING STRUCT INSTANCES
// ====================================
//
// Structs can be the return type of functions.
// The last expression (without semicolon) implicitly returns the value.
//
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // Repetitive - we can optimize this!
        email: email,       // Repetitive - we can optimize this!
        sign_in_count: 1,
    }
}

// ==========================================
// 5. FIELD INIT SHORTHAND SYNTAX
// ==========================================
//
// When function parameter names match struct field names:
// - Instead of writing: username: username
// - Just write: username
//
// This is a concise way to reduce repetition
//
fn build_user_optimized(email: String, username: String) -> User {
    User {
        active: true,
        username, // Shorthand for username: username
        email,    // Shorthand for email: email
        sign_in_count: 1,
    }
}

// ============================================
// 6. STRUCT UPDATE SYNTAX (.. operator)
// ============================================
//
// When creating a new struct instance that's mostly the same as an existing one:
// - Use the .. operator to fill remaining fields from the old instance
// - The .. must come LAST in the struct initialization
// - Specify only the fields you want to change
// - This is more efficient than manually copying every field
//
// IMPORTANT: This uses a MOVE for non-Copy types (like String)!
// After this operation, fields with moved types can no longer be accessed.
//
fn struct_update_example() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Old way: manually copy everything
    let _user2_old = User {
        active: user1.active,
        username: user1.username, // This MOVES the String
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // After this, user1.username is no longer accessible!

    // Better way: use struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // Fill remaining fields from user1
    };
    // user1.username was moved to user3!
    // But user1.active and user1.sign_in_count are still accessible
    // (because bool and u64 implement the Copy trait)

    println!("user1.active still works: {}", user1.active);
    // println!("user1.username no longer works: {}", user1.username); // Compile error!
}

// ================================
// 7. TUPLE STRUCTS
// ================================
//
// Structs that look like tuples but with a struct name.
// Useful when:
// - You want to name the tuple as a whole
// - You want to distinguish it as a different type
// - Naming each field would be verbose or redundant
//
// Access fields using dot notation with indices (like tuples)
// Destructure using pattern matching
//
struct Color(i32, i32, i32); // Red, Green, Blue values
struct Point(i32, i32, i32); // X, Y, Z coordinates

fn tuple_struct_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Despite identical data, these are different types!
    // fn takes_color(c: Color) { } won't accept Point

    // Access using dot notation with indices
    println!("Black red value: {}", black.0);
    println!("Origin x coordinate: {}", origin.0);

    // Destructure using pattern matching
    let Color(r, g, b) = black;
    println!("Color components: r={}, g={}, b={}", r, g, b);
}

// =============================
// 8. UNIT-LIKE STRUCTS
// =============================
//
// Structs with NO fields!
// Useful when:
// - You need a type to implement traits but don't need stored data
// - You want distinct types for type-checking purposes
// - Placeholder for future functionality
//
struct AlwaysEqual; // Defined with just a semicolon, no brackets

fn unit_struct_example() {
    let subject = AlwaysEqual;
    println!("Unit struct created: {:?}", subject);
}

// ============================================
// 9. OWNERSHIP AND STRUCT DATA
// ============================================
//
// WHY WE USE String INSTEAD OF &str IN STRUCTS:
//
// When using owned types (String):
// - The struct owns all its data
// - The data is valid as long as the struct is valid
// - No need to worry about lifetimes of references
//
// When using references (&str):
// - The struct holds a reference to data owned elsewhere
// - Requires lifetime annotations (Chapter 10)
// - More complex but sometimes necessary
//
// This is why the User struct uses String, not &str.
//
// EXAMPLE: This code WON'T compile without lifetime annotations:
// struct UserWithRef {
//     active: bool,
//     username: &str,  // ERROR! Missing lifetime specifier
//     email: &str,     // ERROR! Missing lifetime specifier
//     sign_in_count: u64,
// }

// =========================================================
// 10. PRACTICAL EXAMPLE: CALCULATING RECTANGLE AREA
// =========================================================

// ---- APPROACH 1: Separate variables (NOT RECOMMENDED) ----
// Problem: Unclear that width and height are related
fn calculate_area_separate() {
    let width1 = 30;
    let height1 = 50;
    let area = area_from_separate(width1, height1);
    println!("Area (separate): {} square pixels", area);
}

fn area_from_separate(width: u32, height: u32) -> u32 {
    width * height
}

// ---- APPROACH 2: Using tuples (BETTER, but not ideal) ----
// Problem: Index access (0, 1) is unclear. What does each index mean?
fn calculate_area_tuple() {
    let rect1 = (30, 50); // (width, height), but this is unclear!
    let area = area_from_tuple(rect1);
    println!("Area (tuple): {} square pixels", area);
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // Unclear what 0 and 1 represent
}

// ---- APPROACH 3: Using structs (RECOMMENDED) ----
// Benefits:
// - Clear that width and height belong together
// - Self-documenting code
// - Easy to add more related data and methods
// - More maintainable and readable
//
#[derive(Debug)] // We'll explain this attribute below
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Note: We pass a reference (&rect1) to keep ownership in main
    let area = area_from_struct(&rect1);
    println!("Area (struct): {} square pixels", area);
}

fn area_from_struct(rectangle: &Rectangle) -> u32 {
    // &Rectangle is an immutable borrow
    // This allows area() to use rect1 without taking ownership
    rectangle.width * rectangle.height
}

// ====================================
// 11. DEBUG TRAIT AND PRINTING
// ====================================
//
// THE PROBLEM:
// You can't print custom structs with println!("{}", rect1);
// Rust doesn't know how you want to format it.
//
// THE SOLUTION:
// Add the #[derive(Debug)] attribute above your struct.
// This automatically implements the Debug trait.
//
// USAGE:
// - {:?} for single-line debug output
// - {:#?} for pretty-printed multi-line output
// - dbg!() macro for debugging (prints to stderr, shows line number)
//
fn debug_printing_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Single-line debug format
    println!("Debug format: {rect1:?}");
    // Output: Debug format: Rectangle { width: 30, height: 50 }

    // Pretty-printed debug format (useful for complex structs)
    println!("Pretty debug format:\n{rect1:#?}");
    // Output:
    // Pretty debug format:
    // Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    // dbg!() macro: useful during development
    // - Takes ownership of the expression
    // - Prints file and line number
    // - Prints to stderr (not stdout)
    // - Returns ownership of the value
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // Prints: 30 * scale = 60
        height: 50,
    };

    dbg!(&rect2); // Prints the entire struct with file:line info
}

// =========================
// MAIN: RUN ALL EXAMPLES
// =========================
fn main() {
    println!("========== RUST STRUCTS LEARNING EXAMPLES ==========\n");

    println!("--- 1. Basic Instantiation ---");
    basic_instantiation_example();

    println!("\n--- 2. Mutable Instance ---");
    mutable_instance_example();

    println!("\n--- 3. Function Returning Struct ---");
    let user = build_user_optimized(String::from("test@example.com"), String::from("testuser"));
    println!("Built user: {} ({})", user.username, user.email);

    println!("\n--- 4. Struct Update Syntax ---");
    struct_update_example();

    println!("\n--- 5. Tuple Structs ---");
    tuple_struct_example();

    println!("\n--- 6. Unit-like Structs ---");
    unit_struct_example();

    println!("\n--- 7. Rectangle Area: Separate Variables ---");
    calculate_area_separate();

    println!("\n--- 8. Rectangle Area: Tuples ---");
    calculate_area_tuple();

    println!("\n--- 9. Rectangle Area: Structs ---");
    calculate_area_struct();

    println!("\n--- 10. Debug Printing ---");
    debug_printing_example();

    println!("\n========== END OF EXAMPLES ==========");
}

// ================================================================================
// SUMMARY OF KEY TAKEAWAYS:
// ================================================================================
//
// 1. STRUCTS are named collections of related data
//    - More readable than tuples for complex data
//    - Fields are accessed by name, not index
//
// 2. INSTANTIATION requires specifying all fields (order doesn't matter)
//    - Use key: value syntax
//    - Fields are accessed with dot notation
//
// 3. MUTABILITY must be declared on the entire instance
//    - No field-level mutability control
//
// 4. FIELD INIT SHORTHAND reduces repetition when parameter names match field names
//    - Use just the field name instead of name: name
//
// 5. STRUCT UPDATE SYNTAX (.. operator) reduces code when creating similar instances
//    - Watch out: non-Copy types are MOVED, not copied
//
// 6. TUPLE STRUCTS provide type-safe tuples with semantic meaning
//    - Each tuple struct is a different type
//
// 7. UNIT-LIKE STRUCTS are useful for trait implementation
//    - No data, just type identity
//
// 8. OWNERSHIP: Use owned types (String) unless you need lifetimes
//    - Simpler and more common in struct design
//
// 9. DEBUG TRAIT: Use #[derive(Debug)] to enable printing
//    - {:?} for single-line output
//    - {:#?} for pretty printing
//    - dbg!() for debugging expressions
//
// 10. STRUCT DESIGN PRINCIPLE:
//     When data is related, group it with a struct
//     This makes code more readable and maintainable
//
// ================================================================================
