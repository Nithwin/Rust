// Topic: Macros
// Fix macro syntax.

macro_rules! say_hello {
    // Error: Missing arguments matcher syntax.
    // Should be () => or ($name:expr) =>
    (x) => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!(x);
}
