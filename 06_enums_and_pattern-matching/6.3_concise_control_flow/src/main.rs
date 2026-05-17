// CONCISE CONTROL FLOW WITH if let AND let...else
// Learning file demonstrating Rust's concise pattern matching techniques
// Author: Learning Rust
// Topic: Enums and Pattern Matching - Chapter 6.3

// #[derive(Debug)] allows printing enum variants with {:?}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
}

impl UsState {
    // Method to check if a state existed in a given year
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Arizona => year >= 1912,
            UsState::Arkansas => year >= 1836,
            UsState::California => year >= 1850,
            UsState::Colorado => year >= 1876,
            UsState::Connecticut => year >= 1788,
            UsState::Delaware => year >= 1787,
            UsState::Florida => year >= 1845,
            UsState::Georgia => year >= 1788,
        }
    }
}

fn main() {
    section1_verbose_match();
    section2_if_let_basic();
    section3_if_let_else();
    section4_let_else();
    section5_comparison();
    section6_tradeoffs();
}

fn section1_verbose_match() {
    // Traditional match expression: works but verbose for single-pattern cases
    // Pros: Exhaustive checking (compiler ensures all cases are handled)
    // Cons: Boilerplate code with _ => () for ignored cases

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        None => (), // Boilerplate: we have to handle None even though we don't care
    }

    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(_) => println!("This is a quarter"),
        _ => println!("This is not a quarter"), // Boilerplate: _ => () pattern
    }
}

fn section2_if_let_basic() {
    // if let is syntax sugar for match when you only care about ONE pattern
    // You can think of it as: "if this pattern matches, then execute this code"
    // All other patterns are silently ignored (no boilerplate needed!)

    println!("Example 1: Same as section1 but with if let");
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // Notice: No need to handle None explicitly!

    let coin = Coin::Quarter(UsState::California);

    if let Coin::Quarter(state) = coin {
        println!("Found a quarter from {state:?}");
    }

    let coins = vec![
        Coin::Penny,
        Coin::Quarter(UsState::Colorado),
        Coin::Dime,
        Coin::Quarter(UsState::Florida),
    ];

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("This quarter is from {state:?}");
        }
    }
}

fn section2_if_let_basic_continued() {
    // Key difference between match and if let:
    // - match: Forces you to think about ALL cases (exhaustive)
    // - if let: Only care about ONE case, ignore the rest

    println!("Example 4: Why if let is useful");
    let maybe_value = Some(42);

    // With match, you'd write:
    // match maybe_value {
    //     Some(v) => println!("Got {v}"),
    //     None => {},  // Do nothing
    // }

    // With if let, it's cleaner:
    if let Some(v) = maybe_value {
        println!("Got {v}");
    }
}

fn section3_if_let_else() {
    // if let...else: When you want to handle BOTH the matching and non-matching cases
    // This is useful for counting or performing different actions in each case

    let mut count = 0;
    let coin = Coin::Nickel;

    // This handles both the Quarter case AND the "everything else" case
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
        println!("This is not a quarter. Non-quarter count: {count}");
    }

    let coins = vec![
        Coin::Penny,
        Coin::Quarter(UsState::Colorado),
        Coin::Dime,
        Coin::Nickel,
    ];

    let mut non_quarter_count = 0;

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("Found quarter from {state:?}");
        } else {
            non_quarter_count += 1;
        }
    }
    println!("Found {non_quarter_count} non-quarter coins");
}

fn section4_let_else() {
    // let...else: A modern approach that lets you stay on the "happy path"
    // The pattern must match, otherwise the else branch MUST return/break/continue
    // This keeps the main logic in the outer scope, error handling pushed to else

    let describe_coin_v1 = |coin: Coin| -> String {
        // With let...else: if pattern matches, bind and continue
        // if it doesn't match, the else block MUST exit the function
        let Coin::Quarter(state) = coin else {
            return String::from("Not a quarter");
        };

        // After let...else, we KNOW we have a state and can use it directly
        format!("Quarter from {state:?}")
    };

    println!("{}", describe_coin_v1(Coin::Dime));
    println!("{}", describe_coin_v1(Coin::Quarter(UsState::California)));

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        // If coin is NOT a Quarter, return None immediately
        // If coin IS a Quarter, bind the state and continue
        let Coin::Quarter(state) = coin else {
            return None;
        };

        // Now we're on the "happy path" - we KNOW we have a state
        // The main logic is easy to follow at this level of indentation

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    // Test the function
    println!("Testing with a quarter:");
    match describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        Some(description) => println!("  {}", description),
        None => println!("  Not a quarter"),
    }

    println!("Testing with a penny:");
    match describe_state_quarter(Coin::Penny) {
        Some(description) => println!("  {}", description),
        None => println!("  Not a quarter"),
    }

    let coin_a = Coin::Quarter(UsState::California);
    if let Coin::Quarter(state) = coin_a {
        println!("Approach A (nested if let): {state:?}");
    } else {
        println!("Not a quarter");
    }

    let coin_b = Coin::Quarter(UsState::California);
    let Coin::Quarter(state) = coin_b else {
        println!("Not a quarter");
        return;
    };
    println!("Approach B (let...else): {state:?}");
}

fn section5_comparison() {
    // Same problem solved three different ways
    // Problem: Extract the value from Some(x) and use it, or do nothing if None

    let maybe_count = Some(5);

    match maybe_count {
        Some(count) => println!("Count is {count}"),
        None => (),
    }

    if let Some(count) = maybe_count {
        println!("Count is {count}");
    }

    let coins = vec![
        Coin::Quarter(UsState::California),
        Coin::Dime,
        Coin::Quarter(UsState::Colorado),
        Coin::Penny,
    ];

    // SOLUTION 1: Using if let in a loop
    for coin in &coins {
        if let Coin::Quarter(state) = coin {
            println!("  Found quarter from {state:?}");
        }
    }

    // SOLUTION 2: Using if let...else to count
    let mut quarters = 0;
    let mut others = 0;
    for coin in &coins {
        if let Coin::Quarter(_) = coin {
            quarters += 1;
        } else {
            others += 1;
        }
    }
    println!("  Quarters: {quarters}, Others: {others}");
}

fn section6_tradeoffs() {
    // Understanding when to use match, if let, and let...else

    fn validate_config(config: Option<u8>) -> String {
        // Using let...else for clear control flow
        let Some(value) = config else {
            return String::from("No configuration provided");
        };

        // Happy path: we know we have a value
        if value > 100 {
            String::from("Config exceeds maximum")
        } else {
            format!("Config is valid: {value}")
        }
    }

    println!("validate_config(Some(50)): {}", validate_config(Some(50)));
    println!("validate_config(Some(150)): {}", validate_config(Some(150)));
    println!("validate_config(None): {}", validate_config(None));
}
