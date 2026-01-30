/*
    topic: Error Handling
    
    DEEP DIVE THEORY:
    =================
    Rust separates errors into two categories:
    
    1. **Unrecoverable (`panic!`)**: 
       - The program enters a bugged state it cannot handle (e.g., Index out of bounds).
       - The thread usually crashes and unwinds the stack to clean up memory.
       
    2. **Recoverable (`Result<T, E>`)**:
       - The program can reasonably report and fix logic (e.g., File not found -> Create it).
       - You MUST handle the result. Ignoring a Result is a compiler warning/error.
       
    The ? Operator:
    - `match` is verbose.
    - `?` allows you to say "If result is Ok, give me value. If Err, return the Err from the function immediately".
    - Can only be used in functions that return `Result`.
*/

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 1. Panic (Uncomment to crash)
    // panic!("Crash and burn!");

    // 2. Result
    let f = File::open("hello.txt"); // Result<File, io::Error>

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            // We can even match on the specific error kind!
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("File not found. (Mocking creation...)");
                    // Mock file for demo
                    File::create("hello.txt").unwrap_or_else(|e| {
                        panic!("Problem creating the file: {:?}", e);
                    })
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
        },
    };
    
    // 3. Clean error propagation
    match read_username_from_file() {
        Ok(s) => println!("Username read: {}", s),
        Err(e) => println!("Error reading username: {}", e),
    }
}

// Function returning Result
fn read_username_from_file() -> Result<String, io::Error> {
    // The "Long" way
    // let mut f = File::open("hello.txt")?;
    
    // The "Short" way (Chaining)
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/*
    PRACTICE PROBLEM:
    =================
    Write a function `safe_divide(a: f64, b: f64) -> Result<f64, String>`.
    - If `b` is 0.0, return `Err(String::from("Cannot divide by zero"))`.
    - Otherwise, return `Ok(a / b)`.
    
    In main:
    - Call it with (10.0, 2.0) -> Print "Result: 5.0"
    - Call it with (10.0, 0.0) -> Print "Error: Cannot divide by zero"

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: 10.0, 2.0
    Output: Ok(5.0)

    Input: 10.0, 0.0
    Output: Err("Cannot divide by zero")
*/
