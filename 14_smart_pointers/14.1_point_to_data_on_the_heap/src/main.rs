// Examples for "Using `Box` to Point to Data on the Heap"
// from The Rust Programming Language book, Chapter 15.1.
//
// A `Box<T>` stores its contents on the heap, while the box
// itself is a fixed-size pointer stored on the stack.
// When the box goes out of scope, both the pointer and the heap
// allocation are cleaned up automatically.

enum List {
    Cons(i32, Box<Self>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Store a single `i32` value on the heap using `Box::new`.
    // The value `5` is allocated on the heap, while `b` is the
    // stack value that owns the heap allocation.
    let b = Box::new(5);
    println!("b = {b}");

    // A recursive type like `List` cannot hold itself directly:
    // `Cons(i32, List)` would have infinite size.
    // `Box<List>` breaks the recursion with indirection.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // We can traverse the list and collect its values for display.
    println!("list values = {:?}", list.values());
}

impl List {
    fn values(&self) -> Vec<i32> {
        let mut values = Vec::new();
        self.push_values(&mut values);
        values
    }

    fn push_values(&self, values: &mut Vec<i32>) {
        match self {
            Cons(value, next) => {
                values.push(*value);
                next.push_values(values);
            }
            Nil => {}
        }
    }
}
