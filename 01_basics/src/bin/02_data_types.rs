/*
    topic: Data Types (Scalar)

    DEEP DIVE THEORY:
    =================
    Rust is a **statically typed** language, meaning it must know the types of all variables at compile time.
    However, the compiler is smart enough to infer types most of the time.

    Scalar Types represent a single value:
    1. **Integers**: Whole numbers.
       - Signed (`i`): Can be negative (i8, i16, i32, i64, i128, isize).
       - Unsigned (`u`): Only positive (u8, u16, u32, u64, u128, usize).
       - `i32` is the default. `isize`/`usize` depend on your CPU architecture (32-bit vs 64-bit).

    2. **Floating-Point**: Numbers with decimals.
       - `f32`: Single precision.
       - `f64`: Double precision (default, more precise).

    3. **Booleans**: Logical true or false. One byte in size.

    4. **Characters**:
       - In C, a `char` is 1 byte (ASCII).
       - In Rust, a `char` is **4 bytes** and represents a Unicode Scalar Value.
       - This means 'A', 'ñ', and '🚀' are all valid chars.

    Overflow:
    In debug mode, Rust checks for integer overflow and panics. In release mode, it wraps around (two's complement).
*/

fn main() {
    // 1. Integers
    let year: i32 = 2024;
    let byte: u8 = 255; // Max value for u8

    // 2. Floats
    let pi: f64 = 3.1415926535; // Default is f64
    let score: f32 = 4.5;

    // 3. Booleans
    let is_active: bool = true;
    let is_greater = 10 > 5; // Evaluates to true

    // 4. Characters
    let a_char = 'a';
    let emoji = '🦀'; // This is a 4-byte char!

    println!("Year: {}, Byte: {}", year, byte);
    println!("Pi: {}, Score: {}", pi, score);
    println!("Active: {}, Greater: {}", is_active, is_greater);
    println!("Char: {}, Emoji: {}", a_char, emoji);

    // 5. Explicit Type Conversion (Casting)
    // Rust does NOT do implicit type conversion (coercion) like C++ or Java.
    // You cannot add an i32 and an f64 directly. You must cast.
    let float_val = 10.5;
    let int_val = 5;

    let sum = float_val + (int_val as f64);
    println!("Sum: {}", sum);
}

/*
    PRACTICE PROBLEM:
    =================
    You are building a character stat system.
    1. Declare `health` as `u8` (max 100).
    2. Declare `gold` as `i64` (can be negative implies debt).
    3. Declare `is_alive` as `bool`.
    4. Calculate the average of two ratings: `4.5` (f32) and `5` (i32).
       (Hint: Cast the integer to a float).

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: rating_a = 4.5, rating_b = 5
    Output: "Average Rating: 4.75"

    Input: char = 'Z'
    Output: "Character is: Z"
*/
