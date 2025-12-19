// cargo run --example m4_question_mark
//
// Demonstrates the ? operator for concise error propagation.
// The ? operator:
//   - If Result is Ok(value), unwraps and returns the value
//   - If Result is Err(e), returns early from the function with that error

use std::fs::File;
use std::io::{self, Read};

// Function that might fail - returns Result
// The ? operator automatically propagates errors to the caller
fn read_file() -> Result<String, io::Error> {
    let mut contents = String::new();

    // Without ?: Would need match or if-let to handle errors
    // With ?: If open fails, function returns Err immediately
    //         If open succeeds, we get the File and continue
    File::open("hello.txt")?
        .read_to_string(&mut contents)?;  // Same for read_to_string

    // If we reach here, both operations succeeded
    Ok(contents)
}

// This is equivalent to the above, but more verbose
fn read_file_verbose() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),  // Early return on error
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn main() {
    // Call our function and handle the Result
    match read_file() {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Could not read file: {}", e),
    }

    // The ? operator makes error handling much cleaner!
    // Compare read_file() vs read_file_verbose() above
}
