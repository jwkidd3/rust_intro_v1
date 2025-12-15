# Lab 8: Error Handling

## Duration: 25 minutes

## Objectives
- Understand recoverable vs unrecoverable errors
- Use panic! for unrecoverable errors
- Use Result<T, E> for recoverable errors
- Master the ? operator for error propagation

## Prerequisites
- Completed Lab 7
- Understanding of enums and Option

## Part 1: Unrecoverable Errors (5 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab08_errors
cd lab08_errors
```

### Exercise 2: Using panic!

Edit `src/main.rs`:

```rust
fn main() {
    // Explicit panic
    // panic!("crash and burn");

    // Implicit panic - index out of bounds
    let v = vec![1, 2, 3];
    // v[99];  // This would panic

    // When to panic:
    // - Unrecoverable situations
    // - Programming bugs (invariant violations)
    // - Prototyping (use unwrap/expect)

    println!("Program completed successfully");
}
```

### Exercise 3: unwrap and expect

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    // unwrap: panics if None/Err with generic message
    let first = numbers.get(0).unwrap();
    println!("First: {}", first);

    // expect: panics with custom message
    let second = numbers.get(1).expect("Should have at least 2 elements");
    println!("Second: {}", second);

    // These are useful for:
    // - Prototyping
    // - Tests
    // - When you're sure the value exists

    // In production, prefer proper error handling
}
```

## Part 2: Recoverable Errors with Result (10 minutes)

### Exercise 4: Understanding Result

```rust
use std::fs::File;

fn main() {
    // Result is defined as:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => {
            println!("File opened successfully");
            f
        }
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };
}
```

### Exercise 5: Matching Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("Created new file");
                        fc
                    }
                    Err(e) => {
                        panic!("Could not create file: {}", e);
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

### Exercise 6: Result Methods

```rust
fn main() {
    // unwrap_or: provide default on error
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or(0);
    println!("unwrap_or: {}", value);

    // Handle error with match to compute default
    let result: Result<i32, &str> = Err("error");
    let value = match result {
        Ok(v) => v,
        Err(e) => {
            println!("Error was: {}", e);
            -1
        }
    };
    println!("error handled: {}", value);

    // Transform Result value using match
    let result: Result<i32, &str> = Ok(5);
    let doubled = match result {
        Ok(x) => Ok(x * 2),
        Err(e) => Err(e),
    };
    println!("doubled: {:?}", doubled);

    // is_ok and is_err
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("error");
    println!("ok.is_ok(): {}", ok.is_ok());
    println!("err.is_err(): {}", err.is_err());
}
```

## Part 3: Error Propagation (10 minutes)

### Exercise 7: Manual Error Propagation

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error reading username: {}", e),
    }
}
```

### Exercise 8: The ? Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;  // ? propagates error
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter with chaining
fn read_username_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Shortest with fs::read_to_string
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Exercise 9: ? with Option

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let text = "Hello\nWorld";
    match last_char_of_first_line(text) {
        Some(c) => println!("Last char of first line: {}", c),
        None => println!("No characters found"),
    }

    let empty = "";
    println!("Empty text result: {:?}", last_char_of_first_line(empty));
}
```

### Exercise 10: Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "cannot take square root of negative"),
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn main() {
    println!("10 / 2 = {:?}", divide(10.0, 2.0));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));
    println!("sqrt(16) = {:?}", sqrt(16.0));
    println!("sqrt(-4) = {:?}", sqrt(-4.0));
}
```

### Exercise 11: Practical Error Handling

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::IoError(err)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> ConfigError {
        ConfigError::ParseError(err)
    }
}

fn read_config() -> Result<i32, ConfigError> {
    let contents = fs::read_to_string("config.txt")?;
    let value = contents.trim().parse::<i32>()?;
    Ok(value)
}

fn main() {
    // Create a test config file
    fs::write("config.txt", "42").expect("Unable to write config");

    match read_config() {
        Ok(value) => println!("Config value: {}", value),
        Err(ConfigError::IoError(e)) => println!("IO Error: {}", e),
        Err(ConfigError::ParseError(e)) => println!("Parse Error: {}", e),
    }

    // Clean up
    fs::remove_file("config.txt").ok();
}
```

## Verification Steps

### Checklist
- [ ] Used panic! for unrecoverable errors
- [ ] Used unwrap and expect appropriately
- [ ] Matched on Result variants (Ok/Err)
- [ ] Used Result methods (unwrap_or, map, etc.)
- [ ] Used the ? operator for error propagation
- [ ] Created custom error types
- [ ] Converted between error types with From

## Lab Questions

1. When should you use panic! vs Result?
2. What does the ? operator do?
3. What is the difference between unwrap() and expect()?
4. How do you convert between different error types?

## Answers

1. Use **panic!** for unrecoverable errors, bugs, or when continuing would be unsafe. Use **Result** for expected failures the caller should handle (file not found, invalid input, network errors).

2. The **?** operator unwraps Ok values or returns early with the Err. It also converts error types if the From trait is implemented.

3. Both panic on Err/None. **unwrap()** gives a generic message. **expect()** lets you provide a custom message explaining what went wrong.

4. Implement the **From** trait to enable automatic conversion with ?, or use **map_err()** for inline conversion.

## Common Issues

**Issue: "the ? operator can only be used in a function that returns Result"**
```
Solution: Change your function signature to return Result<T, E> or use match/if let instead.
```

**Issue: "cannot use the ? operator in a function that returns ()"**
```
Solution: In main, you can return Result<(), Box<dyn Error>> or handle errors with match.
```

## Next Steps

In Lab 9, you will:
- Work with Vec, String, and HashMap
- Use iterators and closures
- Chain iterator methods

## Completion

You have completed Lab 8 when you can:
- Choose between panic! and Result appropriately
- Use the ? operator for concise error propagation
- Handle multiple error types in a function
- Create and use custom error types
