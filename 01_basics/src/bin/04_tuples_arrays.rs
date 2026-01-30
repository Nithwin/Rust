/*
    topic: Compound Types (Tuples & Arrays)

    DEEP DIVE THEORY:
    =================
    Compound types can group multiple values into one type. Rust has two primitives:

    1. **Tuples**:
       - Fixed length: Once declared, they cannot grow or shrink.
       - Heterogeneous: Can contain different types (e.g., `(i32, f64, u8)`).
       - Stored on the Stack: Because size is known at compile time.
       - Access: Dot notation (`tup.0`, `tup.1`).
       - Destructuring: `let (x, y, z) = tup;` breaks it apart.

    2. **Arrays**:
       - Fixed length: Specified at compile time `[Type; Length]`.
       - Homogeneous: MUST contain the same type.
       - Stored on the Stack: Very fast access.
       - Access: Bracket notation (`arr[0]`).
       - Bounds Checking: Rust checks array access at runtime (panic) and compile time (error)
         to prevent "Buffer Overflow" attacks common in C/C++.

    Note: If you need a growable list, use a `Vector` (discussed later). Arrays are for fixed data
    like "Days of the Week" or "RGB Values".
*/

fn main() {
    // 1. Tuples
    let hero_stats: (String, i32, bool) = (String::from("Thor"), 1500, true);

    // Destructuring (Unpacking)
    let (name, power, is_god) = hero_stats;
    println!("Hero: {}, Power: {}, Is God: {}", name, power, is_god);

    // Dot Notation
    // We have to use the tuple directly if we didn't destructure or if we want to access fields.
    // Note: We can't access `hero_stats.0` easily here because `name` took ownership of the String!
    // Let's make a simple copy-type tuple.
    let coords = (10, 20);
    println!("X: {}, Y: {}", coords.0, coords.1);

    // 2. Arrays
    let months = ["Jan", "Feb", "Mar", "Apr"]; // Type: [&str; 4]

    // Repeated Value Initialization
    let buffer = [0u8; 5]; // [0, 0, 0, 0, 0]

    println!("First Month: {}", months[0]);
    println!("Buffer: {:?}", buffer);

    // 3. Slices (Preview)
    // A slice is a view into an array.
    let q1 = &months[0..3]; // ["Jan", "Feb", "Mar"]
    println!("Q1 Months: {:?}", q1);
}

/*
    PRACTICE PROBLEM:
    =================
    1. Define a tuple `rgb` representing the color Red (255, 0, 0).
    2. Define an array `messy_readings` with values `[10.5, 20.0, 0.0, 50.2]`.
    3. Calculate the average of the readings.
    4. Print both the color and the average.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input (Code): rgb = (255, 0, 0)
    Output: "Color: (255, 0, 0)"

    Input (Code): readings = [10.5, 20.0, 0.0, 50.2]
    Output: "Average Reading: 20.175"
*/
