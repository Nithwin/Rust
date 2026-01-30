# Rust Systems Foundation: Zero to Hero 🦀

> "The journey of a thousand miles begins with a single `println!`."

Welcome to **Rust Systems Foundation**. This repository is your complete guide to mastering Rust, starting from the absolute basics of printing text to the screen, all the way to advanced memory management and building your own operating system tools.

## Course Structure

The course is organized into sequential folders. We recommend following them in order.

### 01. The Basics (`01_basics/`)
Master the syntax, the standard library, and the unique memory model of Rust.

| Phase | Module | Description |
| :--- | :--- | :--- |
| **Primitives** | [01_variables.rs](./01_basics/src/bin/01_variables.rs) | `let`, `mut`, Shadowing. |
| | [02_data_types.rs](./01_basics/src/bin/02_data_types.rs) | Integers, Floats, Chars. |
| | [03_operators.rs](./01_basics/src/bin/03_operators.rs) | Math & Logic. |
| **Compounds** | [04_tuples_arrays.rs](./01_basics/src/bin/04_tuples_arrays.rs) | Fixed-size groups. |
| | [05_strings.rs](./01_basics/src/bin/05_strings.rs) | `String` (Heap) vs `&str` (Stack). |
| | [06_input.rs](./01_basics/src/bin/06_input.rs) | Reading usage input. |
| **Control Flow** | [07_conditionals.rs](./01_basics/src/bin/07_conditionals.rs) | `if`, `else`, `let if`. |
| | [08_loops.rs](./01_basics/src/bin/08_loops.rs) | `loop`, `while`, `for`. |
| | [09_functions.rs](./01_basics/src/bin/09_functions.rs) | Parameters & Return values. |
| **Types** | [10_structs.rs](./01_basics/src/bin/10_structs.rs) | Custom objects. |
| | [11_enums.rs](./01_basics/src/bin/11_enums.rs) | Variants & `Option<T>`. |
| **Memory** | [12_ownership.rs](./01_basics/src/bin/12_ownership.rs) | **The Core Concept.** |
| | [13_references.rs](./01_basics/src/bin/13_references.rs) | Borrowing rules. |
| | [14_slices.rs](./01_basics/src/bin/14_slices.rs) | Views into memory. |
| **Collections** | [15_vectors.rs](./01_basics/src/bin/15_vectors.rs) | Growable arrays. |
| | [16_hashmaps.rs](./01_basics/src/bin/16_hashmaps.rs) | Key-Value stores. |
| **Errors** | [17_errors.rs](./01_basics/src/bin/17_errors.rs) | `Result<T, E>` and `panic!`. |

### 02. Hex Viewer Project (`02_hex_viewer/`)
**Goal:** Build a tool to inspect binary files byte-by-byte.
- [Source Code](./02_hex_viewer/src/main.rs)
- **Concepts:** File I/O, Buffers, Bitwise Operations.

### 03. Safe Shell Project (`03_safe_shell/`)
**Goal:** Build a command-line interface that manages processes.
- [Source Code](./03_safe_shell/src/main.rs)
- **Concepts:** Syscalls, Process Management, REPL patterns.

### 04. Advanced Topics (`04_advanced/`)
**Goal:** Unlock the full power of Rust.

| Module | Description |
| :--- | :--- |
| [01_smart_pointers.rs](./04_advanced/src/bin/01_smart_pointers.rs) | `Box`, `Rc`, `RefCell`. |
| [02_concurrency.rs](./04_advanced/src/bin/02_concurrency.rs) | Threads & Mutexes. |
| [03_unsafe.rs](./04_advanced/src/bin/03_unsafe.rs) | Raw Pointers & FFI. |

## How to Run the Code

1. **Install Rust:** [rustup.rs](https://rustup.rs/)
2. **Clone the Repo:**
   ```bash
   git clone https://github.com/YourUsername/rust-systems-foundation.git
   cd rust-systems-foundation
   ```
3. **Run a Module:**
   To run a specific lesson (e.g., Variables):
   ```bash
   cargo run --bin 01_variables
   ```
   *Note: This works because `01_basics` is part of the workspace.*

4. **Run a Project:**
    To run the Hex Viewer:
    ```bash
    cargo run -p hex_viewer -- <file_path>
    ```

## Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License
MIT License. See [LICENSE](./LICENSE).
