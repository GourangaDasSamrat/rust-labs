use interior_mutability_pattern::{LimitTracker, Messenger};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    println!("Learning RefCell and interior mutability pattern examples");

    runtime_borrow_example();
    shared_owner_example();
    limit_tracker_example();
}

fn runtime_borrow_example() {
    // RefCell checks borrow rules at runtime.
    let cell = RefCell::new(vec![1, 2, 3]);

    {
        let borrow1 = cell.borrow();
        let borrow2 = cell.borrow();
        println!("immutable borrow 1 = {borrow1:?}");
        println!("immutable borrow 2 = {borrow2:?}");
    }

    {
        let mut borrow_mut = cell.borrow_mut();
        borrow_mut.push(4);
        println!("mutable borrow after push = {borrow_mut:?}");
    }

    println!("final value = {:?}", cell.borrow());
}

fn shared_owner_example() {
    // Rc<RefCell<T>> gives multiple owners shared mutable ownership.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

fn limit_tracker_example() {
    println!("\nLimitTracker example using lib.rs");

    struct ConsoleMessenger;

    impl Messenger for ConsoleMessenger {
        fn send(&self, msg: &str) {
            println!("Messenger send: {msg}");
        }
    }

    let messenger = ConsoleMessenger;
    let mut tracker = LimitTracker::new(&messenger, 100);

    tracker.set_value(80);
}
