// Lifetime rules
/*
1. a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/

#[allow(clippy::unwrap_used, unused)]
fn main() {
    // Dangling References
    /*
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
    */

    // Use Lifetime via Function
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");

    let result1 = longest(&s1, &s2);
    println!("The longest string is {result1}");

    // Use Lifetime via Structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // The Static Lifetime
    // All string literals have the 'static lifetime.
    let s: &'static str = "I have a static lifetime.";

    // Use Generic Type Parameters, Trait Bounds, and Lifetimes
    let result = longest_with_an_announcement(&s1, &s2, "Today is someone's birthday!");
    println!("The longest string is {result}");
}

// Lifetime In Function Signatures
const fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime In Struct Definitions
#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// In Method Definitions
#[allow(unused, clippy::pedantic)]
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
