// Topic: Iterators & Closures
// Fix the ownership issue in the closure.

fn main() {
    let name = String::from("Rust");

    // We want to use 'name' inside the closure.
    // By default, it might borrow.
    // But if we force a move (or if the API requires FnOnce), we check behavior.

    let print_name = || {
        // Just printing is usually fine as borrow.
        // Let's try to drop it to force a move.
        drop(name);
    };

    print_name();

    // Error: 'name' was moved into the closure (because of drop), so we can't use it here.
    // Fix: Maybe don't drop it? Or clone it before?
    println!("Used name: {}", name);
}
