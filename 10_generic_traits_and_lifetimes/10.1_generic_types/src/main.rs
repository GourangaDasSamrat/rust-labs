#![allow(unused)]

use std::cmp::PartialOrd;

// Generics In Function Definitions
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
// Generics In Struct Definitions
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
// with multiple types
struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

// Generics In Enum Definitions
enum Option<T> {
    Some(T),
    None,
}

// Generics In Method Definitions
struct ThirdPoint<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> ThirdPoint<T> {
    const fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    // Use Generics In Function
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['a', 'b', 'Z', 'z'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Use Generics In Struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("The integer struct {integer:#?} and the float struct {float:#?}");

    // with multiple types
    let both_integer = AnotherPoint { x: 5, y: 10 };
    let both_float = AnotherPoint { x: 1.0, y: 4.0 };
    let integer_and_float = AnotherPoint { x: 5, y: 4.0 };

    println!(
        "The both integer struct {both_integer:#?}x the both float struct {both_float:#?} and the integer and float struct {integer_and_float:#?}"
    );

    // Use Generics in Method
    let p = ThirdPoint { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
