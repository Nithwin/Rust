// Topic: Strings
// Fix the type mismatch: expected String, found &str.

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    let my_name = "World"; // This is a &str (string slice)

    // Error: greet expects a String struct, not a slice.
    // Convert it using .to_string() or String::from().
    greet(my_name);
}
