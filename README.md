# 🦀 Rust: Zero to Hero

Welcome! You've just found your new favorite place to learn Rust.

This isn't just a list of files. It's a carefully crafted journey designed to take you from printing "Hello World" to building complex, high-performance systems. Whether you're here to fix a broken async function or just curious about why everyone loves the borrow checker, you're in the right place.

## 🗺️ Your Journey

We've organized your path into clear, manageable steps.

### Level 1: The Foundation (`01_basics`)
Start here. We cover everything you need to feel comfortable reading and writing Rust.

| Module | What You'll Learn | Challenge yourself! |
| :--- | :--- | :--- |
| **Variables** | `let` vs `mut`, Shadowing | [Fix the Bug](01_basics/challenges/01_variables.rs) |
| **Data Types** | Integers, Floats, Boolean | [Fix the Overflow](01_basics/challenges/02_data_types.rs) |
| **Operators** | Math & Logic | [Fix the Math](01_basics/challenges/03_operators.rs) |
| **Ownership** | **The Core Concept** | [Fix the Move](01_basics/challenges/12_ownership.rs) |
| **Structs** | Custom Types | [Fix Visibility](01_basics/challenges/10_structs.rs) |
| ... and more! | Slices, Vectors, HashMaps, Errors... | Check the `challenges/` folder! |

> **💡 Pro Tip:** Each topic has a corresponding challenge in the `challenges/` subfolder. 
> To run a challenge: `cargo run -p basics --bin challenge_01_variables`

### Level 2: Building Real Tools
Theory is great, but building stuff is better.

- **02_hex_viewer**: Build a CLI tool to inspect binary files (like a hacker!).
- **03_safe_shell**: Create your own mini-shell to manage processes.

### Level 3: Advanced Concepts (`04_advanced`)
Ready for the heavy lifting?

| Module | What You'll Learn | Challenge yourself! |
| :--- | :--- | :--- |
| **Smart Pointers** | `Box`, `Rc`, `RefCell` | [Fix Box Usage](04_advanced/challenges/01_smart_pointers.rs) |
| **Concurrency** | Threads & Mutexes | [Fix Thread Safety](04_advanced/challenges/02_concurrency.rs) |
| **Async** | `async` / `await` | [Fix the Future](04_advanced/challenges/04_async.rs) |
| **Macros** | Metaprogramming | [Fix Macro Syntax](04_advanced/challenges/05_macros.rs) |

## 🚀 How to Start

1.  **Clone this repo:**
    ```bash
    git clone https://github.com/YourUsername/rust-zero-to-hero.git
    cd rust-zero-to-hero
    ```

2.  **Pick a topic.**
    Open `01_basics/src/bin/01_variables.rs` to read the lesson.

3.  **Break things (and fix them).**
    Open `01_basics/challenges/01_variables.rs`. It's broken on purpose!
    Run it, read the error, and fix it:
    ```bash
    cargo run -p basics --bin challenge_01_variables
    ```

## 🤝 Contributing
Found a typo? Have a better explanation? We'd love your help! feel free to open a PR.

---
*Happy Coding! 🦀*
