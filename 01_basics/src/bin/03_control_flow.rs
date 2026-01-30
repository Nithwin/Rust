fn main() {
    let number = 3;

    // 1. If/Else
    // Conditions MUST be booleans. No "if (1)" like in C.
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // 2. Using 'if' in a 'let' statement
    // Because 'if' is an expression, it returns a value.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // 3. Loops
    // 'loop' runs forever until you break.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // You can return values from loops!
        }
    };
    println!("The result is {}", result);

    // 4. While
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 5. For (Iterators)
    // This is the safest way to loop (no off-by-one errors).
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Ranges
    for number in (1..4).rev() {
        // 1 to 3, reversed
        println!("{}...", number);
    }
}
