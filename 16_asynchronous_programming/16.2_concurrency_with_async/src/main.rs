use async_std::channel;
use async_std::task::{sleep, spawn};
use std::time::Duration;

#[async_std::main]
async fn main() {
    // spawn_task-like example (await handle to run to completion)
    let h = spawn(async {
        for i in 1..10 {
            println!("hi number {} from the first task!", i);
            sleep(Duration::from_millis(500)).await;
        }
    });

    for i in 1..5 {
        println!("hi number {} from the second task!", i);
        sleep(Duration::from_millis(500)).await;
    }

    h.await;

    // join two futures fairly
    let fut1 = async {
        for i in 1..10 {
            println!("join fut1 {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("join fut2 {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    };

    futures::join!(fut1, fut2);

    // async channel: single send/recv
    let (tx, rx) = channel::unbounded::<String>();
    tx.send(String::from("hi")).await.unwrap();
    let received = rx.recv().await.unwrap();
    println!("received '{}'", received);

    // multiple producers + receiver
    let (tx, rx) = channel::unbounded::<String>();
    let tx1 = tx.clone();

    let tx1_fut = async move {
        let vals = ["hi", "from", "the", "future"];
        for v in vals {
            tx1.send(v.to_string()).await.unwrap();
            sleep(Duration::from_millis(500)).await;
        }
    };

    let tx_fut = async move {
        let vals = ["more", "messages", "for", "you"];
        for v in vals {
            tx.send(v.to_string()).await.unwrap();
            sleep(Duration::from_millis(1500)).await;
        }
    };

    let rx_fut = async {
        while let Ok(value) = rx.recv().await {
            println!("received '{}'", value);
        }
    };

    futures::join!(tx1_fut, tx_fut, rx_fut);
}
