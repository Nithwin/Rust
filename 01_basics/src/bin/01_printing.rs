fn main() {
    // 1. The Macro
    // println! is a MACRO, not a function. You can tell by the '!'.
    // Macros write code for you before the program compiles.
    println!("Hello, world!");

    // 2. Formatting
    // You can inject values into strings using '{}'.
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // 3. Positional Arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // 4. Named Arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );

    // 5. Debug Printing
    // For things that aren't just strings (like lists), use '{:?}'.
    let list = [1, 2, 3];
    println!("My list: {:?}", list);
}
