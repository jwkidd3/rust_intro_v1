# Lab 6: Error Handling

**Duration:** 35 minutes

## Objectives

- Use panic! for unrecoverable errors
- Handle errors with Result and match
- Use unwrap and expect
- Propagate errors with the ? operator
- Create functions returning Result

## Prerequisites

- Completed Lab 5

---

## Setup

Create a new project:
```bash
cargo new lab06_errors
cd lab06_errors
code .
```

---

## Exercise 1: panic! (5 min)

### Step 1: Basic panic

```rust
fn main() {
    println!("Before panic");
    panic!("Something went wrong!");
    println!("This will never print");
}
```

Run with `cargo run` and observe the error output.

### Step 2: Array bounds panic

```rust
fn main() {
    let v = vec![1, 2, 3];

    // This will panic
    println!("{}", v[99]);
}
```

### Step 3: Get backtrace

Run with environment variable:
```bash
RUST_BACKTRACE=1 cargo run
```

---

## Exercise 2: Result and match (10 min)

### Step 1: Basic Result handling

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => {
            println!("File opened successfully");
            f
        }
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };

    println!("File: {:?}", file);
}
```

### Step 2: Handle different error types

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, creating it...");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("Could not create file: {:?}", e);
                    }
                }
            }
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    println!("File handle: {:?}", file);
}
```

### Step 3: Create your own Result-returning function

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Exercise 3: unwrap and expect (5 min)

### Step 1: Using unwrap

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // unwrap - panics on Err with default message
    let result = divide(10, 2).unwrap();
    println!("Result: {}", result);

    // This would panic:
    // let bad = divide(10, 0).unwrap();
}
```

### Step 2: Using expect

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // expect - panics with custom message
    let result = divide(10, 2).expect("Division failed");
    println!("Result: {}", result);

    // This would panic with our message:
    // let bad = divide(10, 0).expect("Division failed");
}
```

### Step 3: Using unwrap_or

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Provide default value on error
    let result1 = divide(10, 2).unwrap_or(0);
    let result2 = divide(10, 0).unwrap_or(0);

    println!("10 / 2 = {}", result1);  // 5
    println!("10 / 0 = {}", result2);  // 0 (default)
}
```

---

## Exercise 4: The ? Operator (10 min)

### Step 1: Without ? (verbose)

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_verbose(path: &str) -> Result<String, io::Error> {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_file_verbose("hello.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Step 2: With ? (concise)

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Step 3: Chaining with ?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_chained(path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_chained("hello.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Step 4: Using ? in main

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    // Create the file first
    std::fs::write("hello.txt", "Hello, Rust!")?;

    // Now read it
    let mut contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

    // Clean up
    std::fs::remove_file("hello.txt")?;

    Ok(())
}
```

---

## Exercise 5: Practical Error Handling (5 min)

### Complete example

```rust
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parse_port(config: &str) -> Result<u16, String> {
    config.trim()
        .parse()
        .map_err(|_| String::from("Invalid port number"))
}

fn get_server_port(config_path: &str) -> Result<u16, String> {
    let config = read_config(config_path)
        .map_err(|e| format!("Could not read config: {}", e))?;

    parse_port(&config)
}

fn main() {
    // Create a test config file
    fs::write("config.txt", "8080").expect("Could not create config");

    match get_server_port("config.txt") {
        Ok(port) => println!("Server will run on port {}", port),
        Err(e) => println!("Error: {}", e),
    }

    // Test with invalid config
    fs::write("config.txt", "not a number").expect("Could not create config");

    match get_server_port("config.txt") {
        Ok(port) => println!("Server will run on port {}", port),
        Err(e) => println!("Error: {}", e),
    }

    // Clean up
    fs::remove_file("config.txt").ok();
}
```

---

## Challenge Exercise (Bonus)

Create a function that:
1. Reads a file containing numbers (one per line)
2. Parses each line as an integer
3. Returns the sum of all numbers
4. Handles file errors and parsing errors appropriately

```rust
use std::fs;

fn sum_numbers_from_file(path: &str) -> Result<i32, String> {
    // Your implementation here
    Ok(0)
}

fn main() {
    // Create test file
    fs::write("numbers.txt", "10\n20\n30\n40").unwrap();

    match sum_numbers_from_file("numbers.txt") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }

    fs::remove_file("numbers.txt").ok();
}
```

---

## Verification Checklist

- [ ] Used panic! to crash the program
- [ ] Handled Result with match
- [ ] Created a function returning Result
- [ ] Used unwrap and expect
- [ ] Used unwrap_or for default values
- [ ] Used the ? operator for propagation
- [ ] Chained operations with ?
- [ ] Used ? in main() with Result return type

---

## Summary

You have learned:
- `panic!` for unrecoverable errors (crashes the program)
- `Result<T, E>` for recoverable errors (`Ok` or `Err`)
- `match` for explicit error handling
- `unwrap()` and `expect()` for quick prototyping (panic on error)
- `unwrap_or()` for providing default values
- `?` for concise error propagation
- Functions can return `Result` to let callers handle errors

**Next:** Lab 7 - Structs and Methods
