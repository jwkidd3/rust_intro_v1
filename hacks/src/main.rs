

use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let _file = match file_result {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };
}