// - creating a reference cycle with `Rc` + `RefCell` (memory leak)
// - preventing cycles using `Weak` references in a parent/child tree

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    // Each `Cons` holds an integer and a `RefCell`-wrapped `Rc` to another `List`.
    // The `RefCell` gives interior mutability so we can mutate which `Rc` it
    // points to at runtime. Using `Rc` allows shared ownership of list nodes.
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

impl List {
    // Return an optional reference to the `RefCell<Rc<List>>` stored in a node.
    // We return a shared reference to the inner `RefCell` so callers can borrow
    // or mutate the `Rc<List>` inside.
    const fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Cons(_, item) => Some(item),
            Self::Nil => None,
        }
    }
}

// Demonstrate creating a reference cycle that leaks memory.
fn run_list_cycle_example() {
    println!("-- List reference cycle example --");

    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}\n", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}\n", b.tail());

    // Create a cycle: make `a` point to `b`.
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}\n", Rc::strong_count(&a));

    // If we tried to traverse and print `a` now (following strong refs), we'd
    // recurse forever because `a` -> `b` -> `a`..., causing a stack overflow.
    // The cycle means the `Rc` strong counts never reach 0 and memory is leaked.
    // println!("a next item = {:?}", a.tail()); // would overflow
}

#[derive(Debug)]
struct Node {
    value: i32,
    // `parent` is a weak reference because a child should not own its parent.
    parent: RefCell<Weak<Self>>,
    // children are owned (strong `Rc`) so dropping a parent will drop children
    // when there are no other strong owners.
    children: RefCell<Vec<Rc<Self>>>,
}

// Demonstrate using `Weak` to avoid reference cycles in a tree structure.
fn run_tree_weak_example() {
    println!("-- Tree with Weak parent references example --");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}\n", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Set `leaf`'s parent to a weak reference to `branch`.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}\n",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // At this point `branch` has a strong_count >= 1 and leaf.parent is a weak ref.
        // When `branch` goes out of scope the strong count drops to 0 and branch is dropped.
    }

    // After `branch` is dropped, upgrading the weak reference returns `None`.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    // Run the examples in sequence. They are small, self-contained demos
    // illustrating the concepts from the book chapter.
    run_list_cycle_example();
    println!();
    run_tree_weak_example();
}
