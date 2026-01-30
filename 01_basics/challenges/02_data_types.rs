// Topic: Data Types
// Fix the type mismatch and the overflow.

fn main() {
    let my_float: f64 = 5.0;

    // Error 1: Mismatched types. Fix by casting the integer.
    let result = my_float + 10;
    println!("Result: {}", result);

    // Error 2: Overflow.
    let mut mini_byte: u8 = 255;
    println!("Mini byte: {}", mini_byte);

    // This will panic/overflow. Use wrapping_add or check constraints.
    mini_byte = mini_byte + 1;
    println!("Mini byte after add: {}", mini_byte);
}
