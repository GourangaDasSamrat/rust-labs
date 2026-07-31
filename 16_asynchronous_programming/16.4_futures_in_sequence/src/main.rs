// ============================================================
// Streams are the async version of Iterators.
// - Iterator::next()               -> synchronous, returns Option<T>
// - Stream (via StreamExt::next()) -> asynchronous, must be awaited
// The Stream trait itself is low-level (Iterator + Future combined).
// StreamExt is the "extension" trait that gives us the ergonomic
// methods we actually use (next, map, filter, etc.), similar to
// how Iterator has helper methods built on top of just `next`.
// ============================================================

use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        // --------------------------------------------------
        // Example 1: Turning a plain iterator into a stream
        // --------------------------------------------------
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Ordinary synchronous iterator chain — nothing async yet.
        let iter = values.iter().map(|n| n * 2);

        // Convert the synchronous iterator into an async Stream.
        // LEARNING NOTE: any Iterator can become a Stream this way,
        // because Rust's stream design deliberately mirrors Iterator.
        let mut stream = trpl::stream_from_iter(iter);

        // `while let Some(x) = stream.next().await` is the stream
        // equivalent of `while let Some(x) = iter.next()`.
        // The `.await` is the only structural difference —
        // it suspends this task until the next item is ready,
        // instead of blocking or returning immediately.
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }

        // --------------------------------------------------
        // Example 2: Why StreamExt matters — chaining methods
        // --------------------------------------------------
        // LEARNING NOTE: because StreamExt mimics Iterator's API,
        // once it's in scope you get iterator-like combinators,
        // but each step in the pipeline can be asynchronous.
        let iter2 = values.iter().copied();
        let stream2 = trpl::stream_from_iter(iter2);

        let mut evens = stream2
            .filter(|n| n % 2 == 0)   // sync predicate, still fine on a stream
            .map(|n| n * 10);         // transform each item as it arrives

        while let Some(value) = evens.next().await {
            println!("Even * 10: {value}");
        }

        // --------------------------------------------------
        // Example 3: A channel receiver is NOT a Stream by default
        // --------------------------------------------------
        // LEARNING NOTE: trpl::Receiver (built on top of
        // tokio::sync::mpsc::UnboundedReceiver) does not implement
        // the Stream trait on its own. It only exposes its own
        // async `recv()` method — that's how the book's earlier
        // "Message Passing" section uses it, with no StreamExt
        // needed at all.
        //
        // But if we *want* to treat channel messages as a Stream —
        // so we can reuse StreamExt combinators like `.map()`,
        // `.filter()`, `.throttle()` on top of them, the same way
        // we just did with `stream2` above — we need an adapter
        // that bridges "a type with recv()" to "a type with Stream".
        //
        // That adapter is `UnboundedReceiverStream` from the
        // `tokio-stream` crate:
        //
        //   Cargo.toml:
        //     tokio-stream = "0.1"
        //
        // It wraps the raw receiver and implements Stream for it,
        // which is exactly the trait bound StreamExt requires.
        use tokio_stream::wrappers::UnboundedReceiverStream;

        let (tx, rx) = trpl::channel();

        // Wrap the raw receiver so it implements Stream.
        // LEARNING NOTE: this is the same pattern as
        // `trpl::stream_from_iter()` from Example 1 — both are
        // adapters that take "something sequence-like" and give
        // back "something that implements Stream".
        //   Iterator        -> Stream  via stream_from_iter()
        //   mpsc::Receiver   -> Stream  via *ReceiverStream wrapper
        let mut rx_stream = UnboundedReceiverStream::new(rx);

        // Producer: sends a few values, then drops `tx`.
        // LEARNING NOTE: dropping the sender is what eventually
        // makes the stream yield None — same "end of sequence"
        // signal an iterator gives when it's exhausted.
        let tx_future = async move {
            for i in 1..=3 {
                tx.send(i).unwrap();
            }
        };

        // Consumer: now that rx_stream implements Stream, we can
        // use StreamExt::next() on it just like any other stream,
        // and even chain combinators before consuming it.
        let rx_future = async {
            while let Some(msg) = rx_stream.next().await {
                println!("Received (as stream): {msg}");
            }
        };

        // LEARNING NOTE: trpl::join runs both futures concurrently
        // on the same task, polling each as it makes progress —
        // this is what lets the producer send while the consumer
        // is simultaneously awaiting new values.
        trpl::join(tx_future, rx_future).await;
    });
}

// ============================================================
// KEY TAKEAWAYS
// 1. Stream    = async analog of Iterator (combines Iterator + Future).
// 2. StreamExt = the trait that actually gives you `.next()` and
//    other combinators (map, filter, etc.) — must be imported.
// 3. Not everything "sequence-like" implements Stream automatically:
//      - trpl::stream_from_iter() adapts a plain Iterator.
//      - UnboundedReceiverStream adapts a channel Receiver.
//    Both exist because the underlying types (Iterator, Receiver)
//    predate/are separate from the Stream trait itself.
// 4. Because streams are futures, they compose with everything else
//    in the async ecosystem: timeouts, batching, throttling, join!, etc.
// 5. Rule of thumb when you hit "method `next` not satisfied" errors:
//    ask whether the type actually implements Stream, or whether it
//    needs a wrapper/adapter crate (like tokio-stream) to bridge it.
// ============================================================
