use std::fmt;
use std::ops::Add;

// Learning note:
// Associated types connect a trait to a type placeholder that each implementation fixes once.
// This is different from specifying a generic parameter on the trait itself, because a type
// can only implement a trait once, but it could implement the same generic trait multiple times
// with different type arguments.
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            None
        } else {
            let value = self.current;
            self.current += 1;
            Some(value)
        }
    }
}

fn associated_type_demo() {
    let mut counter = Counter { current: 0, max: 3 };

    println!("Associated type demo:");
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

// Learning note:
// Default generic type parameters help us write trait definitions that are easy to use most of the time,
// while still allowing specialized behavior when needed. The standard library's Add trait is a classic example.
//
// trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn operator_overloading_demo() {
    let point_a = Point { x: 1, y: 0 };
    let point_b = Point { x: 2, y: 3 };

    println!("Operator overloading demo:");
    println!("{:?}", point_a + point_b);

    let mm = Millimeters(10);
    let m = Meters(2);
    println!("{:?}", mm + m);
}

// Learning note:
// Traits can have methods with the same name. If the type itself also defines a method with the same name,
// Rust prefers the direct method on the type. For trait methods, we can disambiguate by specifying the trait
// or using fully qualified syntax when needed.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn method_disambiguation_demo() {
    let person = Human;

    println!("Method disambiguation demo:");
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// Learning note:
// A supertrait is a required trait dependency. If `OutlinePrint: Display`, then any type that implements
// `OutlinePrint` must also implement `Display`, so the method body can use `to_string()` safely.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn supertrait_demo() {
    let p = Point { x: 1, y: 3 };
    println!("Supertrait demo:");
    p.outline_print();
}

// Learning note:
// The orphan rule says we cannot implement an external trait for an external type directly.
// The newtype pattern wraps the foreign type inside a local tuple struct, then implements the target trait
// for the wrapper type. This preserves behavior without changing the underlying type.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newtype_pattern_demo() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Newtype pattern demo:");
    println!("w = {w}");
}

fn main() {
    // Section 1: Associated types
    associated_type_demo();

    // Section 2: Default generic parameters and operator overloading
    operator_overloading_demo();

    // Section 3: Disambiguating identical names
    method_disambiguation_demo();

    // Section 4: Supertraits
    supertrait_demo();

    // Section 5: Newtype pattern
    newtype_pattern_demo();
}
