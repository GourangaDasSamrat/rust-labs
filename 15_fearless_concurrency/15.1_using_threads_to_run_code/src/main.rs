use std::thread;
use std::time::Duration;

fn main() {
    println!("Example 1: Basic Thread Spawning with spawn()\n");
    example_1_basic_spawn();

    println!("\nExample 2: Using JoinHandle with join()\n");
    example_2_join_at_end();

    println!("\nExample 3: Calling join() Early (Sequential)\n");
    example_3_join_early();

    println!("\nExample 4: Using move Closures with Data\n");
    example_4_move_closures();

    println!("\nExample 5: Multiple Threads\n");
    example_5_multiple_threads();
}

// EXAMPLE 1: Basic Thread Spawning
// Learning Note: thread::spawn creates a new thread that runs concurrently
// with the main thread. The order of output is NOT guaranteed because both
// threads compete for CPU time. When main() exits, all spawned threads are
// immediately shut down, regardless of whether they finished executing.
//
fn example_1_basic_spawn() {
    println!("Creating a spawned thread...\n");

    // thread::spawn takes a closure containing the code to run in the new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("  [SPAWNED] hi number {i} from the spawned thread!");
            // thread::sleep pauses execution to allow the OS scheduler to
            // switch between threads, demonstrating concurrent execution
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread continues executing immediately (doesn't wait for spawned thread)
    for i in 1..5 {
        println!("  [MAIN]    hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // ⚠️ IMPORTANT: The main thread exits here, and the spawned thread is shut down!
    // You might not see all the spawned thread's output because main() exits quickly.
    println!("\n  => Notice: Spawned thread output may be incomplete because");
    println!("     the main thread shut it down when main() exited.");

    // Small sleep to demonstrate the problem
    thread::sleep(Duration::from_millis(50));
}

// EXAMPLE 2: Using JoinHandle to Wait for Thread Completion
// Learning Note: thread::spawn returns a JoinHandle, which is an "owned handle"
// to the spawned thread. Calling join() on the handle BLOCKS the current thread
// until the spawned thread finishes. This guarantees the spawned thread will
// complete before main() exits.
//
// Blocking means: the thread calling join() stops executing and waits.
//
fn example_2_join_at_end() {
    println!("Creating a spawned thread and storing its JoinHandle...\n");

    // Store the JoinHandle returned by thread::spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("  [SPAWNED] hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread runs its loop (threads execute concurrently)
    for i in 1..5 {
        println!("  [MAIN]    hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // ✓ IMPORTANT: Call join() to wait for the spawned thread to finish
    // handle.join() returns Result<(), Box<dyn Any + Send + 'static>>
    // We call unwrap() to panic if the thread panicked
    println!("\n  => Main thread calling join()... (will wait for spawned thread)\n");
    handle.join().unwrap();

    println!("\n  => Spawned thread has finished! Main thread can now exit.");
}

// EXAMPLE 3: Calling join() Early (Sequential Execution)
// Learning Note: When you call join() IMMEDIATELY after spawn(), the threads
// don't run concurrently. The main thread blocks and waits for the spawned
// thread to finish before continuing its own loop.
//
// Key Insight: Placement of join() determines whether threads run concurrently
// or sequentially. This demonstrates that even with multiple threads, you can
// control the execution order.
//
fn example_3_join_early() {
    println!("Calling join() immediately after spawn (sequential execution)...\n");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("  [SPAWNED] hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ✓ Call join() BEFORE the main loop (not after!)
    // This blocks the main thread until the spawned thread finishes
    println!("  => Main thread calling join() IMMEDIATELY...\n");
    handle.join().unwrap();

    // Now the main loop runs (after spawned thread finishes)
    println!("\n  => Spawned thread finished. Now running main loop:\n");
    for i in 1..5 {
        println!("  [MAIN]    hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    println!("\n  => Notice: There's no interleaving. Threads executed sequentially!");
}

// EXAMPLE 4: Using `move` Closures with Thread Data
// Learning Note: When a closure captures variables from its environment,
// Rust normally borrows them. However, thread::spawn requires the closure
// to have a 'static lifetime (valid for the entire program's lifetime).
//
// If we just borrow a variable, the main thread might drop it before the
// spawned thread uses it, causing a use-after-free bug!
//
// Solution: Use the `move` keyword to transfer OWNERSHIP of the variable
// into the closure. This guarantees the variable stays valid in the new thread.
//
fn example_4_move_closures() {
    println!("Using `move` to transfer ownership to the spawned thread...\n");

    let v = vec![1, 2, 3];
    println!("  Created vector in main: {v:?}");

    // Without `move`, this would fail to compile because Rust can't guarantee
    // the reference to `v` stays valid while the thread runs.
    // The `move` keyword forces the closure to take ownership of `v`.
    let handle = thread::spawn(move || {
        println!("  [SPAWNED] Received vector: {v:?}");
        for value in &v {
            println!("  [SPAWNED] Value: {value}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    // After moving v into the thread, the main thread cannot use v anymore!
    // Uncommenting the next line would cause a compile error:
    // println!("  Vector in main: {:?}", v); //  ERROR: v has been moved!

    handle.join().unwrap();

    println!("\n  => The spawned thread took ownership of the vector.");
    println!("     Main thread cannot use v after moving it!");
}

// EXAMPLE 5: Multiple Threads
// Learning Note: You can spawn multiple threads and collect their JoinHandles.
// Then wait for all of them to finish by calling join() on each handle.
// This pattern is useful for creating a thread pool or parallel work.
//
fn example_5_multiple_threads() {
    println!("Spawning multiple threads...\n");

    // Vec to store the JoinHandles
    let mut handles = vec![];

    // Spawn 3 threads
    for thread_id in 1..=3 {
        let handle = thread::spawn(move || {
            for iteration in 1..=3 {
                println!("  [Thread {thread_id}] Iteration {iteration}");
                thread::sleep(Duration::from_millis(5));
            }
        });
        handles.push(handle);
    }

    println!("  All threads spawned. Main thread continuing...\n");

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n  => All spawned threads have finished!");
}

// KEY TAKEAWAYS AND IMPORTANT POINTS

//
// 1. THREAD EXECUTION ORDER IS NON-DETERMINISTIC
//    - You cannot rely on a specific execution order without synchronization
//    - The OS scheduler decides when threads run
//    - Output might differ between runs of the same program
//
// 2. MAIN THREAD SHUTDOWN
//    - When main() exits, ALL spawned threads are immediately shut down
//    - Use JoinHandle::join() to wait for threads before exiting
//    - join() blocks the calling thread until the spawned thread finishes
//
// 3. THE `move` KEYWORD IS ESSENTIAL FOR THREADS
//    - Closures passed to thread::spawn must own their captured variables
//    - Use `move || { ... }` to transfer ownership into the thread
//    - This prevents use-after-free bugs from the main thread dropping data
//
// 4. THREAD SAFETY CHALLENGES
//    - Race Conditions: Multiple threads accessing the same data in inconsistent order
//    - Deadlocks: Two threads waiting for each other, both blocked forever
//    - These require careful synchronization using Mutex, Arc, channels, etc.
//
// 5. RUST'S SAFETY GUARANTEES
//    - Rust's ownership system prevents many threading bugs at compile time
//    - You must explicitly use `move` to transfer data (prevents accidental aliasing)
//    - Type system enforces Send and Sync traits for thread safety
//
