/*
    topic: Control Flow - Loops

    DEEP DIVE THEORY:
    =================
    Rust provides three kinds of loops:

    1. **loop**:
       - Infinite loop.
       - Can return a value using `break value;`.
       - Useful for retrying operations or servers.

    2. **while**:
       - Loops while a condition is true.
       - Standard "C-style" while loop.

    3. **for**:
       - The most common and safest loop in Rust.
       - Iterates over a collection or a range.
       - `for x in collection` prevents "Off-by-one" errors common in Cstyle `for (i=0; i<len; i++)`.

    Ranges:
    - `1..4` -> 1, 2, 3 (Exclusive)
    - `1..=4` -> 1, 2, 3, 4 (Inclusive)
*/

fn main() {
    // 1. loop (returning a value)
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Returns 20
        }
    };

    println!("The result is: {}", result);

    // 2. while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 3. for (Range)
    for i in 1..4 {
        println!("Range (exclusive): {}", i); // 1, 2, 3
    }

    // 4. for (Collection)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("Value: {}", element);
    }
}

/*
    PRACTICE PROBLEM:
    =================
    Find the factorial of a number using a `for` loop.
    1. Variable `n` = 5.
    2. Variable `factorial` = 1.
    3. Loop from 1 to n (inclusive) and multiply `factorial` by logic.
    4. Print result.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: n = 5
    Output: "Factorial of 5 is 120" (1*2*3*4*5)

    Input: n = 0
    Output: "Factorial of 0 is 1" (Loop range 1..=0 is empty, factorial stays 1)
*/
