// Topic: Unsafe Rust
// Fix unsafe block.

fn main() {
    let mut num = 5;

    // Create raw pointer
    let r1 = &mut num as *mut i32;

    // Error: Dereferencing raw pointer requires unsafe block.
    // Wrap the next line in 'unsafe { ... }'
    *r1 += 1;

    println!("Num is now: {}", num);
}
