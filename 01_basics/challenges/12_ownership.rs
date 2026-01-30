// Topic: Ownership
// Fix the use after move.

fn main() {
    let s1 = String::from("hello");

    // Move occurs here
    let s2 = s1;

    println!("s2: {}", s2);

    // Error: s1 is invalid now.
    // Fix: Clone s1 when creating s2, OR don't use s1 here.
    println!("s1: {}", s1);
}
