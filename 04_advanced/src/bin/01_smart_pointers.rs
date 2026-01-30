/*
    topic: Smart Pointers
    
    DEEP DIVE THEORY:
    =================
    Smart Pointers are data structures that act like pointers but have "metadata" and "capabilities".
    In Rust, they typically implement `Deref` (to behave like pointers) and `Drop` (to clean up).
    
    1. **Box<T>**: 
       - Allocates data on the HEAP.
       - You own the box, the box owns the data.
       - Use when: You have a recursive type (unknown size) or need to transfer ownership of large data without copying.
       
    2. **Rc<T>** (Reference Counted):
       - Enables MULTIPLE owners for the same data (Immutable).
       - Keeps track of the number of references.
       - Drops data only when count is 0.
       - Single-threaded only (not atomic).
       
    3. **RefCell<T>** (Interior Mutability):
       - Allows you to mutate data even when there are immutable references to it.
       - Checks borrowing rules at RUNTIME, not COMPILE TIME.
       - Use carefully! Panics if rules are broken.
       
    Combining `Rc<RefCell<T>>` allows multiple owners to mutate the same data.
*/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 1. Box<T> (Heap Allocation)
    let b = Box::new(5);
    println!("b = {}", b);
    
    // 2. Rc<T> (Multiple Ownership)
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    
    let c = Cons(4, Rc::clone(&a));
    println!("Count after creating c: {}", Rc::strong_count(&a));
    
    // 3. Interior Mutability (RefCell)
    let data = Rc::new(RefCell::new(5));
    
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    
    *owner1.borrow_mut() += 10;
    
    println!("Owner 2 sees: {:?}", owner2.borrow());
}

/*
    PRACTICE PROBLEM:
    =================
    Create a mock "logger" system shared by multiple workers.
    1. Define a struct `Worker` that holds a `name` and a shared `log` (Vec<String>).
    2. Use `Rc<RefCell<Vec<String>>>` for the log.
    3. Create two workers sharing the SAME log.
    4. Have Worker A write "A started".
    5. Have Worker B write "B started".
    6. Print the log from both workers to prove they see the same data.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: Worker A writes, Worker B writes
    Output: Log contains ["A started", "B started"]
*/
