/*
    topic: Concurrency
    
    DEEP DIVE THEORY:
    =================
    Concurrency means parts of a program execute independently.
    Parallelism means they execute at the exact same time.
    Rust enables "Fearless Concurrency".
    
    1. **Threads**: `thread::spawn`.
       - Runs code in a new OS thread.
       - `join()` ensures the thread finishes before main exits.
       - `move` closure allows transferring ownership of data INTO the thread.
       
    2. **Message Passing**: Channels (`mpsc`).
       - "Do not communicate by sharing memory; share memory by communicating." (Go slogan, adopted by Rust).
       - `tx` (Transmitter) can be cloned (Multi-producer).
       - `rx` (Receiver) is single-consumer.
       
    3. **Shared State**: `Mutex<T>` (Mutual Exclusion).
       - Only one thread can access data at a time.
       - `lock()` blocks until access is granted.
       - Often wrapped in `Arc<T>` (Atomic Reference Counted) to share across threads.
*/

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    // 1. Threads
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Wait for thread to finish

    // 2. Channels
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // Error: val moved to channel
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 3. Shared State (Mutex)
    // Arc is like Rc, but thread-safe (Atomic).
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/*
    PRACTICE PROBLEM:
    =================
    Simulate a bank run.
    1. Create a `balance` wrapped in `Arc<Mutex<i32>>` with value 1000.
    2. Spawn 5 threads.
    3. Each thread should attempt to withdraw 200.
    4. Ensure the lock is acquired, value decremented, and lock released.
    5. Print final balance (should be 0).

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: Start 1000, 5 threads withdraw 200
    Output: Final Balance: 0
*/
