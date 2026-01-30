/*
    topic: Variables and Mutability

    DEEP DIVE THEORY:
    =================
    In most languages (like Python or JavaScript), variables are mutable by default. You create `x = 5`,
    and later you can change it to `x = 6`. Rust is different. In Rust, variables are **immutable by default**.

    Why?
    1. **Safety**: If a variable is immutable, you don't have to worry about it changing unexpectedly
       in a complex multi-threaded program.
    2. **Optimization**: The compiler can make better decisions knowing a value won't change.

    Shadowing:
    Rust allows you to declare a new variable with the *same name* as a previous one. This is called "Shadowing".
    Unlike mutation, shadowing effectively creates a fresh variable. This allows you to:
    - Change the **type** of the variable (e.g., String "42" -> Integer 42).
    - Perform a transformation and prevent the use of the old, unprocessed value.

    Constants:
    Constants (`const`) are different from immutable variables. They are evaluated at compile-time
    and their memory is often inlined directly where they are used. They MUST have a type annotation.
*/

fn main() {
    // 1. Immutable Variable
    // Trying to assign to this later will cause a compile-time error.
    let safety_score = 95;
    println!("Initial Safety Score: {}", safety_score);
    // safety_score = 100; // ERROR!

    // 2. Mutable Variable
    // The `mut` keyword opts-in to mutability.
    let mut fuel_level = 100;
    println!("Fuel Level: {}", fuel_level);
    fuel_level = 80;
    println!("Fuel Level after burn: {}", fuel_level);

    // 3. Shadowing
    let data = "42"; // data is type &str (string slice)
    println!("Data (String): {}", data);

    let data = data.parse::<i32>().unwrap(); // data is now type i32 (integer)
    println!("Data (Integer): {}", data + 10); // We can do math now!

    // 4. Constants
    // Convention is SCREAMING_SNAKE_CASE.
    const MAX_SPEED: u32 = 299_792_458; // Speed of light in m/s
    println!("Max Speed: {}", MAX_SPEED);
}

/*
    PRACTICE PROBLEM:
    =================
    You are writing a temperature converter system.
    1. Define a constant `FREEZING_POINT_F` (32.0).
    2. Create a mutable variable `temp_f` starting at 32.0.
    3. Convert it to Celsius using the formula: (F - 32) * 5/9.
    4. Shadow `temp_f` with a new value (e.g., string "Freezing") using an if/else expression (optional)
       or just create a new variable to print the status.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input (Code): temp_f = 32.0
    Output: "Temperature in C: 0.0"

    Input (Code): temp_f = 212.0
    Output: "Temperature in C: 100.0"

    Input (Code): Shadowing example
    Output: "Status: Freezing"
*/
