// Examples and learning notes about yielding control and building async
// abstractions in Rust. These examples use the `futures` crate's
// primitives so we don't require a full async runtime like Tokio.
//
// Learning notes are provided as comments inline; the code is runnable with
// `cargo run` from this workspace after fetching dependencies.

use futures::{
    channel::oneshot,
    executor::block_on,
    future::{Either, select},
};
use std::{future::Future, thread, time::Duration};

// A blocking helper that simulates CPU-bound or blocking work.
// Note: this uses `thread::sleep` so it will block the current thread.
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{}' ran for {}ms", name, ms);
}

// A small cooperative-yield helper that yields once by waiting on a oneshot
// sent from a background thread. This is a lightweight stand-in for a
// runtime-provided `yield_now()` when no async runtime is available.
async fn yield_now_task() {
    let (tx, rx) = oneshot::channel::<()>();
    thread::spawn(move || {
        // send immediately to allow the awaiting task to resume later
        let _ = tx.send(());
    });
    let _ = rx.await;
}

// Example 1: Starvation demonstration.
// Both futures do blocking work with no await points, so one will run to
// completion (or until it hits an await) before the other gets a chance.
async fn example_starvation() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        slow("a", 10);
        slow("a", 20);
        // No await between slow calls -> this future will block progress.
        println!("'a' finished.");
        "a done"
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        slow("b", 10);
        slow("b", 15);
        slow("b", 350);
        println!("'b' finished.");
        "b done"
    };

    // `select` races the two futures and returns when the first finishes.
    // Because `a` and `b` call blocking `slow`, you'll see no interleaving
    // between their `slow` calls in this example (starvation).
    futures::pin_mut!(a, b);

    match select(a, b).await {
        Either::Left((out, _)) => println!("select finished with: {}", out),
        Either::Right((out, _)) => println!("select finished with: {}", out),
    }
}

// Example 2: Yielding to the runtime to allow interleaving.
// Insert `yield_now().await` between expensive synchronous chunks so other
// tasks can make progress. This is cooperative multitasking.
async fn example_yielding() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        yield_now_task().await; // yield control back to the executor
        slow("a", 10);
        yield_now_task().await;
        slow("a", 20);
        yield_now_task().await;
        println!("'a' finished.");
        "a done"
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        yield_now_task().await;
        slow("b", 10);
        yield_now_task().await;
        slow("b", 15);
        yield_now_task().await;
        slow("b", 350);
        yield_now_task().await;
        println!("'b' finished.");
        "b done"
    };

    futures::pin_mut!(a, b);

    match select(a, b).await {
        Either::Left((out, _)) => println!("select finished with: {}", out),
        Either::Right((out, _)) => println!("select finished with: {}", out),
    }
}

// Building a `timeout` abstraction using `select` and a timer implemented
// with a background thread + oneshot channel.
//
// Notes:
// - We create a oneshot-based timer so the timer doesn't block the current
//   thread while sleeping. Instead a background thread sleeps and then sends
//   on the channel.
// - If the `future_to_try` itself blocks the thread (e.g., calls
//   `thread::sleep` directly inside), the executor cannot poll the timer and
//   the timeout cannot preempt the blocking work. This demonstrates why
//   cooperatively yielding or using non-blocking I/O/timers is important.
async fn timeout<F>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration>
where
    F: Future,
{
    // oneshot channel lets a background thread signal the timer completion
    let (tx, rx) = oneshot::channel::<()>();

    // Spawn a background thread to sleep and then notify via the channel.
    thread::spawn(move || {
        thread::sleep(max_time);
        // ignore send error; receiver might be dropped if future finished.
        let _ = tx.send(());
    });

    let timer = async {
        let _ = rx.await; // completes when the background thread sends
    };

    // `select` races the caller future and our timer. The first to finish
    // determines the result.
    futures::pin_mut!(future_to_try, timer);

    match select(future_to_try, timer).await {
        Either::Left((out, _timer_future)) => Ok(out),
        Either::Right((_, _running_future)) => Err(max_time),
    }
}

fn main() {
    // Run the examples sequentially. Use `block_on` from `futures` to
    // execute our async examples on the current thread.
    println!("-- example_starvation --");
    block_on(example_starvation());

    println!("\n-- example_yielding --");
    block_on(example_yielding());

    println!("\n-- timeout example --");
    // A slow future that blocks the thread for 5 seconds, simulating a
    // long-running synchronous operation inside an async context.
    let slow_future = async {
        thread::sleep(Duration::from_secs(5));
        "Finally finished"
    };

    // We set the timeout to 2 seconds. Because `slow_future` blocks the
    // executor thread, the timer cannot be polled and the timeout will NOT
    // interrupt the blocking work; this highlights why blocking inside
    // async tasks is problematic.
    match block_on(timeout(slow_future, Duration::from_secs(2))) {
        Ok(message) => println!("Succeeded with '{}'", message),
        Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
    }
}
