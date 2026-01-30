/*
    topic: Operators
    
    DEEP DIVE THEORY:
    =================
    Operators are the verbs of the language.
    
    1. **Arithmetic**: `+`, `-`, `*`, `/`, `%` (Remainder).
       - Integer Division uses truncation (rounds down). `5 / 2` is `2`.
    
    2. **Logical**: `&&` (AND), `||` (OR), `!` (NOT).
       - Short-circuiting: In `A && B`, if A is false, B is never evaluated.
    
    3. **Comparison**: `==`, `!=`, `>`, `<`, `>=`, `<=`.
    
    4. **Type Casting**: `as`.
       - `let b = a as u32;`
    
    Side Effects:
    Rust does NOT support increment/decrement operators like `++` or `--` from C/C++.
    Why? Because `x++` (return old value, then increment) and `++x` (increment, then return) 
    are often confusing and lead to subtle bugs. Rust forces `x += 1`.
*/

fn main() {
    // 1. Arithmetic
    let a = 10;
    let b = 3;
    
    println!("Sum: {}", a + b);
    println!("Difference: {}", a - b);
    println!("Product: {}", a * b);
    println!("Division (Int): {}", a / b); // Truncates towards zero
    println!("Remainder: {}", a % b); 
    
    // Float division
    println!("Division (Float): {}", a as f64 / b as f64);

    // 2. Logic
    let has_license = true;
    let is_sober = true;
    
    if has_license && is_sober {
        println!("You can drive.");
    }

    // 3. Comparison
    let x = 5;
    let y = 10;
    
    println!("Is x greater than y? {}", x > y);
}

/*
    PRACTICE PROBLEM:
    =================
    Write a leap year checker.
    A year is a leap year if:
    - It is divisible by 4
    - EXCEPT if it is divisible by 100 (unless it is also divisible by 400).
    
    Logic: (year % 4 == 0) && (!(year % 100 == 0) || (year % 400 == 0))

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: 2024
    Output: "2024 is leap: true"

    Input: 1900
    Output: "1900 is leap: false"

    Input: 2000
    Output: "2000 is leap: true"
*/
