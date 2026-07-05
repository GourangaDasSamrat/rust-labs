use std::rc::Rc;

// A singly linked list that can share its tail across multiple owners.
// Each `Cons` node owns an `Rc<List>` instead of a `Box<List>`.
// This enables multiple lists to point to the same shared tail.
#[allow(unused)]
enum List {
    Cons(i32, Rc<Self>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Create list `a` containing 5 -> 10 -> Nil.
    // We wrap the list in `Rc` so the same list can be shared.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // `b` and `c` share ownership of the tail list `a`.
    // `Rc::clone(&a)` only increments the reference count.
    // It does not deep-copy the entire list.
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));

    // Print the strong count after creating `b` and `c`.
    // `strong_count` shows how many `Rc` owners are alive.
    println!("count after creating a, b, c = {}", Rc::strong_count(&a));

    // Create an inner scope so we can drop one of the shared owners.
    {
        let _c_inner = Cons(4, Rc::clone(&a));
        println!("count after creating c_inner = {}", Rc::strong_count(&a));

        // `c_inner` goes out of scope at the end of this block,
        // and the strong count decreases automatically.
    }

    println!(
        "count after c_inner goes out of scope = {}",
        Rc::strong_count(&a)
    );

    // `a`, `b`, and the original `c` are still valid here.
    // When the last `Rc` owner goes out of scope, the shared tail is dropped.

    // Demonstrate that `Rc` is for shared read-only ownership in single-threaded code.
    println!("final count before main ends = {}", Rc::strong_count(&a));
}
