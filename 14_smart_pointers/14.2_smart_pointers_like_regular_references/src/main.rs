// Examples from "Treating Smart Pointers Like Regular References"

use std::ops::Deref;

// Learning note: A reference is a pointer to a value. Use `*` to follow it.
fn example_reference() {
    let x = 5;
    let y = &x; // `y` is a reference to `x`

    // `x` is an `i32`, `y` is `&i32`. To compare the inner value, dereference `y`.
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Learning note: `Box<T>` stores a value on the heap and implements `Deref`.
fn example_box() {
    let x = 5;
    let y = Box::new(x); // `y` is a Box<i32>

    // Box implements `Deref`, so `*y` follows the pointer to the inner value.
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Learning note: A tuple struct with one element behaves like a wrapper type.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    const fn new(x: T) -> Self {
        Self(x)
    }
}

// The following commented code shows what *would* fail if `Deref` is not
// implemented. We keep it commented out so the file compiles.
/*
fn example_mybox_without_deref_fails() {
    let x = 5;
    let y = MyBox::new(x);

    // Learning note: This line would fail to compile because `MyBox<T>` does
    // not implement `Deref` yet. The compiler error would say "type `MyBox` cannot be dereferenced".
    // assert_eq!(5, *y);
}
*/

// Learning note: Implementing `Deref` lets `*` call `deref()` behind the scenes.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Now `MyBox` can be used like a reference with `*`.
fn example_mybox_with_deref() {
    let x = 5;
    let y = MyBox::new(x);

    // Behind the scenes `*y` becomes `*(y.deref())`.
    assert_eq!(5, *y);
}

// Learning note: `&String` can be coerced to `&str` because `String: Deref<Target=str>`.
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Learning note: Deref coercion can convert `&MyBox<String>` -> `&String` -> `&str`.
fn example_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));

    // This works because `MyBox<String>` implements `Deref<Target=String>`,
    // and `String` implements `Deref<Target=str>`. The compiler inserts calls
    // to `deref()` as needed to match the parameter type `&str`.
    hello(&m);

    // Equivalent explicit code without coercion (harder to read):
    // hello(&(*m)[..]);
}

fn main() {
    example_reference();
    example_box();
    example_mybox_with_deref();
    example_deref_coercion();

    println!("All examples ran successfully.");
}
