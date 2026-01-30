// Topic: Smart Pointers
// Fix the box dereference.

fn main() {
    let x = 5;
    let b = Box::new(x);

    // Error: Cannot compare Box<i32> with integer.
    // Use *b to dereference.
    if b == 5 {
        println!("Match!");
    } else {
        println!("Mismatch");
    }
}
