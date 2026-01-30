// Topic: References
// Fix the mutable borrow occurring while immutable borrow is active.

fn main() {
    let mut list = vec![1, 2, 3];

    let first = &list[0]; // Immutable borrow

    list.push(4); // Mutable borrow (modification)

    // Error: We use 'first' here, extending its lifetime scope to include the push.
    // But we modified the list in the middle!
    println!("First was: {}", first);
}
