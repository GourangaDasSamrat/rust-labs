/// Demonstrates basic iterator usage with the `iter()` method
///
/// This example shows how to:
/// - Create an iterator from an array using `iter()`
/// - Consume the iterator in a for loop
/// - Access each element from the iterator
fn main() {
    // Create an array of integers
    let v1 = [1, 2, 3];

    // Create an iterator by calling iter() on the array
    // iter() returns a borrowed iterator (does not consume the array)
    let v1_iter = v1.iter();

    // Iterate through the values using a for loop
    // The for loop automatically calls next() on the iterator
    for val in v1_iter {
        println!("Got: {val}");
    }
}
