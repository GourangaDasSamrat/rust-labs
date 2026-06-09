use std::{env, fs};

#[allow(clippy::expect_used, clippy::indexing_slicing)]
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Extract search query from the first argument (args[0] is the program name)
    let query = &args[1];
    // Extract the file path from the second argument
    let file_path = &args[2];

    // Display the search query to the user
    println!("Searching for {query}");
    // Display the file being searched
    println!("In file {file_path}");

    // Read the entire file contents into a string, panic if file cannot be read
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Print the file contents to verify what we're searching in
    println!("With text:\n{contents}");

    // Usage example: `cargo run -- the letter.txt`
}
