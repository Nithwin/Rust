use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

// Constants: Magic numbers are bad, named constants are good.
// 16 bytes is a standard hex view width.
const BYTES_PER_LINE: usize = 16;

fn main() -> io::Result<()> {
    // 1. ARGUMENT PARSING
    // We get arguments from the OS. The first one is the program name,
    // so we need the second one (index 1).
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // 2. OPENING THE FILE
    // We ask the OS to give us a handle to the file.
    // This is a "System Call" under the hood!
    let file = File::open(filename)?;

    // We use a BufReader. Why?
    // Reading byte-by-byte from the disk is slow (lots of system calls).
    // A buffer reads a big chunk into memory (RAM) at once, which is much faster.
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BYTES_PER_LINE];

    let mut offset = 0;

    loop {
        // 3. READING DATA
        // Try to fill our buffer with bytes from the file.
        // `n` is the number of bytes we actually read.
        let n = reader.read(&mut buffer)?;

        if n == 0 {
            break; // End of file (EOF)
        }

        // 4. DISPLAYING DATA
        print_hex_line(&buffer[..n], offset);

        offset += n;
    }

    Ok(())
}

/// Formats and prints a single line of the hex dump.
///
/// Format:
/// OFFSET   HEX BYTES                  ASCII
/// 00000000 48 65 6c 6c 6f 20 57 6f... Hello Wo...
fn print_hex_line(bytes: &[u8], offset: usize) {
    // Print the offset (where we are in the file)
    // %08x means "print as hex, pad with 0s to 8 characters"
    print!("{:08x}  ", offset);

    // Print the Hexadecimal representation
    for i in 0..BYTES_PER_LINE {
        if i < bytes.len() {
            print!("{:02x} ", bytes[i]);
        } else {
            print!("   "); // Padding for partial lines
        }

        // Add an extra space in the middle for readability
        if i == 7 {
            print!(" ");
        }
    }

    print!(" |");

    // Print the ASCII representation
    for &byte in bytes {
        // We only print printable characters (ASCII 32-126).
        // Everything else (like newlines or null bytes) gets a dot.
        if byte >= 32 && byte <= 126 {
            print!("{}", byte as char);
        } else {
            print!(".");
        }
    }

    print!("|");
    println!(); // Newline
}
