# Module 8: Error Handling

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**Error Handling**

- Robust Programs in Rust
- Module 8 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand Rust's error handling philosophy
- Use panic! for unrecoverable errors
- Work with Result<T, E> for recoverable errors
- Propagate errors with the ? operator
- Create custom error types
- Know when to panic vs return Result

---

## Slide 3: Two Types of Errors

```
┌─────────────────────────────────────────────┐
│            Error Categories                  │
├─────────────────────┬───────────────────────┤
│   Unrecoverable     │     Recoverable       │
├─────────────────────┼───────────────────────┤
│ • Bug in code       │ • Expected failures   │
│ • Invalid state     │ • File not found      │
│ • Index out of      │ • Network timeout     │
│   bounds            │ • Invalid input       │
├─────────────────────┼───────────────────────┤
│     panic!          │     Result<T, E>      │
└─────────────────────┴───────────────────────┘
```

---

## Slide 4: The panic! Macro

**For unrecoverable errors:**

```rust
fn main() {
    panic!("crash and burn!");
}
```

**Output:**
```
thread 'main' panicked at 'crash and burn!', src/main.rs:2:5
```

**What happens:**
1. Prints error message
2. Unwinds the stack
3. Cleans up memory
4. Exits the program

---

## Slide 5: When to panic!

**Use panic! when:**
- Bug in your code (shouldn't happen)
- Impossible to continue safely
- In examples and prototypes
- In tests (to fail the test)

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Panics: index out of bounds
    v[99];
}
```

---

## Slide 6: RUST_BACKTRACE

**Get detailed error information:**

```bash
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds', src/main.rs:4:5
stack backtrace:
   0: std::panicking::begin_panic
   1: example::main
             at ./src/main.rs:4
   ...
```

**Set `RUST_BACKTRACE=1` for full backtrace**

---

## Slide 7: The Result Enum

**For recoverable errors:**

```rust
enum Result<T, E> {
    Ok(T),   // Success with value of type T
    Err(E),  // Error with value of type E
}
```

**Example:**
```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");
    // Type: Result<File, std::io::Error>
}
```

---

## Slide 8: Handling Result

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };

    // Use file here
}
```

---

## Slide 9: Matching Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Cannot create file: {:?}", e),
                }
            }
            other_error => panic!("Cannot open file: {:?}", other_error),
        },
    };
}
```

---

## Slide 10: unwrap and expect

**Shortcuts that panic on Err:**

```rust
use std::fs::File;

fn main() {
    // Panics with default message if Err
    let file1 = File::open("hello.txt").unwrap();

    // Panics with custom message if Err
    let file2 = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}
```

**Use for:**
- Prototyping
- When you're certain it won't fail
- When failure means a bug

---

## Slide 11: unwrap_or and unwrap_or_default

**Provide fallback values:**

```rust
fn main() {
    let result: Result<i32, &str> = Err("error");

    // Provide default value
    let value1 = result.unwrap_or(0);  // Returns 0

    // Use type's default
    let value2: i32 = result.unwrap_or_default();  // Returns 0

    let ok_result: Result<i32, &str> = Ok(42);
    let value3 = ok_result.unwrap_or(0);  // Returns 42
}
```

---

## Slide 12: Propagating Errors

**Return errors to the caller:**

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

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
```

---

## Slide 13: The ? Operator

**Shorthand for error propagation:**

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

**`?` returns early with Err, or unwraps Ok**

---

## Slide 14: Chaining with ?

**Even more concise:**

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

---

## Slide 15: Using fs::read_to_string

**Standard library convenience:**

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

**Many common operations have convenience functions**

---

## Slide 16: ? with Option

**Works with Option too:**

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let result = last_char_of_first_line("hello\nworld");
    println!("{:?}", result);  // Some('o')

    let empty = last_char_of_first_line("");
    println!("{:?}", empty);   // None
}
```

---

## Slide 17: ? in main()

**main can return Result:**

```rust
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let file = File::open("hello.txt")?;
    // Use file...
    Ok(())
}
```

**Exit codes:**
- `Ok(())` → exit code 0
- `Err(_)` → exit code non-zero

---

## Slide 18: Converting Error Types

**When error types don't match:**

```rust
use std::num::ParseIntError;

fn parse_and_add(s: &str) -> Result<i32, ParseIntError> {
    let num: i32 = s.parse()?;
    Ok(num + 1)
}

fn main() {
    match parse_and_add("42") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Slide 19: Creating Custom Errors

```rust
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn might_fail(fail: bool) -> Result<(), CustomError> {
    if fail {
        Err(CustomError {
            message: String::from("Something went wrong"),
        })
    } else {
        Ok(())
    }
}
```

---

## Slide 20: Option vs Result

| Option<T> | Result<T, E> |
|-----------|--------------|
| Something or nothing | Success or failure |
| No error info | Contains error info |
| `Some(T)` / `None` | `Ok(T)` / `Err(E)` |
| Finding elements | I/O operations |
| Optional values | Computations that can fail |

```rust
let found: Option<i32> = vec.iter().find(|&&x| x > 5);
let result: Result<File, Error> = File::open("file.txt");
```

---

## Slide 21: Converting Option to Result

```rust
fn main() {
    let opt: Option<i32> = Some(42);

    // Convert Option to Result
    let result: Result<i32, &str> = opt.ok_or("no value");

    let none_opt: Option<i32> = None;
    let err_result: Result<i32, &str> = none_opt.ok_or("no value");
    // Returns Err("no value")
}
```

---

## Slide 22: When to Panic vs Result

**Use panic! when:**
- Contract violation (bug in calling code)
- Unrecoverable state
- Examples, prototypes, tests

**Use Result when:**
- Expected failure modes
- Caller can handle the error
- Library code (let caller decide)

```rust
// Panic: invalid input is a bug
fn get_index(v: &[i32], i: usize) -> i32 { v[i] }

// Result: file might not exist
fn read_file(path: &str) -> Result<String, io::Error> { ... }
```

---

## Slide 23: Key Takeaways

1. **panic!** for unrecoverable errors (bugs)
2. **Result<T, E>** for recoverable errors
3. **?** propagates errors concisely
4. **unwrap/expect** are shortcuts that panic
5. **unwrap_or** provides fallback values
6. **Option** for absence, **Result** for failures
7. **Libraries should return Result**, let callers decide

---

## Slide 24: Lab Preview

**Lab 8: Error Handling** (25 minutes)

You will:
- Use panic! for unrecoverable errors
- Handle Result with match
- Use unwrap, expect, and unwrap_or
- Propagate errors with ?
- Create functions that return Result
- Convert between Option and Result

---

## Questions?

**Next Module:** Collections and Iterators
