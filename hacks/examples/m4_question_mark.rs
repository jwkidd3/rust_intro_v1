// cargo run --example m4_question_mark

use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file() {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
