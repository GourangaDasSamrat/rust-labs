use std::collections::HashMap;

// The newtype pattern creates a distinct type wrapper around an existing one.
// It gives us stronger type safety without changing the underlying representation.
#[derive(Debug, Clone, Copy)]
struct Millimeters(u32);

#[derive(Debug, Clone, Copy)]
struct Meters(u32);

fn print_distance(d: Meters) {
    println!("Distance: {} meters", d.0);
}

// Newtypes can also hide implementation details behind a simpler public API.
struct People(HashMap<i32, String>);

impl People {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_person(&mut self, id: i32, name: &str) {
        self.0.insert(id, name.to_string());
    }

    fn get_name(&self, id: i32) -> Option<&str> {
        self.0.get(&id).map(String::as_str)
    }
}

// A type alias is not a new type. It is just another name for the same underlying type.
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type IoResult<T> = Result<T, std::io::Error>;

fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("This closure is stored as a thunk."))
}

// The never type `!` is used for code that never returns, such as `continue`, `panic!`,
// and loops that never terminate. Rust can use this to coerce the expression into another type.
fn read_guess() -> u32 {
    loop {
        let input = "42";
        let parsed: Result<u32, _> = input.parse();
        let guess: u32 = match parsed {
            Ok(value) => value,
            Err(_) => continue,
        };

        return guess;
    }
}

fn unwrap_demo() -> u32 {
    let maybe_value = Some(7u32);

    match maybe_value {
        Some(value) => value,
        None => panic!("This branch does not run for the example."),
    }
}

// A `str` value is a DST: its length is only known at runtime.
// We can use it only behind a pointer like `&str` or `Box<str>`.
fn pointer_example<T: ?Sized + std::fmt::Debug>(value: &T) {
    println!("Behind a pointer: {:?}", value);
}

trait Speaker {
    fn speak(&self);
}

struct Dog;

impl Speaker for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    // --- Newtype pattern: stronger type safety ---
    let millimeters = Millimeters(2500);
    let meters = Meters(2);
    println!("millimeters = {:?}", millimeters);
    println!("meters = {:?}", meters);
    print_distance(meters);

    // The next line would fail to compile because `Millimeters` and `Meters` are different types.
    // print_distance(millimeters);

    // --- Newtype pattern: hide a concrete collection behind a safer API ---
    let mut people = People::new();
    people.add_person(1, "Alice");
    people.add_person(2, "Bob");
    println!("Person 1: {:?}", people.get_name(1));
    println!("Person 2: {:?}", people.get_name(2));

    // --- Type aliases: help with repetition ---
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let thunk: Thunk = Box::new(|| println!("Hi from a thunk!"));
    takes_long_type(thunk);

    let returned_thunk = returns_long_type();
    returned_thunk();

    let _io_result: IoResult<()> = Ok(());

    // --- Never type (`!`) ---
    let guess = read_guess();
    println!("Read guess: {}", guess);

    let unwrapped = unwrap_demo();
    println!("Unwrapped value from Option: {}", unwrapped);

    // --- Dynamically sized types and `?Sized` ---
    // This line intentionally does not compile if used directly:
    // let bad: str = "Hello";
    // Rust requires a pointer behind the value.

    let text: &str = "Hello there!";
    pointer_example(text);

    let boxed: Box<str> = "This is a boxed str".to_string().into_boxed_str();
    pointer_example(&boxed);

    let dog: Box<dyn Speaker> = Box::new(Dog);
    dog.speak();

    // The rule is simple: DSTs must be behind a pointer, and `?Sized` allows generics to work
    // with types whose size is not known until runtime.
}
