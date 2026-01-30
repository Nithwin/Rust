// Topic: Operators
// Fix the calculation to get the expected result (Temperature conversion).

fn main() {
    let fahrenheit = 90.0;

    // Goal: Convert F to C. Formula: (F - 32) * 5/9
    // Error: Operator precedence is wrong here.
    let celsius = fahrenheit - 32.0 * 5.0 / 9.0;

    println!("{}F is {}C", fahrenheit, celsius);

    // Validation
    if (celsius - 32.22).abs() < 0.1 {
        println!("Correct!");
    } else {
        println!("Wrong! Expected approx 32.22C");
    }
}
