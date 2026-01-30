/*
    topic: Collections - HashMaps

    DEEP DIVE THEORY:
    =================
    HashMaps store data as Key-Value pairs (`HashMap<K, V>`).

    Hashing:
    - Keys are hashed using a "Hash Function" to determine where to store them in memory.
    - Rust uses a secure hash function (SipHash) by default to prevent "HashDoS" attacks.
    - This is slower than some algorithm but safer. You can swap the hasher if you need raw speed.

    Ownership:
    - Types that implement `Copy` (i32) are copied into the map.
    - Types that don't (String) are MOVED into the map. You lose the original variable.
*/

use std::collections::HashMap;

fn main() {
    // 1. Creating
    let mut scores = HashMap::new();

    // 2. Inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. Accessing
    let team_name = String::from("Blue");

    // .get() returns Option<&V> because the key might not exist.
    if let Some(score) = scores.get(&team_name) {
        println!("{} team score: {}", team_name, score);
    }

    // 4. Updating
    // Overwrite
    scores.insert(String::from("Blue"), 25);

    // Build or Insert (Only if missing)
    scores.entry(String::from("Yellow")).or_insert(50); // Exists, does nothing
    scores.entry(String::from("Green")).or_insert(50); // Missing, inserts

    println!("{:?}", scores);

    // 5. Updating based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns mutable reference to value
        *count += 1; // dereference and add
    }

    println!("{:?}", map);
}

/*
    PRACTICE PROBLEM:
    =================
    Create a student grade book.
    1. Create a HashMap `grades` (Student Name -> Grade).
    2. Add "Alice": 85, "Bob": 90.
    3. Alice retook the exam. Update her grade to 95.
    4. "Charlie" is new. Add him with 88 only if he isn't there (use entry).
    5. Print the final map.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input (Code steps above)
    Output: {"Alice": 95, "Bob": 90, "Charlie": 88} (Order may vary in HashMap)
*/
