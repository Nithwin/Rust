/*
    Topic: Macros

    Rust macros allow you to write code that writes code (Metaprogramming).

    Key Concepts:
    1. Declarative Macros (`macro_rules!`) - simpler, pattern matching
    2. Procedural Macros - complex, work on AST (not covered in depth here)
*/

// A simple macro to say hello
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    // With an argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// A macro to create a vector (simplified built-in vec![])
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("--- Declarative Macros ---");
    say_hello!();
    say_hello!("Rustacean");

    println!("\n--- Custom Vec Macro ---");
    let v = my_vec![1, 2, 3, 4, 5];
    println!("Created vector: {:?}", v);
}
