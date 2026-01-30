/*
    Topic: Traits in Rust

    Traits are similar to "Interfaces" in other languages.
    They define shared behavior that different types can implement.

    Key Concepts:
    1. Defining a Trait
    2. Implementing a Trait for a Type
    3. Default Implementations
    4. Trait Bounds (Generic Constraints)
*/

// 1. Defining a Trait
trait Summarizable {
    fn author(&self) -> String;

    // Abstract method (must be implemented)
    fn summary(&self) -> String;

    // 3. Default Implementation
    fn notify(&self) {
        println!("(Read more from {}...)", self.author());
    }
}

struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 2. Implementing a Trait for a Type
impl Summarizable for Article {
    fn author(&self) -> String {
        self.author.clone()
    }

    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 4. Trait Bounds
// This function accepts ANY type that implements Summarizable
fn notify_item<T: Summarizable>(item: &T) {
    println!("Breaking News! {}", item.summary());
    item.notify(); // Calls default implementation if not overridden
}

fn main() {
    let article = Article {
        headline: String::from("Rust 1.0 Released"),
        location: String::from("The Internet"),
        author: String::from("Graydon Hoare"),
        content: String::from("Rust is now stable..."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    println!("1. Trait Usage:");
    println!("Article Summary: {}", article.summary());
    println!("Tweet Summary: {}", tweet.summary());

    println!("\n2. Default Implementations:");
    article.notify();

    println!("\n3. Generic Function with Trait Bounds:");
    notify_item(&article);
    notify_item(&tweet);
}
