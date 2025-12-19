// cargo run --example m4_result

use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    match file_result {
        Ok(_f) => println!("File opened!"),
        Err(e) => println!("Error: {}", e),
    }
}
