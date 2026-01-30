// Topic: Conditionals
// Fix the type mismatch between if/else blocks.

fn main() {
    let condition = true;

    // Error: 'if' and 'else' arms must return the same type.
    // One returns an integer, the other a string.
    let result = if condition { 5 } else { "six" };

    // Hint: Make them both integers or both strings?
    // Let's say we want the number 6.
    println!("Result: {}", result);
}
