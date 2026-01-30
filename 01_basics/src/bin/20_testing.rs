/*
    Topic: Testing

    Rust has a built-in test runner.

    Key Concepts:
    1. Unit Tests (inside the same file)
    2. `#[test]` attribute
    3. Assertions
*/

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

fn main() {
    println!("To run tests, use the command: cargo test --bin 20_testing");
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    // Bring parent functions into scope
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(10, -2), 8);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    fn test_assertion_failure() {
        let result = add(2, 2);
        // This is how you verify boolean conditions
        assert!(result == 4, "Result should be 4");
    }
}
