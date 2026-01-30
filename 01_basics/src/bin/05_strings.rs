/*
    topic: Strings (The Complex Beast)

    DEEP DIVE THEORY:
    =================
    In strict terms, Rust has only one string type in the core language: `str` (string slice).
    The `String` type is actually a standard library feature, but it's used everywhere.

    Difference:
    1. **`String`**:
       - Heap-allocated.
       - Growable (can push/pop characters).
       - Mutable.
       - Owned (you are responsible for freeing it).

    2. **`&str` (String Slice)**:
       - A reference (pointer + length) to valid UTF-8 data.
       - Fixed size (can't grow).
       - Immutable.
       - Usually points to:
         - A string literal binary (static memory).
         - A part of a `String` (on the Heap).

    UTF-8:
    Rust strings are strictly UTF-8. This means you cannot index them by integer (`s[0]`)
    because some characters (like '🦀') take 4 bytes. `s[0]` would give you just the first byte,
    which is invalid on its own.
*/

fn main() {
    // 1. String Literals (&str)
    // Stored directly in the program binary. Fast. Immutable.
    let greeting: &str = "Hello, World!";

    // 2. Owned String (String)
    // Allocated on the Heap. Slower creation, but flexible.
    let mut name = String::from("Alice");

    // Appending
    name.push_str(" in Wonderland"); // push string slice
    name.push('!'); // push char

    println!("Greeting: {}", greeting);
    println!("Name: {}", name);

    // 3. Concatenation
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    // The + operator looks like: fn add(self, s: &str) -> String
    // s1 is MOVED (consumed). s2 and s3 are BORROWED.
    let game = s1 + "-" + &s2 + "-" + &s3;
    println!("Game: {}", game);
    // println!("{}", s1); // ERROR: s1 is gone!

    // 4. Formatting (The better way)
    // Does not take ownership. Handles memory allocation for you.
    let score = 100;
    let report = format!("Player {} has {} points", name, score);
    println!("{}", report);
}

/*
    PRACTICE PROBLEM:
    =================
    1. Create a String "Rust".
    2. Loop 3 times and append " is great" each time such that the string grows.
    3. Print the final length and capacity of the string.
    4. Create a formatting string that says "Final string: [content]".

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: Start = "Rust"
    Steps: Append " is great" x3
    Output: "Final: Rust is great is great is great"
    Output: "Length: 31"
*/
