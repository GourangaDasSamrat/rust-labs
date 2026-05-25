use std::collections::HashMap;

fn main() {
    create_and_insert_hashmap();
    access_values_example();
    ownership_example();
    updating_values_example();
    entry_api_example();
    update_based_on_old_value();
    iteration_example();
    word_count_project();
}

fn create_and_insert_hashmap() {
    println!("\n1. Creating and Inserting into a HashMap");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);
}

fn access_values_example() {
    println!("\n2. Accessing Values from a HashMap");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score);

    let missing_team = String::from("Red");
    let missing_score = scores.get(&missing_team).copied().unwrap_or(0);
    println!(
        "Score for {} (not found, returns default): {}",
        missing_team, missing_score
    );
}

fn ownership_example() {
    println!("\n3. Ownership in HashMap");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("Map after insertion: {:?}", map);

    if let Some(color) = map.get(&String::from("Favorite color")) {
        println!("Color value: {}", color);
    }
}

fn updating_values_example() {
    println!("\n4. Overwriting Values in a HashMap");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("After overwriting Blue from 10 to 25: {:?}", scores);
}

fn entry_api_example() {
    println!("\n5. Using the Entry API to Insert Only If Key Doesn't Exist");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("After entry API: {:?}", scores);

    println!("Blue already had a value, so it wasn't changed.");
    println!("Yellow didn't exist, so 50 was inserted.");
}

fn update_based_on_old_value() {
    println!("\n6. Updating a Value Based on the Old Value");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count: {:?}", map);
    println!("We use entry() to get a mutable reference, then dereference it with * to increment.");
}

fn iteration_example() {
    println!("\n7. Iterating Over HashMap Keys and Values");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);

    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    println!("Note: Iteration order is arbitrary, not guaranteed to be in insertion order.");
}

fn word_count_project() {
    println!("\n8. Real-World Example: Word Frequency Counter");

    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_freq = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_freq.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word frequencies:");
    for word in &["the", "fox", "quick", "nonexistent"] {
        let freq = word_freq.get(*word).copied().unwrap_or(0);
        println!("  '{}': appears {} time(s)", word, freq);
    }
}
