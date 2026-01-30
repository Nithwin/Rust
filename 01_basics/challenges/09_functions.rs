// Topic: Functions
// Fix the return value mismatch.

fn square(x: i32) -> i32 {
    // Error: The semicolon at the end makes this a Statement, dealing '()' (unit).
    // We promised to return 'i32'.
    // Remove the semicolon to make it an Expression!
    x * x;
}

fn main() {
    let res = square(5);
    println!("5 squared is {}", res);
}
