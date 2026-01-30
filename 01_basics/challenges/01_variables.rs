// Topic: Variables
// Fix the error by changing how 'x' is declared!

fn main() {
    let x = 5;
    println!("x is {}", x);
    x = 6; // Error: cannot assign twice to immutable variable
    println!("x is now {}", x);
}
