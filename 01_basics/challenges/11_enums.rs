// Topic: Enums
// Fix the non-exhaustive match.

enum Command {
    Start,
    Stop,
    Pause, // <-- New variant
}

fn execute(cmd: Command) {
    match cmd {
        Command::Start => println!("Starting"),
        Command::Stop => println!("Stopping"),
        // Error: Missing 'Command::Pause' arm.
    }
}

fn main() {
    execute(Command::Pause);
}
