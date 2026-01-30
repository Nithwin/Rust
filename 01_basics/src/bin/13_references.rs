/*
    topic: References and Borrowing

    DEEP DIVE THEORY:
    =================
    We often want to use a value without taking ownership of it. This is called **Borrowing**.
    We create a **Reference** using `&`.

    The Rules of Borrowing (The Borrow Checker):
    1. At any given time, you can have **EITHER** one mutable reference (`&mut T`)
       **OR** any number of immutable references (`&T`).
    2. References must always be valid (no null pointers, no dangling pointers).

    Why?
    This prevents **Data Races** at compile time. A data race occurs when:
    - Two pointers access the same data.
    - At least one overrides (mutates) the data.
    - No synchronization mechanisms are used.
*/

fn main() {
    let s1 = String::from("hello");

    // 1. Immutable Reference (&T)
    let len = calculate_length(&s1); // We pass a reference. s1 is BORROWED.
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid!

    // 2. Mutable Reference (&mut T)
    let mut s2 = String::from("hello");
    change(&mut s2); // We pass a mutable reference.
    println!("Changed: {}", s2);

    // 3. The Rules (Race Condition Prevention)
    let mut s3 = String::from("hello");

    let r1 = &s3; // OK
    let r2 = &s3; // OK
    // let r3 = &mut s3; // EXTREME ERROR! Cannot borrow mutably while immutable borrows exist.

    println!("{}, {}", r1, r2); // Scope of r1/r2 ENDS here (Non-Lexical Lifetimes).

    let r3 = &mut s3; // Now OK, because r1/r2 are no longer used.
    r3.push_str(" world");
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't own the string, so nothing changes.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
    PRACTICE PROBLEM:
    =================
    1. Write a function `add_stars(s: &mut String)` that appends "***" to the string.
    2. In `main`, create a string "Rust".
    3. Pass it to the function.
    4. Print the result.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: "Rust"
    Output: "Rust***"
*/
