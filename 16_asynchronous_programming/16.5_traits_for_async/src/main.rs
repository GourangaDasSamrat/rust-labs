// ============================================================
// This file mirrors the chapter's three big ideas,
// in order:
//   1. The Future trait + poll/Poll — what `.await` compiles down to
//   2. Pin / Unpin — why moving futures in memory can be dangerous
//   3. The Stream trait + StreamExt — async Iterator
// ============================================================

use std::pin::{pin, Pin};
use std::future::Future;
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        section_1_future_and_poll().await;
        section_2_pin_and_unpin().await;
        section_3_stream_trait().await;
    });
}

// ============================================================
// SECTION 1: The Future trait
// ============================================================
// Future's real definition:
//
//   pub trait Future {
//       type Output;
//       fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//   }
//
// Key ideas:
// - `Output` is like Iterator's `Item` — the type you eventually get.
// - `poll` returns `Poll<T>`, shaped like Option but meaning
//   something different:
//     Ready(T)  -> future is done, here's the value
//     Pending   -> not done yet, "ask me again later"
// - `await` is sugar for a loop that calls `poll` repeatedly, but
//   WITHOUT blocking the thread. The async runtime decides when to
//   poll again — typically by parking the task until something
//   wakes it (e.g. a channel receives a message).
// - Important gotcha from the book: most futures should NOT be
//   polled again after returning Ready — many will panic if you do.
//   This mirrors how calling Iterator::next() again after None is
//   allowed but often just keeps returning None — except futures
//   are stricter about it.
async fn section_1_future_and_poll() {
    println!("--- Section 1: Future & poll ---");

    // we never call poll() by hand in ordinary code.
    // Writing `.await` is exactly equivalent (conceptually) to:
    //
    //   loop {
    //       match Pin::new(&mut fut).poll(cx) {
    //           Poll::Ready(value) => break value,
    //           Poll::Pending => { /* runtime parks this task, tries later */ }
    //       }
    //   }
    //
    // The compiler + runtime handle this loop for us so `.await`
    // never blocks the OS thread while waiting.
    let value = some_async_fn().await;
    println!("Got value via await: {value}");

    // this is exactly what happens with rx.recv().await
    // from the "Message Passing" section earlier in the chapter:
    // - Poll::Pending      -> no message yet, don't busy-loop, just wait
    // - Poll::Ready(Some(msg)) -> a message arrived, advance
    // - Poll::Ready(None)      -> channel closed, sender was dropped
}

async fn some_async_fn() -> i32 {
    42
}

// ============================================================
// SECTION 2: Pin and Unpin
// ============================================================
// Why does Pin exist at all?
//
// When the compiler turns an async block into a state machine, some
// state-machine variants can end up holding references INTO
// themselves — e.g. a local variable that's borrowed across an
// await point. This is called a "self-referential" type.
//
// If such a value gets MOVED in memory (e.g. pushed into a Vec),
// its internal reference would still point at the OLD memory
// address — now invalid, possibly reused for something else. This
// is exactly the kind of bug Rust's ownership/borrowing rules exist
// to prevent, but moves aren't normally tracked that way, so Rust
// needs a new tool: Pin.
//
// - Pin<P> (P = some pointer type: &mut T, Box<T>, etc.) guarantees
//   the *pointee* will never move in memory again, making internal
//   self-references safe.
// - Unpin is a marker trait (like Send/Sync — no methods, just a
//   compiler signal) meaning "this type has no self-references, so
//   it's always safe to move, even through a Pin wrapper."
// - Most ordinary types (String, Vec<T>, i32, bool...) implement
//   Unpin automatically, because they can never be self-referential.
// - Some compiler-generated async-block futures ARE self-referential
//   (if they borrow a local across an await point) and are !Unpin.
async fn section_2_pin_and_unpin() {
    println!("--- Section 2: Pin & Unpin ---");

    // --------------------------------------------------------
    // THE PROBLEM (Listing 17-23 style):
    // --------------------------------------------------------
    // this version would NOT compile:
    //
    //   let futures: Vec<Box<dyn Future<Output = ()>>> =
    //       vec![Box::new(fut_a()), Box::new(fut_b())];
    //   trpl::join_all(futures).await;
    //
    //   error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
    //
    // Why: `join_all`'s internal `JoinAll<F>` struct requires
    // `F: Future`. And `Box<T>` only implements `Future` when the
    // `T` inside is itself `Unpin`. A `dyn Future<Output = ()>`
    // trait object has no guarantee of being Unpin (the concrete
    // async-block type behind it might be self-referential), so
    // the bound isn't satisfied.

    // --------------------------------------------------------
    // THE FIX (Listing 17-24 style): pin each future FIRST with
    // the `pin!` macro, which produces a `Pin<&mut dyn Future<...>>`
    // instead of relying on `Box`.
    // --------------------------------------------------------
    // `pin!` pins the future on the STACK — no heap
    // allocation required (unlike `Box::pin`, which pins on the
    // heap). Either works; `pin!` is cheaper when you don't need
    // to move the pinned value out of the current scope.
    let fut_a = pin!(async {
        println!("fut_a running");
    });

    let fut_b = pin!(async {
        println!("fut_b running");
    });

    // Note the type change from `Box<dyn Future<Output = ()>>`
    // to `Pin<&mut dyn Future<Output = ()>>` — this is the whole fix.
    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut_a, fut_b];

    // Now `join_all` is satisfied, because `Pin<&mut T>` where
    // `T: Future` also implements `Future`, regardless of whether
    // the underlying future is Unpin or not.
    trpl::join_all(futures).await;

    // --------------------------------------------------------
    // CONTRAST: an Unpin type, like String, doesn't have this problem.
    // --------------------------------------------------------
    // String has no internal self-references, so
    // it implements Unpin automatically. That means even while
    // pinned, we can safely swap out its entire contents in memory
    // via `get_mut()` — there's no self-referential data to break.
    let mut s: Pin<&mut String> = pin!(String::from("hello"));
    *s.as_mut().get_mut() = String::from("goodbye");
    println!("s is now: {}", s);
}

// ============================================================
// SECTION 3: The Stream trait (and how StreamExt builds on it)
// ============================================================
// Stream is essentially "Future + Iterator" merged
// into one trait:
//
//   trait Stream {
//       type Item;
//       fn poll_next(
//           self: Pin<&mut Self>,
//           cx: &mut Context<'_>,
//       ) -> Poll<Option<Self::Item>>;
//   }
//
// - `Item`      -> like Iterator::Item — a stream can produce many
//                  items over time (unlike Future's single Output).
// - `poll_next` -> OUTER Poll = readiness, exactly like Future::poll
//                  INNER Option = "more items, or done?", exactly
//                  like Iterator::next
//
// Note: as of this writing Stream isn't in std yet — it lives in
// the `futures` crate ecosystem (and by extension in `trpl`), but
// its shape is expected to standardize eventually.
//
// We almost never call poll_next by hand. Instead, StreamExt gives
// us the ergonomic `.next().await`:
//
//   trait StreamExt: Stream {
//       async fn next(&mut self) -> Option<Self::Item>
//       where
//           Self: Unpin;
//       // ...plus other combinator methods (map, filter, etc.)
//   }
//
// notice the `Self: Unpin` bound on `next` — this is
// the Stream-world version of the exact same Pin/Unpin story from
// Section 2. It's why custom Stream implementations sometimes need
// wrapping/pinning before `.next()` will compile for them.
//
// StreamExt is automatically implemented for every
// type that implements Stream — same relationship as Iterator's
// required `next` method unlocking all its default methods (map,
// filter, take, etc.) for free.
async fn section_3_stream_trait() {
    println!("--- Section 3: Stream & StreamExt ---");

    let values = [1, 2, 3];

    // stream_from_iter adapts a plain, synchronous
    // Iterator into something that implements Stream — bridging
    // the "sequence" concept (Iterator) into the "sequence over
    // time" concept (Stream).
    let mut stream = trpl::stream_from_iter(values.iter().copied());

    // `.next()` here comes from StreamExt (not Stream directly) —
    // Stream itself only defines the lower-level `poll_next`.
    while let Some(v) = stream.next().await {
        println!("stream item: {v}");
    }
}

// ============================================================
// SUMMARY / KEY TAKEAWAYS
// 1. Future::poll returns Poll::{Ready(T), Pending}. `.await` is
//    sugar for a non-blocking poll loop; the runtime decides when
//    to re-poll instead of busy-waiting.
// 2. Pin<P> guarantees the pointee won't move in memory — needed
//    because async-block state machines can be self-referential
//    (they may hold references to their own local data across
//    await points).
// 3. Unpin is a marker trait meaning "safe to move even when
//    pinned." Ordinary types (String, Vec, i32...) get it for
//    free; some compiler-generated futures don't.
// 4. Fix for "cannot be unpinned" errors on collections of
//    futures: pin each one first with `pin!` (stack) or
//    `Box::pin` (heap), and change the container's type from
//    `Box<dyn Future<...>>` to `Pin<&mut dyn Future<...>>` (or
//    `Pin<Box<dyn Future<...>>>`).
// 5. Stream = Iterator + Future merged: `poll_next` returns
//    `Poll<Option<Item>>`. StreamExt supplies the friendly
//    `.next()` (plus other combinators), requiring `Self: Unpin`
//    under the hood — the same Pin story, just for streams.
// 6. Practical debugging rule: if the compiler mentions
//    "cannot find function" or "unused import" together, check
//    whether a function that USES that import got accidentally
//    deleted or left unfinished — the two errors are often linked.
// ============================================================