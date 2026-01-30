/*
    topic: Functions

    DEEP DIVE THEORY:
    =================
    Functions are the primary way code is decomposed in Rust.

    Parameters:
    - Must ALWAYS have type annotations.

    Return Values:
    - Arrow syntax `-> Type`.
    - Implicit Return: The last expression in a block (without a semicolon) is returned.
      `x + 1` returns the value.
      `x + 1;` returns `()` (Unit type / Void).

    Statements vs Expressions:
    - **Statement**: Performs an action, returns nothing (e.g., `let x = 6;`).
    - **Expression**: Evaluates to something (e.g., `6`, `x + 1`, `if ...`).
    - You cannot do `let x = (let y = 6);` in Rust.
*/

fn main() {
    print_labeled_measurement(5, 'h');

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// Parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Return Type
fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon! This is an expression.
    // x + 1; // Un-commenting this makes it return (), causing an error.
}

/*
    PRACTICE PROBLEM:
    =================
    Write a function `convert_temperature` that takes a value and a unit ('C' or 'F').
    - If 'C', convert to F: (val * 9/5) + 32
    - If 'F', convert to C: (val - 32) * 5/9
    - Return the converted value as f64.

    Note: Ensure you handle integer/float math correctly.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: convert_temperature(0.0, 'C')
    Output: 32.0

    Input: convert_temperature(32.0, 'F')
    Output: 0.0

    Input: convert_temperature(100.0, 'C')
    Output: 212.0
*/
