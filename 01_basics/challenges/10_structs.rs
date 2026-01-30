// Topic: Structs
// Fix private field access and use struct update syntax.

mod user_system {
    pub struct User {
        pub username: String,
        // Error: This field is private. Make it 'pub'.
        email: String,
        pub active: bool,
    }

    impl User {
        pub fn new(username: String, email: String) -> User {
            User {
                username,
                email,
                active: true,
            }
        }
    }
}

fn main() {
    let user1 = user_system::User::new(String::from("alice"), String::from("alice@example.com"));

    // Error: Accessing private field.
    println!("User email: {}", user1.email);

    // Error: Manual copy instead of update syntax.
    // Use `..user1` instead of manually setting active: false (wait, we want to KEEP active status or change it?)
    // Let's say we want to create a new user with same active status.
    let user2 = user_system::User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        // Error: We are missing 'active'.
        // Use `..user1` to fill the rest.
    };

    println!("User 2 active: {}", user2.active);
}
