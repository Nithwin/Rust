// Topic: Testing
// Fix the bug in the code so the test passes.

fn add_two(x: i32) -> i32 {
    x + 3 // Error: Logic bug! Should be + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
}
