// Advanced Functions and Closures
//
// - A function pointer is the type `fn` (lowercase f), and it is different from
//   the closure traits `Fn`, `FnMut`, and `FnOnce`.
// - Function pointers implement those closure traits, so they can be passed where
//   a closure is expected.
// - We can pass named functions as arguments, not only closures.
// - Closures are anonymous, unique types. Returning multiple different closures
//   with the same signature using `impl Fn` is not allowed because Rust creates
//   a distinct opaque type for each return site.
// - When we need a collection of heterogeneous closures with the same behavior,
//   we use `Box<dyn Fn(...)>`.

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Example from the book: a function pointer can be used as an argument to another
// function, just like a closure can.
fn function_pointer_example() {
    let answer = do_twice(add_one, 5);
    println!("function pointer result: {answer}");
}

// Example: using a closure and a named function with `Iterator::map`.
fn iterator_map_examples() {
    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings_with_closure: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    println!("closure map: {:?}", list_of_strings_with_closure);

    let list_of_strings_with_function: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("function map: {:?}", list_of_strings_with_function);
}

// Enum variant constructors are also initializer functions. They can be used as
// function pointers in places where a closure is expected.
fn enum_initializer_example() {
    let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("enum initializer statuses: {:?}", list_of_statuses);
}

// Learning note:
// If we try to return `impl Fn(i32) -> i32` from two different functions and put
// those return values in the same vector, Rust rejects it because each `impl Trait`
// return is a distinct opaque type.
//
// This is the compile-error pattern:
//
// let handlers = vec![returns_closure(), returns_initialized_closure(123)];
//
// The correct approach is to return a trait object, so all closures share the same
// concrete type erased behind `dyn Fn`.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn returning_closure_example() {
    let handlers: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        returns_closure(),
        returns_initialized_closure(123),
    ];

    for handler in handlers {
        let output = handler(5);
        println!("returned closure output: {output}");
    }
}

fn main() {
    function_pointer_example();
    iterator_map_examples();
    enum_initializer_example();
    returning_closure_example();
}
