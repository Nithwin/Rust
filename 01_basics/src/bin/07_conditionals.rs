/*
    topic: Control Flow - Conditionals

    DEEP DIVE THEORY:
    =================
    1. **if/else**:
       - The condition MUST be a boolean. Rust will not automatically convert numbers to bools
         (e.g., `if 1 { ... }` is an error, unlike C).
       - Blocks `{}` are mandatory. You cannot omit them for single lines.

    2. **let if (Expression)**:
       - In Rust, `if` is an **expression**, meaning it returns a value.
       - You can use this to assign values based on a condition cleanly.
       - Both arms of the `if/else` must return the SAME type.

    3. **match**:
       - Rust's superpower. It's like a `switch` statement but exhaustive.
       - You MUST handle every possible case.
       - Can pattern match on values, ranges, enums, etc.
*/

fn main() {
    let number = 6;

    // 1. Standard if/else
    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else {
        println!("Not divisible by 4 or 3");
    }

    // 2. let if (Expression)
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let error = if condition { 5 } else { "six" }; // ERROR: incompatible types

    println!("Number is: {}", number);

    // 3. match (Preview)
    let input = "start";
    match input {
        "start" => println!("Starting engine..."),
        "stop" => println!("Stopping engine..."),
        _ => println!("Invalid command"), // _ catches everything else
    }
}

/*
    PRACTICE PROBLEM:
    =================
    Create a grading system.
    1. Create a variable `score`.
    2. Use an `if/else if/else` chain to determine the grade:
       - 90+ = "A"
       - 80-89 = "B"
       - 70-79 = "C"
       - Below 70 = "F"
    3. Print "Score: [score], Grade: [grade]".

    Bonus: Try to do step 2 using `let grade = if ...` assignment.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: score = 95
    Output: "Score: 95, Grade: A"

    Input: score = 80
    Output: "Score: 80, Grade: B"

    Input: score = 50
    Output: "Score: 50, Grade: F"
*/
