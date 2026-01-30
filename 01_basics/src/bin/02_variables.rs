fn main() {
    // 1. Immutability by Default
    // In Rust, variables are immutable (constant) by default.
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause a compile error!

    // 2. Mutability
    // To make a variable changeable, allow it to "mutate" with 'mut'.
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The value of y is now: {}", y);

    // 3. Constants
    // Constants are ALWAYS immutable and must have a type.
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // 4. Shadowing
    // You can declare a new variable with the same name as a previous one.
    let spaces = "   ";
    let spaces = spaces.len(); // Changing type from string to int!
    println!("Spaces count: {}", spaces);
}
