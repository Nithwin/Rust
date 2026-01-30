// Topic: Loops
// Fix the infinite loop so the program finishes.

fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            // Error: We check the count but never break!
            // Add a 'break;' statement here.
        }
    }

    println!("Done!");
}
