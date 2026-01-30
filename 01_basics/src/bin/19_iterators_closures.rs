/*
    Topic: Iterators and Closures

    Functional programming patterns are first-class citizens in Rust.

    Key Concepts:
    1. Closures: Anonymous functions that can capture their environment.
    2. Iterators: A sequence of elements processing.
    3. Common Adapters: map, filter, collect.
*/

fn main() {
    // --- Closures ---
    println!("--- Closures ---");

    let factor = 2;
    // A closure that captures `factor` from the scope
    let multiplier = |x: i32| x * factor;

    let val = 10;
    println!("{} * {} = {}", val, factor, multiplier(val));

    // --- Iterators ---
    println!("\n--- Iterators ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // .iter() creates an iterator. The vector is NOT consumed here.
    // .map() transforms each item
    // .collect() consumes the iterator and builds a new collection
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);

    // .filter() keeps items that return true
    let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    // Note: `into_iter()` MOVES (consumes) `numbers`. We can't use `numbers` after this.

    println!("Evens: {:?}", evens);

    // --- Fold ---
    // Fold reduces a collection to a single value (like 'reduce' in JS)
    let sum: i32 = doubled.iter().fold(0, |acc, x| acc + x);
    println!("Sum of doubled: {}", sum);
}
