/*
    topic: Unsafe Rust
    
    DEEP DIVE THEORY:
    =================
    Rust's compiler guarantees memory safety... usually. 
    `unsafe` is a keyword that tells the compiler: "Trust me, I know what I'm doing."
    
    It grants 5 Superpowers:
    1. Dereference raw pointers.
    2. Call `unsafe` functions (like C FFI).
    3. Access or modify mutable static variables.
    4. Implement `unsafe` traits.
    5. Access fields of `union`s.
    
    It does NOT turn off the borrow checker. It just allows specific operations that *could* 
    cause undefined behavior if done wrong.
*/

fn main() {
    // 1. Raw Pointers
    let mut num = 5;

    // Creating raw pointers is SAFE.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing them is UNSAFE.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        // We can create a pointer to random memory (EXTREMELY DANGEROUS)
        // let address = 0x012345usize;
        // let _r = address as *const i32;
        // println!("{}", *_r); // Segfault!
    }

    // 2. Calling Unsafe Functions
    unsafe {
        dangerous();
    }
    
    // 3. FFI (Foreign Function Interface) - Calling C code
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

unsafe fn dangerous() {
    // We could do anything here...
}

/*
    PRACTICE PROBLEM:
    =================
    (Theoretical)
    1. Create a mutable static variable `GlobalCounter`.
    2. Create a function that increments it inside an `unsafe` block.
    3. Call it 3 times.
    4. Print the result.
    
    Warning: Mutable statics are notorious for race conditions in threaded code.

    INPUT/OUTPUT TEST CASES:
    ========================
    Input: call increment() x3
    Output: GlobalCounter: 3
*/
