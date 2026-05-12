// RUST METHODS
//
// Methods are similar to functions but are defined within the context of a
// struct (or enum/trait object). The key difference: their first parameter
// is always 'self', representing the instance of the struct.

// Enable Debug trait for our Rectangle struct so we can print it
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//
// SECTION 1: BASIC METHOD SYNTAX
//
// All methods for Rectangle are defined in an 'impl' (implementation) block.
// Everything within this impl block is associated with the Rectangle type.

impl Rectangle {
    // BASIC METHOD: area()
    // - Uses &self (immutable borrow) because we only read the data
    // - &self is short for self: &Self (Self is an alias for Rectangle)
    // - We don't take ownership because we just need to read width & height
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // KEY CONCEPT: Why &self?
    // There are three ways to borrow self in a method:
    // 1. &self      - immutable borrow (read-only) ← used here
    // 2. &mut self  - mutable borrow (can modify)
    // 3. self       - takes ownership (rare, transforms the instance)

    // ========================================================================
    // SECTION 2: AUTOMATIC REFERENCING & DEREFERENCING
    // ========================================================================
    // When you call rect1.area(), Rust automatically adds &, &mut, or *
    // to match the method signature.
    //
    // These are equivalent:
    // rect1.area()          ← clean syntax (what we use)
    // (&rect1).area()       ← explicit referencing
    //
    // This makes Rust ergonomic - you don't have to write &rect1.area()

    // ========================================================================
    // SECTION 3: METHODS WITH MULTIPLE PARAMETERS
    // ========================================================================
    // Methods can take additional parameters after self

    fn can_hold(&self, other: &Rectangle) -> bool {
        // Check if self can contain the other rectangle completely
        // Both width AND height of self must be greater than other's
        self.width > other.width && self.height > other.height
    }

    // Why &other?
    // - We only need to read the data in 'other'
    // - We don't want to take ownership (caller might use it again)
    // - So we use an immutable borrow (&Rectangle)

    // ========================================================================
    // SECTION 4: METHODS WITH SAME NAME AS FIELDS (GETTERS)
    //
    // You can name a method the same as a field. This is useful for "getters"
    // which allow read-only access to private fields.

    fn width(&self) -> bool {
        // Return true if width is greater than 0
        // This distinguishes the method width() from the field width
        self.width > 0
    }

    // When you use rect1.width(), Rust calls the method (parentheses mean method)
    // When you use rect1.width, Rust accesses the field (no parentheses)
    // This pattern is common in Rust for creating getters!

    // ========================================================================
    // SECTION 5: ASSOCIATED FUNCTIONS (No 'self' parameter)
    // ========================================================================
    // Functions in an impl block that DON'T have 'self' are called
    // "associated functions" - they're associated with the type but
    // don't operate on an instance.
    //
    // Common use: constructors that return a new instance

    fn square(size: u32) -> Self {
        // 'Self' is a type alias for Rectangle (the type after impl)
        // Creates a square by using the same value for width and height
        Self {
            width: size,
            height: size,
        }
    }

    // Associated functions are called with :: syntax (not . syntax)
    // Example: let sq = Rectangle::square(5);
    // This is similar to String::from() which you've already used!
}

// ========================================================================
// SECTION 6: MULTIPLE IMPL BLOCKS
// ========================================================================
// You can have multiple impl blocks for the same struct.
// While not necessary here, this is valid and useful for organizing code
// (we'll see this more with generics and traits in later chapters).

impl Rectangle {
    fn perimeter(&self) -> u32 {
        // Demonstrating a method in a separate impl block
        // All impl blocks are merged together - this is just organization
        2 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        // Another useful method: check if the rectangle is actually a square
        self.width == self.height
    }
}

//
// MAIN FUNCTION - PUTTING IT ALL TOGETHER
//

fn main() {
    println!("=== RUST METHODS LEARNING NOTES ===\n");

    // Create rectangle instances
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // ========================================================================
    // EXAMPLE 1: Basic Method Call - area()
    // ========================================================================
    println!("--- Example 1: Basic Method (area) ---");
    println!("Rectangle 1: {:?}", rect1);
    println!("Area of rect1: {} square pixels", rect1.area());
    // Method syntax: rect1.area() instead of area(&rect1)
    // The . notation makes it clear we're calling a method on an instance
    println!();

    // ========================================================================
    // EXAMPLE 2: Methods with Parameters - can_hold()
    // ========================================================================
    println!("--- Example 2: Method with Parameters (can_hold) ---");
    println!(
        "Can rect1 (30x50) hold rect2 (10x40)? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 (30x50) hold rect3 (60x45)? {}",
        rect1.can_hold(&rect3)
    );
    // Note: We pass &rect2 and &rect3 because can_hold() takes &Rectangle
    // Rust's automatic referencing handles this for us!
    println!();

    // ========================================================================
    // EXAMPLE 3: Method with Same Name as Field (Getter) - width()
    // ========================================================================
    println!("--- Example 3: Getter Method (width) ---");
    if rect1.width() {
        // Calling the method width() with parentheses
        println!("Rectangle has nonzero width: {} pixels", rect1.width);
        // Accessing the field width without parentheses
    }
    println!();

    // ========================================================================
    // EXAMPLE 4: Associated Function - square()
    // ========================================================================
    println!("--- Example 4: Associated Function (square) ---");
    let square = Rectangle::square(25);
    // Notice the :: syntax for associated functions (not . syntax)
    // This is similar to String::from("hello")
    println!("Created square: {:?}", square);
    println!("Area of square: {}", square.area());
    println!();

    // ========================================================================
    // EXAMPLE 5: Methods from Multiple impl Blocks
    // ========================================================================
    println!("--- Example 5: Methods from Multiple impl Blocks ---");
    println!("Rectangle 1 perimeter: {} pixels", rect1.perimeter());
    println!("Rectangle 1 is square? {}", rect1.is_square());
    println!("Square is square? {}", square.is_square());
    println!();
}

//
// COMPARISON - FUNCTION vs METHOD
//
//
// OLD WAY (using a function):
//   fn area_func(rectangle: &Rectangle) -> u32 {
//       rectangle.width * rectangle.height
//   }
//   let result = area_func(&rect1);  // Pass reference explicitly
//
// NEW WAY (using a method):
//   impl Rectangle {
//       fn area(&self) -> u32 {
//           self.width * self.height
//       }
//   }
//   let result = rect1.area();  // Cleaner, automatic referencing!
//
// BENEFITS OF METHODS:
// - More intuitive: object.action() feels natural
// - Better organization: all methods for a type are in one impl block
// - Automatic referencing: no need to manually write &
// - Namespacing: methods are tied to the type they work on
//
//
