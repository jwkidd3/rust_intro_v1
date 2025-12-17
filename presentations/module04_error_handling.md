# Module 4: Error Handling

**Duration:** 30 minutes

---

## Slide 1: Title

**Error Handling**

- Robust Programs in Rust
- Module 4 of 6

---

## Slide 2: Overview

**Two types of errors in Rust:**

```
┌─────────────────────────────────────────────┐
│            Error Categories                  │
├─────────────────────┬───────────────────────┤
│   Unrecoverable     │     Recoverable       │
├─────────────────────┼───────────────────────┤
│ • Bug in code       │ • Expected failures   │
│ • Invalid state     │ • File not found      │
│ • Should not happen │ • Network timeout     │
├─────────────────────┼───────────────────────┤
│     panic!          │     Result<T, E>      │
└─────────────────────┴───────────────────────┘
```

---

## Slide 3: Unrecoverable Errors - panic!

```rust
fn main() {
    panic!("crash and burn!");
}
```

**Output:**
```
thread 'main' panicked at 'crash and burn!', src/main.rs:2:5
```

**When to use:**
- Bug in code that shouldn't happen
- Impossible to continue safely
- In tests (to fail the test)

---

## Slide 4: panic! Examples

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Panics: index out of bounds
    v[99];
}
```

**Get backtrace:**
```bash
RUST_BACKTRACE=1 cargo run
```

---

## Slide 5: Recoverable Errors - Result

```rust
enum Result<T, E> {
    Ok(T),   // Success with value
    Err(E),  // Error with value
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

## Slide 6: Handling Result with match

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
}
```

---

## Slide 7: unwrap and expect

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

---

## Slide 8: unwrap_or

**Provide fallback values:**

```rust
fn main() {
    let result: Result<i32, &str> = Err("error");

    let value = result.unwrap_or(0);  // Returns 0

    let ok_result: Result<i32, &str> = Ok(42);
    let value2 = ok_result.unwrap_or(0);  // Returns 42
}
```

---

## Slide 9: The ? Operator

**Shorthand for error propagation:**

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**`?` returns early with Err, or unwraps Ok**

---

## Slide 10: ? vs match

```rust
// With match (verbose)
fn read_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ...
}

// With ? (concise)
fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    // ...
}
```

---

## Slide 11: Chaining with ?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}
```

---

## Slide 12: ? in main()

**main can return Result:**

```rust
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let file = File::open("hello.txt")?;
    // ...
    Ok(())
}
```

---

## Slide 13: Creating Functions that Return Result

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
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Slide 14: When to panic! vs Result

**Use panic! when:**
- Bug in your code
- Contract violation
- Examples and tests

**Use Result when:**
- Expected failure modes
- Caller can handle the error
- Library code

```rust
// Panic: invalid input is a bug
fn get_index(v: &[i32], i: usize) -> i32 { v[i] }

// Result: file might not exist
fn read_file(path: &str) -> Result<String, io::Error> { ... }
```

---

## Slide 15: Best Practices

1. **Use `?`** for clean error propagation
2. **Use `expect()`** with descriptive messages during development
3. **Return `Result`** from functions that can fail
4. **Let callers decide** how to handle errors
5. **panic!** only for programming errors

---

## Slide 16: Key Takeaways

1. **panic!** for unrecoverable errors (bugs)
2. **Result<T, E>** for recoverable errors
3. **match** for explicit error handling
4. **unwrap/expect** for quick prototyping
5. **?** for concise error propagation
6. **Functions should return Result** when they can fail

---

## Slide 17: Lab Preview

**Lab 4: Error Handling** (35 min)

You will:
- Use panic! for unrecoverable errors
- Handle errors with Result and match
- Use unwrap and expect
- Propagate errors with ?
- Create functions returning Result

---

## Questions?
