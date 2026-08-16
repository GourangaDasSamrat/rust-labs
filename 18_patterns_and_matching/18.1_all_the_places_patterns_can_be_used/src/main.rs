use std::sync::mpsc;
use std::thread;

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    // match arms
    let x = Some(5);
    let result = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("match result: {:?}", result);

    // let statement destructuring
    let (a, b, c) = (1, 2, 3);
    println!("let destructure: {a}, {b}, {c}");
    // let (x, y) = (1, 2, 3); // mismatched tuple lengths

    // if let / else if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("while let received: {value}");
    }

    // for loop destructuring
    let values = vec!['a', 'b', 'c'];
    for (index, value) in values.iter().enumerate() {
        println!("for loop: {value} is at index {index}");
    }

    // function parameters as patterns
    let point = (3, 5);
    print_coordinates(&point);
}
