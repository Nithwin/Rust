/*
    topic: Slices

    DEEP DIVE THEORY:
    =================
    A **Slice** is a reference to a contiguous sequence of elements in a collection
    rather than the whole collection. Slices let you access a subset of data safely
    without copying.

    String Slices (`&str`):
    - A reference to a part of a String.
    - Internally: A pointer to the start byte + a length.
    - Example: `&s[0..5]`.

    Array Slices (`&[T]`):
    - A reference to a part of an Array.
    - Example: `&a[1..3]`.

    Caveat: String indices must fall on valid UTF-8 character boundaries.
*/

fn main() {
    // 1. String Slices
    let s = String::from("Hello World");

    let hello = &s[0..5]; // Index 0 to 4
    let world = &s[6..11]; // Index 6 to 10

    // &str is the type of string literals too!
    let literal: &str = "Fixed String";

    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", world);

    // 2. Array Slices
    let a = [10, 20, 30, 40, 50];
    let slice = &a[1..4]; // [20, 30, 40]

    println!("Array Slice: {:?}", slice);

    // 3. String Function Parameters
    // Experienced Rustaceans use `&str` instead of `&String` for parameters
    // because it allows passing both String references AND string literals.
    greet("Reference to String");
    greet(literal);
}

fn greet(text: &str) {
    println!("Greeting: {}", text);
}

/*
    PRACTICE PROBLEM:
    =================
    Write a function `first_word(s: &str) -> &str` that returns the first word of a string.
    - Iterate over the bytes.
    - If you find a space (b' '), return the slice up to that index.
    - If no space is found, return the whole string.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: "Hello World"
    Output: "Hello"

    Input: "Rust"
    Output: "Rust"

    Input: "   " (Leading/Trailing spaces need trim, or handle empty logic)
    Output: "" (if purely space)
*/
