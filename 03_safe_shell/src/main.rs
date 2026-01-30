use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Welcome to Safe-Shell 🐚");
    println!("Type 'exit' to quit.");

    loop {
        // 1. PRINT PROMPT
        // We use print! instead of println! because we want the cursor to stay on the same line.
        print!("> ");
        // Standard output is line-buffered. We need to flush it to ensure the prompt appears immediately.
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
            continue;
        }

        // 2. READ INPUT
        let mut input = String::new();
        // stdin().read_line() blocks until the user presses Enter.
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        // 3. PARSE INPUT
        // trim() removes the newline character at the end.
        // split_whitespace() breaks the string into words.
        let mut parts = input.trim().split_whitespace();

        // The first part is the command name.
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue, // Empty input
        };

        // The rest are arguments.
        let args: Vec<&str> = parts.collect();

        // 4. EXECUTE COMMAND
        // We handle built-in commands (like 'cd' and 'exit') differently from external programs.
        match command {
            "exit" => {
                println!("Goodbye! 👋");
                break;
            }
            "cd" => {
                // Change Directory
                // Why is this a built-in? Because if we ran 'cd' as a child process,
                // it would change *its own* directory and then die. The parent shell would stay put!
                // We must change the shell's own current directory.
                let new_dir = args.get(0).unwrap_or(&"/");
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}", e);
                }
            }
            _ => {
                // External Command
                // We ask the OS to spawn a new process.
                let child_process = Command::new(command).args(args).spawn();

                match child_process {
                    Ok(mut child) => {
                        // The child is running. We must wait for it to finish.
                        // If we didn't wait, the shell would print the next prompt immediately,
                        // mixing its output with the child's output (a race condition!).
                        if let Err(e) = child.wait() {
                            eprintln!("Failed to wait on child: {}", e);
                        }
                    }
                    Err(e) => {
                        // This usually happens if the command doesn't exist.
                        eprintln!("Command not found: '{}' ({})", command, e);
                    }
                }
            }
        }
    }
}
