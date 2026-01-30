// Topic: Async
// Fix await usage.

use std::time::Duration;
use tokio::time::sleep;

async fn do_something() {
    sleep(Duration::from_millis(100)).await;
    println!("Done!");
}

#[tokio::main]
async fn main() {
    // Error: This future does nothing unless awaited.
    do_something();
}
