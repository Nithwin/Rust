/*
    topic: Collections - Vectors

    DEEP DIVE THEORY:
    =================
    Vectors (`Vec<T>`) are re-sizable arrays.

    Memory Layout:
    - **Stack**: Pointer to Heap data, Length, Capacity.
    - **Heap**: The actual elements.

    Capacity vs Length:
    - **Length**: How many elements are currently in the vector.
    - **Capacity**: How many elements can represent before re-allocating.
    - When you push beyond capacity, Rust allocates a new, larger chunk of memory on the heap,
      copies everything over, and frees the old chunk. This is "expensive".

    Access Safety:
    - `&v[100]` -> Panics if out of bounds (Crash). use this when you are 100% sure.
    - `v.get(100)` -> Returns `Option<&T>` (None). Use this for user input or uncertain logic.
*/

fn main() {
    // 1. Creating a Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    // Macro shortcut
    let mut v2 = vec![10, 20, 30];

    // 2. Reading
    let third: &i32 = &v2[2]; // Immutable borrow
    println!("The third element is {}", third);

    // 3. Iterating (Immutable)
    for i in &v2 {
        println!("{}", i);
    }

    // 4. Modifying while iterating
    // We need a mutable reference to the vector element.
    for i in &mut v2 {
        *i += 50; // Dereference (*) to get to the value and change it
    }
    println!("Modified: {:?}", v2);

    // 5. Storing multiple types (The Enum trick)
    // Vectors can only store ONE type. To store multiple, wrap them in an Enum.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row: {:?}", row);
}

/*
    PRACTICE PROBLEM:
    =================
    1. Create a function `filter_even(numbers: Vec<i32>) -> Vec<i32>`.
    2. It should take ownership of the vector, keep only even numbers, and return the new/modified vector.
    3. Hint: You can create a new vector and push evens into it, or use `retain`.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: vec![1, 2, 3, 4, 5, 6]
    Output: vec![2, 4, 6]

    Input: vec![1, 3, 5]
    Output: vec![]
*/
