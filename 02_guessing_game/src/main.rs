// Import the Ordering enum to compare values
use std::cmp::Ordering;
// Import io module for standard input/output operations
use std::io;

// Import the Rng trait for random number generation
use rand::Rng;

fn main() {
    // Display welcome message to the player
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Start the main game loop that continues until the player wins
    loop {
        // Prompt the player to enter their guess
        println!("Please input your guess.");

        // Create a mutable string to store the user's input
        let mut guess = String::new();

        // Read the input line from standard input and handle potential errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input string to a u32 integer, skip invalid input and continue loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Display the player's guess
        println!("You guessed: {guess}");

        // Compare the player's guess with the secret number and provide feedback
        match guess.cmp(&secret_number) {
            // If guess is less than secret number
            Ordering::Less => println!("Too small!"),
            // If guess is greater than secret number
            Ordering::Greater => println!("Too big!"),
            // If guess equals the secret number
            Ordering::Equal => {
                println!("You win!");
                // Exit the game loop when the player guesses correctly
                break;
            }
        }
    }
}
