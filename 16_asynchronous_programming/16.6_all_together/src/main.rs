use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        run_thread_and_async().await;
        run_async_jobs().await;
        run_cpu_bound_thread().await;
    });
}

async fn run_thread_and_async() {
    println!("--- thread + async channel ---");

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while let Some(message) = rx.recv().await {
        println!("message from thread: {message}");
    }
}

async fn run_async_jobs() {
    println!("--- async jobs with join ---");

    let fast = async {
        for i in 1..5 {
            println!("fast task {i}");
            trpl::sleep(Duration::from_millis(150)).await;
        }
    };

    let slow = async {
        for i in 1..4 {
            println!("slow task {i}");
            trpl::sleep(Duration::from_millis(300)).await;
        }
    };

    trpl::join(fast, slow).await;
}

async fn run_cpu_bound_thread() {
    println!("--- cpu-bound work in a thread ---");

    let handle = thread::spawn(|| {
        let total = count_primes(10_000);
        println!("prime count: {total}");
    });

    trpl::sleep(Duration::from_millis(100)).await;

    handle.join().unwrap();
}

fn count_primes(limit: u64) -> u64 {
    (2..=limit).filter(|&n| is_prime(n)).count() as u64
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut divisor = 2;
    while divisor * divisor <= n {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
    }

    true
}
