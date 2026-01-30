// Topic: Traits
// Implement the missing method.

trait Speak {
    fn say_hello(&self);
}

struct Human;

// Error: 'say_hello' is not implemented in this block.
impl Speak for Human {}

fn main() {
    let h = Human;
    h.say_hello();
}
