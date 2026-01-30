// Topic: HashMaps
// Fix handling of missing keys (Option).

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Green");

    // Error: .get() returns Option<&v>. We can't print Option directly as a value without checking.
    // Also, if we unwrap() a missing key, it panics.
    // Fix: Use .unwrap_or(&0) or match.
    let score = scores.get(&team_name).unwrap();

    println!("Score for {}: {}", team_name, score);
}
