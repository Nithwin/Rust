/*
    topic: Types - Enums
    
    DEEP DIVE THEORY:
    =================
    Enums (Enumerations) allow you to define a type by enumerating its possible variants.
    
    Superpowered Enums:
    In C/C++, enums are mostly just integers. In Rust, enum variants can hold **data**.
    This eliminates the need for "Tagged Unions" or "Variant" classes found in other languages.
    
    The Option Enum:
    Rust does NOT have `null`. A value is either present (`Some(T)`) or absent (`None`).
    This forces you to handle the "missing" case explicitly, preventing Null Pointer Exceptions (The Billion Dollar Mistake).
    
    enum Option<T> {
        Some(T),
        None,
    }
*/

// 1. Enum with Data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Moving to {}, {}", x, y),
            Message::Write(text) => println!("Message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello Rust"));
    msg.call();
    
    // 2. Option<T>
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    
    // We must unwrap or match to use the value. We can't just add 5 + some_number.
    if let Some(n) = some_number {
        println!("We have a number: {}", n);
    } else {
        println!("No number found.");
    }
}

/*
    PRACTICE PROBLEM:
    =================
    1. Define an enum `Shape` with variants:
       - `Circle(f64)` (radius)
       - `Rectangle(f64, f64)` (width, height)
       - `Triangle(f64, f64)` (base, height)
    2. Implement a procedure that takes a `Shape` and prints its area.
       - Circle: 3.14 * r * r
       - Rectangle: w * h
       - Triangle: 0.5 * b * h

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: Shape::Circle(2.0)
    Output: "Area: 12.56"

    Input: Shape::Rectangle(3.0, 4.0)
    Output: "Area: 12.0"
*/
