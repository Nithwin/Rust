// Topic: Vectors
// Fix mutation during iteration.

fn main() {
    let mut vec = vec![1, 2, 3];

    for val in vec.iter() {
        if *val == 2 {
            // Error: Cannot borrow `vec` as mutable because it is also borrowed as immutable (by iter loop).
            vec.push(4);
        }
    }

    println!("Vec: {:?}", vec);
}
