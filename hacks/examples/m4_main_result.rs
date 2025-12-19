// cargo run --example m4_main_result
//
// Demonstrates using ? operator directly in main() by having main return Result.
// This is useful for simple programs where you want errors to propagate to exit.

use std::fs::File;
use std::io::{self, Read};

// main() can return Result<(), E> where E implements std::error::Error
// If main returns Err, the program exits with an error message
fn main() -> Result<(), io::Error> {
    println!("=== main() returning Result ===\n");

    // With main returning Result, we can use ? directly
    // If the file doesn't exist, main returns Err and program exits
    println!("Attempting to open 'test_file.txt'...");

    // Create a test file first so the example can succeed
    std::fs::write("test_file.txt", "Hello from test file!")?;
    println!("Created test file.");

    // Now read it using ?
    let mut file = File::open("test_file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

    // Clean up
    std::fs::remove_file("test_file.txt")?;
    println!("Cleaned up test file.");

    // Return Ok(()) to indicate success
    // If we reach here, everything worked!
    println!("\nProgram completed successfully!");
    Ok(())
}

// Note: If any ? returns an Err, you'll see output like:
// Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
