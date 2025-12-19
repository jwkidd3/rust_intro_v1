// cargo run --example m4_result
//
// Demonstrates Result<T, E> - Rust's primary error handling type.
// Result has two variants:
//   Ok(T)  - operation succeeded, contains the success value
//   Err(E) - operation failed, contains the error value

use std::fs::File;

fn main() {
    // File::open returns Result<File, std::io::Error>
    // It might succeed (file exists) or fail (file doesn't exist)
    let file_result = File::open("hello.txt");

    // Use match to handle both success and failure cases
    match file_result {
        Ok(file) => {
            // We successfully opened the file
            println!("File opened successfully!");
            println!("File: {:?}", file);
        }
        Err(error) => {
            // The file couldn't be opened - handle the error gracefully
            println!("Failed to open file: {}", error);
            // Program continues instead of crashing
        }
    }

    println!("Program continues after error handling!");
}
