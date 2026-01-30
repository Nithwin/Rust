// Topic: Errors
// Fix the function signature to propagate errors.

use std::fs::File;

// Error: The '?' operator returns an Err early.
// This function needs to return a Result.
fn open_file() {
    let _f = File::open("hello.txt")?;
}

fn main() {
    match open_file() {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}
