// Topic: Tuples & Arrays
// Fix the array index access that causes a panic.

fn main() {
    let numbers = [1, 2, 3];

    println!("First: {}", numbers[0]);

    // Error: Index out of bounds! The array has length 3 (indices 0, 1, 2).
    let last = numbers[3];

    println!("Last: {}", last);
}
