// Learning: The match control flow construct in Rust is like a coin-sorting machine.
// Values move through each pattern until they find one that fits, then execute the
// associated code block. Unlike if statements that require Boolean conditions, match
// can compare against any type and its patterns can be literals, variables, or wildcards.

// Example 1: Basic Enum Matching - US Coin Value Counter
// Learning: match arms consist of a pattern and code separated by =>. The compiler
// ensures all possible cases are handled (exhaustiveness), protecting us from logic bugs.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // Each arm checks if coin matches that variant and returns the corresponding value
    match coin {
        Coin::Penny => 1, // Short arms don't need curly braces or commas
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    // Learning: The value returned is the value of the matching arm's expression
}

// Example 2: Multi-line Arm Code with Curly Braces
// Learning: When you need multiple statements in a match arm, use curly braces.
// The last value in the block is what gets returned. The comma after the block is optional.

fn value_in_cents_verbose(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Example 3: Pattern Binding to Extract Enum Data
// Learning: Enums can hold data inside their variants. match arms can bind variables
// to extract that data. This allows us to access the inner values while pattern matching.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            // The state variable binds to the UsState value inside Quarter
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Example 4: Matching on Option<T>
// Learning: Option<T> is one of Rust's most powerful enums. match lets us handle
// both Some(value) and None cases. This prevents null pointer errors by forcing
// us to handle the absence of a value explicitly.

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // Handle the case where there's no value
        Some(i) => Some(i + 1), // i binds to the value inside Some
    }
}

// Example 5: Demonstrating Option<T> Matching
// Learning: When you call plus_one(Some(5)), the variable i binds to 5.
// The compiler checks that we handle all possible outcomes (None and Some variants).

fn demonstrate_option_matching() {
    let five = Some(5);
    let six = plus_one(five); // Some(6)
    let none = plus_one(None); // None

    println!("Plus one of Some(5): {:?}", six);
    println!("Plus one of None: {:?}", none);
}

// Example 6: Non-Exhaustive Match (COMMENTED OUT - DOES NOT COMPILE)
// Learning: Rust requires match expressions to be exhaustive. If you forget to handle
// a case like None, the compiler will refuse to compile and show you exactly what's missing.
// This prevents runtime bugs where you assume a value exists when it might be None.

/*
fn plus_one_buggy(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // BUG: forgot to handle None case! Compiler error E0004
    }
}
*/

// Example 7: Catch-All Pattern with Variable Binding
// Learning: The catch-all pattern matches any value not covered by previous arms.
// Use a variable name to capture the value, or use _ to ignore it.

fn game_dice_roll_v1() {
    let dice_roll = 9;

    match dice_roll {
        3 => {
            println!("You rolled a 3! Got a fancy hat!");
            add_fancy_hat();
        }
        7 => {
            println!("You rolled a 7! Lost your fancy hat!");
            remove_fancy_hat();
        }
        other => {
            // other captures any value not explicitly matched (1,2,4,5,6,8,9...)
            println!("You rolled {}. Moving {} spaces.", other, other);
            move_player(other);
        }
    }
}

// Example 8: Using _ Wildcard Pattern
// Learning: The _ pattern matches any value but doesn't bind to it. Use _ when
// you don't care about the value. This tells Rust we intentionally ignore it,
// preventing unused variable warnings.

fn game_dice_roll_v2() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // _ matches all other values without binding
    }
}

// Example 9: Unit Value as Default Action
// Learning: The unit value () (empty tuple) means "do nothing". Using _ => ()
// tells Rust we're exhaustively handling all cases but have no action for default.

fn game_dice_roll_v3() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // All other rolls: do nothing, turn ends
    }
}

// Placeholder functions for game examples
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn reroll() {}

// Example 10: Combining Multiple Concepts
// Learning: Real-world code often combines all these concepts: enums with associated
// data, pattern binding, and exhaustive matching. This creates type-safe code where
// the compiler verifies all cases are handled.

#[derive(Debug)]
enum HttpStatus {
    Ok(String),               // 200 with response body
    NotFound,                 // 404
    ServerError(u16, String), // 5xx with code and message
}

fn handle_http_response(status: HttpStatus) -> String {
    match status {
        HttpStatus::Ok(body) => {
            format!("Success: {}", body)
        }
        HttpStatus::NotFound => String::from("Page not found"),
        HttpStatus::ServerError(code, msg) => {
            format!("Server error {}: {}", code, msg)
        }
    }
}

fn main() {
    // Test basic coin matching
    println!("Coin values:");
    println!("Penny: {}¢", value_in_cents(Coin::Penny));
    println!("Quarter: {}¢", value_in_cents(Coin::Quarter));

    println!("\nVerbose coin matching:");
    println!("Penny: {}¢", value_in_cents_verbose(Coin::Penny));

    // Test state quarters
    println!("\nState quarters:");
    println!(
        "Alaska quarter: {}¢",
        value_in_cents_with_state(CoinWithState::Quarter(UsState::Alaska))
    );

    // Test Option matching
    println!("\nOption matching:");
    demonstrate_option_matching();

    // Test dice games
    println!("\nDice roll (version 1 - catch-all with value):");
    game_dice_roll_v1();

    println!("\nDice roll (version 2 - wildcard pattern):");
    game_dice_roll_v2();

    println!("\nDice roll (version 3 - do nothing for others):");
    game_dice_roll_v3();

    // Test HTTP status matching
    println!("\nHTTP responses:");
    let success = HttpStatus::Ok(String::from("Hello World"));
    let not_found = HttpStatus::NotFound;
    let error = HttpStatus::ServerError(500, String::from("Internal Server Error"));

    println!("{}", handle_http_response(success));
    println!("{}", handle_http_response(not_found));
    println!("{}", handle_http_response(error));
}
