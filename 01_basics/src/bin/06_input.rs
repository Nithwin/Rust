/*
    topic: User Input (IO)

    DEEP DIVE THEORY:
    =================
    Interacting with the user involves `std::io`.

    The Buffer:
    When you read input, you need a place to store it. In Rust, we use a mutable `String`.
    This string acts as a buffer that grows to fit whatever the user types.

    References:
    `read_line(&mut buffer)` takes a mutable reference. It needs to change the content of the buffer,
    but it doesn't need to take ownership of the buffer itself.

    Result Type:
    IO operations can fail (e.g., keyboard disconnected, stream closed).
    Rust captures this possibility in a `Result` type (`Ok` or `Err`).
    We must handle this result. `.expect()` is a quick way to say "If Err, crash with this message. If Ok, give me the value".
*/

use std::io;
use std::io::Write; // Needed for flush()

fn main() {
    print!("What is your name? ");
    // stdout is line-buffered. To make the prompt appear before input, we flush.
    io::stdout().flush().expect("Failed to flush");

    // 1. Create Buffer
    let mut name = String::new();

    // 2. Read Input
    // Returns `Result<usize>` (bytes read).
    let bytes_read = io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Read {} bytes.", bytes_read);

    // 3. Parse Input
    // Input usually contains the newline character `\n` or `\r\n`.
    // We MUST trim() it before using it or parsing usage.
    let name = name.trim();

    println!("Hello, {}!", name);

    // Number Example
    print!("Enter your age: ");
    io::stdout().flush().unwrap();

    let mut age_str = String::new();
    io::stdin().read_line(&mut age_str).unwrap();

    // Parse string to integer
    let age: u32 = age_str.trim().parse().expect("Please type a number!");

    println!("In 5 years you will be {}.", age + 5);
}

/*
    PRACTICE PROBLEM:
    =================
    Create a simple adding calculator.
    1. Ask user for "First number: ".
    2. Read and parse input A.
    3. Ask user for "Second number: ".
    4. Read and parse input B.
    5. Print "Sum: A + B".

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: "10", "20"
    Output: "Sum: 30"

    Input: "5", "abc"
    Output: CRASH (Panic) with "Please type a number!"
*/
