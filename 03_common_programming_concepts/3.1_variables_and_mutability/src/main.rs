fn main() {
    // Constants: Declared with 'const', always immutable, must have explicit type annotation
    // Constants are evaluated at compile-time and cannot be changed
    const AGE: u8 = 17;

    println!("The value of age constant is: {AGE}");

    /*
    // Immutable variables: Variables declared with 'let' are immutable by default
    // This means you CANNOT reassign the value after initialization
    // Uncommenting the code below will cause a compile-time error
    let age = 17;
    println!("The value of age is: {age}");
    age = 6;  // ERROR: Cannot reassign an immutable variable
    println!("The value of age is: {age}");
    */

    // Mutable variables: Declared with 'let mut' to allow reassignment
    // You can change the value, but the type must remain the same
    let mut age = 17;
    println!("The value of age is: {age}");
    age = 6; // This is allowed because 'mut' keyword was used
    println!("The value of age is: {age}");

    // Shadowing: Creating a new variable with the same name as a previous variable
    // The new variable 'shadows' or hides the previous one
    // This is different from mutation - it creates a new binding
    let x = 5;

    // Re-declare x with a new binding (shadowing the previous x)
    // Unlike mutation, shadowing can change the type
    let x = x + 1;

    {
        // Inner scope shadowing: Creates a new x that only exists in this block
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // After the block, the outer x is still in scope (the one assigned x + 1)
    println!("The value of x is: {x}");
}
