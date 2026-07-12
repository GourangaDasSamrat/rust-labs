use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    basic_mutex_single_thread();
    shared_mutex_multiple_threads();
    deadlock_demonstration();
}

#[allow(clippy::unwrap_used)]
fn basic_mutex_single_thread() {
    // Mutex (mutual exclusion) allows only one thread to access data at a time.
    // The lock() method acquires exclusive access to the protected data.
    // MutexGuard implements Deref to point at inner data.
    // MutexGuard also implements Drop to automatically release the lock when it goes out of scope.
    // This prevents forgetting to unlock manually.

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("Basic Mutex Result: m = {m:?}");
}

#[allow(clippy::unwrap_used, clippy::arithmetic_side_effects)]
fn shared_mutex_multiple_threads() {
    // Challenge: A single Mutex<T> can't be moved into multiple threads because ownership is not copyable.
    // Each thread's closure tries to move the counter, but it can only be moved once.
    //
    // Solution: Use Arc<T> (Atomic Reference Counting) for thread-safe multiple ownership.
    // Arc is similar to Rc but is designed for concurrent use - it's Send + Sync.
    // Rc is not thread-safe because its reference counting isn't atomic.
    // Arc uses atomic operations to ensure reference count updates are thread-safe.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for thread_id in 0..10 {
        // Clone the Arc, not the data inside. This increments the reference count.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;

            println!("Thread {} incremented counter to: {}", thread_id, *num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Shared Mutex Final Result: {}", *counter.lock().unwrap());
}

#[allow(clippy::unwrap_used)]
fn deadlock_demonstration() {
    // Deadlock Risk with Mutex: Occurs when threads wait for each other infinitely.
    // Classic scenario:
    //   - Thread A locks Resource 1, then tries to lock Resource 2
    //   - Thread B locks Resource 2, then tries to lock Resource 1
    //   - Both threads hold one lock and wait for the other = DEADLOCK
    //
    // Prevention Strategy: Always acquire locks in the same order across all threads.
    // This example shows correct lock ordering to prevent deadlock.

    let resource1 = Arc::new(Mutex::new(vec![1, 2, 3]));
    let resource2 = Arc::new(Mutex::new(vec![4, 5, 6]));

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        // Thread 1 acquires locks in order: resource1 -> resource2

        r1_clone.lock().unwrap().push(100);

        r2_clone.lock().unwrap().push(200);

        println!("Thread 1: Successfully modified both resources");
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        // Thread 2 also acquires locks in the same order: resource1 -> resource2
        // This consistent ordering prevents deadlock.

        r1_clone2.lock().unwrap().push(300);

        r2_clone2.lock().unwrap().push(400);

        println!("Thread 2: Successfully modified both resources");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Deadlock Prevention: No deadlock because both threads use same lock order");
    println!("Final resource1: {:?}", *resource1.lock().unwrap());
    println!("Final resource2: {:?}", *resource2.lock().unwrap());
}

// COMPARISON: RefCell/Rc vs Mutex/Arc
//
// RefCell<Rc<T>>             vs    Mutex<Arc<T>>
//
// Similarities:
//   - Both provide interior mutability (mutate inside an immutable container)
//   - Both use reference counting for multiple ownership
//   - Both allow sharing without full ownership transfer
//
// Differences:
//   - RefCell: Uses runtime borrow checking, single-threaded only
//   - Mutex: Thread-safe, uses locks for exclusive access
//   - Rc: NOT Send/Sync, cannot be shared across threads
//   - Arc: IS Send + Sync, designed specifically for thread-safe sharing
//
// Risks to Avoid:
//   - RefCell Risk: Reference cycles create memory leaks (two Rc's pointing to each other)
//   - Mutex Risk: Deadlocks from incorrect lock ordering (threads waiting on each other)
