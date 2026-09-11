// This is the binary crate for the macro examples.
// We import the macros exported from the library crate and then use them like the
// macros in the Rust book, such as vec! and println!.
use macros::{greet, my_vec};

fn main() {
    // A function has a fixed signature, but a macro can accept a variable number of
    // arguments. The macro below expands to code that creates a Vec from the values
    // we write inside the macro call.
    let numbers = my_vec![10, 20, 30, 40];
    let fruits = my_vec!["apple", "banana", "cherry"];

    println!("numbers = {:?}", numbers);
    println!("fruits = {:?}", fruits);

    // This demonstrates that a macro can match different patterns and generate
    // corresponding code.
    greet!("Rust");
    greet!("Ada", "Lovelace");

    // We can also combine macros with normal Rust code to show the expanded result.
    let doubled = my_vec![1, 2, 3, 4]
        .into_iter()
        .map(|n| n * 2)
        .collect::<Vec<_>>();

    println!("doubled = {:?}", doubled);
}
