use std::{thread, time::Duration};

// Define shirt colors as an enum for the inventory system
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// Store inventory of shirts and provide giveaway logic
struct Inventory {
    shirts: Vec<ShirtColor>,
}

// Methods for managing shirt inventory and giveaway
#[allow(clippy::arithmetic_side_effects)]
impl Inventory {
    // Giveaway method that uses a closure via unwrap_or_else
    // If user has a preference, use it; otherwise, use the most stocked color
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // Determine the most stocked shirt color by counting each color
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        // Iterate through shirts and count by color
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        // Return the color with the most count
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// Generate a workout plan based on intensity and random number
// Demonstrates expensive closures that are only called when needed
fn generate_workout(intensity: u32, random_number: u32) {
    // Define an expensive closure that simulates a slow calculation
    // This closure captures num by value (copy) and returns u32
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Generate workout based on intensity level
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
}

// Rectangle struct for demonstrating closures with sort_by_key
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Main program demonstrating various closure patterns
#[allow(clippy::unwrap_used)]
fn main() {
    // Example 1: Using closures with Option::unwrap_or_else
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");

    // Example 2: Expensive closures in workout generation
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Example 3: Closure that borrows from the environment
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // This closure borrows list immutably
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Example 4: Closure that takes ownership with the move keyword
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Example 5: Using sort_by_key with a closure
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // Sort rectangles by width using a closure as the sorting key
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
