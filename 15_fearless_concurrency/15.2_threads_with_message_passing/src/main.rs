use std::{sync::mpsc, thread, time::Duration};

fn main() {
    create_channel_example();
    send_one_string_from_thread();
    send_multiple_strings_with_sleep();
    send_from_multiple_producers();
}

fn create_channel_example() {
    // A channel is created with a transmitter and a receiver.
    // Rust uses a type parameter for the values that will be sent.
    let (_tx, _rx) = mpsc::channel::<String>();
    // This example does not use the channel, but it shows the shape:
    // transmitter first, receiver second.
}

#[allow(clippy::unwrap_used)]
fn send_one_string_from_thread() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // tx.send takes ownership of `val`. After sending, `val` cannot be used.
        // println!("val is {}", val); // would fail to compile because `val` was moved.
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    handle.join().unwrap();
}

#[allow(clippy::unwrap_used)]
fn send_multiple_strings_with_sleep() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

#[allow(clippy::unwrap_used)]
fn send_from_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}
