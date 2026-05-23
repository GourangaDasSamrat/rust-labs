// Vectors in Rust allow you to store multiple values of the same type
// in a single data structure where all values are stored next to each other in memory.
// Vec<T> is useful for lists like file lines or shopping cart prices.

fn main() {
    create_vectors();
    update_vector();
    read_elements();
    access_patterns();
    iterate_vectors();
    store_multiple_types();
}

fn create_vectors() {
    println!("\n--- Creating Vectors ---");

    // Method 1: Create an empty vector with explicit type annotation.
    // Rust needs the type annotation when the vector is empty because it can't infer the type.
    let v: Vec<i32> = Vec::new();
    println!("Empty vector created: {:?}", v);

    // Method 2: Create a vector with initial values using the vec! macro.
    // This is more common because Rust infers the type from the values.
    // Here it infers Vec<i32> from the literal values 1, 2, 3.
    let v = vec![1, 2, 3];
    println!("Vector with initial values: {:?}", v);

    // The vec! macro works with any type, and Rust infers accordingly.
    let string_vec = vec!["hello", "world"];
    println!("String vector: {:?}", string_vec);
}

fn update_vector() {
    println!("\n--- Updating Vectors ---");

    // To modify a vector, we need to declare it as mutable using 'mut'.
    let mut v = Vec::new();

    // The push method adds an element to the end of the vector.
    // Rust infers the type is i32 from the value we push.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector after pushing values: {:?}", v);
}

fn read_elements() {
    println!("\n--- Reading Elements: Indexing vs get Method ---");

    let v = vec![1, 2, 3, 4, 5];

    // Method 1: Indexing with [] returns a direct reference.
    // Vectors are zero-indexed, so index 2 gives the third element.
    let third = &v[2];
    println!("Third element using indexing: {}", third);

    // Method 2: The get method returns an Option<&T>.
    // This is safer because it handles out-of-bounds access gracefully.
    match v.get(2) {
        Some(third) => println!("Third element using get: {}", third),
        None => println!("There is no third element."),
    }

    // Demonstration: using a loop to access multiple elements by index.
    println!("Accessing elements via loop:");
    for index in 0..v.len() {
        println!("  v[{}] = {}", index, v[index]);
    }
}

fn access_patterns() {
    println!("\n--- Panic Behavior vs Safe Option ---");

    let v = vec![1, 2, 3, 4, 5];

    // Using get with an out-of-bounds index safely returns None.
    // This is the preferred approach when you can't guarantee valid indices.
    match v.get(100) {
        Some(value) => println!("Found: {}", value),
        None => println!("Index 100 is out of bounds (safely handled with get)"),
    }

    // Uncomment the next line to see how indexing with [] panics on out-of-bounds:
    // let does_not_exist = &v[100];  // This would panic!

    println!("\nVector length: {}", v.len());
}

fn iterate_vectors() {
    println!("\n--- Iterating Over Vectors ---");

    // Immutable iteration: iterate over references to each element.
    let v = vec![100, 32, 57];
    println!("Iterating immutably:");
    for i in &v {
        println!("  {}", i);
    }

    // Mutable iteration: iterate over mutable references to modify elements.
    // We can use the dereference operator * to access and modify the value.
    let mut v = vec![100, 32, 57];
    println!("Iterating mutably (adding 50 to each):");
    for i in &mut v {
        *i += 50;
    }
    println!("Modified vector: {:?}", v);

    // Safe iteration rules: The borrow checker prevents modifying the vector
    // while holding references, which would invalidate them.
    // For example, inserting or removing items during iteration causes a compile error.
}

fn store_multiple_types() {
    println!("\n--- Storing Multiple Types Using Enums ---");

    // Vectors can only hold one type, but we can use an enum to store different types.
    // This allows us to have heterogeneous collections while maintaining type safety.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Processing spreadsheet row with mixed types:");
    for (index, cell) in row.iter().enumerate() {
        match cell {
            SpreadsheetCell::Int(n) => println!("  Cell {}: Integer {}", index, n),
            SpreadsheetCell::Float(f) => println!("  Cell {}: Float {}", index, f),
            SpreadsheetCell::Text(s) => println!("  Cell {}: Text {}", index, s),
        }
    }

    // The enum approach ensures Rust knows all possible types at compile time
    // and the match expression handles every case.
}

// Note: Vectors are automatically freed when they go out of scope.
// When a vector is dropped, all its contents are also cleaned up.
// The borrow checker ensures references to vector contents are only valid
// while the vector itself remains valid.
