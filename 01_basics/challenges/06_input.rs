// Topic: User Input
// Fix the error handling.

use std::io;

fn main() {
    println!("Please enter some text:");

    let mut input = String::new();

    // Error: read_line returns a Result. We must handle it or unwrap it.
    // The compiler warns strictly about unused Results.
    io::stdin().read_line(&mut input);

    println!("You typed: {}", input.trim());
}
