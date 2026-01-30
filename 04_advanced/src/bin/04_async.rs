/*
    Topic: Async / Await

    Rust's async model is unique because it is "zero-cost" and "runtime-agnostic".
    We need an external runtime (like `tokio`) to execute async code.

    Key Concepts:
    1. `async fn` and `.await`
    2. The Tokio Runtime
    3. Concurrency vs Parallelism
*/

use std::time::Duration;
use tokio::time::sleep;

// An async function returns a `Future`.
// nothing happens until this future is `.await`ed.
async fn do_work(id: u32, duration_ms: u64) {
    println!("Task {} starting...", id);
    sleep(Duration::from_millis(duration_ms)).await; // Non-blocking sleep
    println!("Task {} finished.", id);
}

#[tokio::main]
async fn main() {
    println!("--- Async Sequential ---");
    // These run one after another because we await them immediately
    do_work(1, 500).await;
    do_work(2, 500).await;

    println!("\n--- Async Concurrent ---");
    // tokio::join! waits for all futures to complete, running them concurrently
    let start = std::time::Instant::now();

    let t1 = do_work(3, 1000);
    let t2 = do_work(4, 1000);
    let t3 = do_work(5, 1000);

    tokio::join!(t1, t2, t3);

    let duration = start.elapsed();
    println!("Total time taken: {:?} (approx 1s, not 3s)", duration);
}
