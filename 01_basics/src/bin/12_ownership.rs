/*
    topic: Ownership (The Heart of Rust)
    
    DEEP DIVE THEORY:
    =================
    Ownership is Rust's most unique feature. It enables memory safety without a garbage collector (Java/Python) 
    and without manual memory management (C/C++).
    
    The Rules of Ownership:
    1. Each value in Rust has a variable that’s called its **owner**.
    2. There can only be **one owner** at a time.
    3. When the owner goes out of scope, the value will be **dropped** (freed).
    
    The Stack vs The Heap:
    - **Stack**: Fast, LIFO, Fixed size. Stores integers, booleans, chars.
    - **Heap**: Slower, arbitrary size. Stores Strings, Vectors.
    
    Move Semantics:
    When you assign a Heap value (like `String`) to another variable, ownership MOVES. 
    The old variable becomes invalid. This prevents "Double Free" errors where two variables try to free the same memory.
    
    Copy Semantics:
    Simple Stack types (integers) implement the `Copy` trait. Assignment copies the bits. Both variables remain valid.
*/

fn main() {
    // 1. Move (Heap)
    let s1 = String::from("hello");
    let s2 = s1; // OWNERSHIP MOVED TO s2!
    
    // println!("{}", s1); // ERROR: value borrowed here after move
    println!("s2: {}", s2); // Valid

    // 2. Clone (Deep Copy)
    // If we want both, we must explicitly clone the heap data. expensive!
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
    
    // 3. Copy (Stack)
    let x = 5;
    let y = x; // x is COPIED to y
    println!("x: {}, y: {}", x, y); // Both valid because i32 implements Copy
    
    // 4. Ownership and Functions
    takes_ownership(s3); // s3 is MOVED into the function
    // println!("{}", s3); // Error! s3 is gone.

    let z = 10;
    makes_copy(z); // z is COPIED
    println!("{}", z); // Valid
} // s2 goes out of scope and is dropped. s1 was already moved.

fn takes_ownership(some_string: String) {
    println!("Received: {}", some_string);
} // some_string goes out of scope and 'drop' is called. Memory freed.

fn makes_copy(some_integer: i32) {
    println!("Received: {}", some_integer);
}

/*
    PRACTICE PROBLEM:
    =================
    1. Create a function `concat_strings(s1: String, s2: String) -> String`.
    2. It should take ownership of two strings, combine them, and return the new string.
    3. In `main`, create two strings, call the function, and print the result.
    4. Verify that trying to use the original strings after the call fails.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: s1 = "Hello", s2 = "World"
    Output: "HelloWorld"
*/
