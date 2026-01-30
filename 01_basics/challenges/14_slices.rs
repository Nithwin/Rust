// Topic: Slices
// Fix slice range indices.

fn main() {
    let s = String::from("hello world");

    // Error: 'hello' is first 5 chars. Indices 0..5 (exclusive).
    // But wait, what if we use the wrong range?
    // Let's try to slice in the middle of a multi-byte char, or just out of bounds.
    // Simplest error: Start index > End index.

    let slice = &s[5..0]; // Error: starting index is greater than ending index panic.
    // Fix it to be &s[0..5] or similar.

    println!("Slice: {}", slice);
}
