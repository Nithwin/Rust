/*
    topic: Types - Structs

    DEEP DIVE THEORY:
    =================
    Structs allow you to name and package together multiple related values that make up a meaningful group.

    1. **Named Structs**: `{ x: i32, y: i32 }`. Clear field names.
    2. **Tuple Structs**: `(i32, i32)`. Named tuples, good for "New Types" (e.g., `struct Color(i32, i32, i32)`).
    3. **Unit Structs**: `struct AlwaysEqual;`. No fields. Useful for Traits.

    Methods vs Functions:
    - **Functions** are free-floating (`fn main()`).
    - **Methods** are defined within an `impl` block and their first parameter is always `self`.

    The `self` parameter:
    - `&self`: Short for `self: &Self`. Borrows the instance immutably. (Read-only)
    - `&mut self`: Borrows the instance mutably. (Read-Write)
    - `self`: Takes ownership. The instance is consumed and cannot be used afterwards. (Destructors/Transformers)
*/

// 1. Definiing a Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 2. Methods
impl User {
    // Associated Function (Constructor pattern) - No `self`
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    // Method (Immutable Borrow)
    fn print_info(&self) {
        println!("User: {} ({})", self.username, self.email);
    }

    // Method (Mutable Borrow)
    fn consume_login(&mut self) {
        self.sign_in_count += 1;
        println!("Login count updated to {}", self.sign_in_count);
    }

    // Method (Ownership logic)
    fn deactivate(mut self) {
        self.active = false;
        println!("User {} has been deactivated.", self.username);
        // `self` is dropped here. Accessing it in main() after this is impossible.
    }
}

fn main() {
    let mut user1 = User::new(
        String::from("rustacean123"),
        String::from("rust@example.com"),
    );

    user1.print_info();
    user1.consume_login();

    // Ownership transfer
    user1.deactivate();
    // user1.print_info(); // ERROR: Value used after move
}

/*
    PRACTICE PROBLEM:
    =================
    Define a `Rectangle` struct.
    1. Fields: `width` (u32), `height` (u32).
    2. Method `area(&self) -> u32`: Returns width * height.
    3. Method `can_hold(&self, other: &Rectangle) -> bool`: Returns true if self can fit other inside it.
    4. Associated Function `square(size: u32) -> Rectangle`: Creates a square.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: rect1 = (30, 50)
    Output: Area = 1500

    Input: rect1 = (30, 50), rect2 = (10, 20)
    Output: can_hold = true

    Input: rect1 = (30, 50), rect3 = (60, 45)
    Output: can_hold = false
*/
