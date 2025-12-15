# Module 12: Lifetimes and Testing

**Duration:** 55 minutes
**Type:** Presentation

---

## Slide 1: Title

**Lifetimes and Testing**

- Advanced Concepts and Quality Assurance
- Module 12 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand why lifetimes exist
- Write lifetime annotations
- Apply lifetime elision rules
- Write unit tests with #[test]
- Use common test macros
- Organize tests in modules

---

## Slide 3: Why Lifetimes?

**The Problem:**

```rust
fn main() {
    let r;                // ------+-- 'a
    {                     //       |
        let x = 5;        // -+-- 'b
        r = &x;           //  |    |
    }                     // -+    |
    // println!("{}", r); //       | ERROR: x doesn't live long enough
}                         // ------+
```

**Lifetimes prevent dangling references**

---

## Slide 4: Lifetime Annotations

**Syntax: `'a` (tick followed by name)**

```rust
&i32        // a reference
&'a i32     // a reference with explicit lifetime
&'a mut i32 // a mutable reference with explicit lifetime
```

**Lifetimes don't change how long references live - they describe relationships**

---

## Slide 5: Lifetime in Functions

**Problem:**

```rust
// ERROR: missing lifetime specifier
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Compiler doesn't know if return references x or y**

---

## Slide 6: Adding Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("xyz");

    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

---

## Slide 7: What Lifetime Annotations Mean

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

**This says:**
- Both `x` and `y` live at least as long as `'a`
- The return value lives at least as long as `'a`
- In practice: return lives as long as the shorter input

---

## Slide 8: Lifetime Scope Example

```rust
fn main() {
    let s1 = String::from("long string");
    let result;

    {
        let s2 = String::from("xyz");
        result = longest(&s1, &s2);
        println!("Longest: {}", result);  // OK: s2 still in scope
    }

    // println!("Longest: {}", result);  // ERROR: s2 dropped
}
```

---

## Slide 9: Lifetimes in Structs

**When structs hold references:**

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", excerpt.part);
}
```

---

## Slide 10: Lifetime Elision Rules

**Compiler infers lifetimes in common cases:**

```rust
// You write:
fn first_word(s: &str) -> &str { ... }

// Compiler sees:
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

**Three elision rules:**
1. Each input reference gets its own lifetime
2. If one input lifetime, output gets that lifetime
3. If `&self` or `&mut self`, output gets self's lifetime

---

## Slide 11: Elision Rule Examples

```rust
// Rule 1: One lifetime per input
fn foo(x: &str, y: &str) { }
// Becomes: fn foo<'a, 'b>(x: &'a str, y: &'b str)

// Rule 2: One input → same output
fn foo(x: &str) -> &str { }
// Becomes: fn foo<'a>(x: &'a str) -> &'a str

// Rule 3: Methods use self's lifetime
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }
    fn announce(&self, announcement: &str) -> &str {
        self.part
    }
}
```

---

## Slide 12: The 'static Lifetime

**Lives for entire program duration:**

```rust
// String literals have 'static lifetime
let s: &'static str = "I live forever!";
```

**Use sparingly - usually there's a better solution**

---

## Slide 13: Generics, Traits, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## Slide 14: Introduction to Testing

**Rust has built-in testing support:**

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

**Run tests with: `cargo test`**

---

## Slide 15: Writing Your First Test

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

---

## Slide 16: Test Assertions

```rust
#[test]
fn test_assertions() {
    // Basic assertions
    assert!(true);
    assert_eq!(4, 2 + 2);
    assert_ne!(4, 3);

    // With custom messages
    assert!(true, "This should be true");
    assert_eq!(4, 2 + 2, "Math is broken: {} != {}", 4, 2+2);
}
```

---

## Slide 17: Testing for Panics

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_panic_message() {
        divide(10, 0);
    }
}
```

---

## Slide 18: Testing with Result

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken!"))
        }
    }
}
```

**Return `Ok(())` for pass, `Err` for fail**

---

## Slide 19: Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with name containing string
cargo test add

# Show output (normally captured)
cargo test -- --show-output

# Run tests sequentially (not in parallel)
cargo test -- --test-threads=1
```

---

## Slide 20: Ignoring Tests

```rust
#[test]
#[ignore]
fn expensive_test() {
    // This test takes a long time
}
```

```bash
# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

---

## Slide 21: Test Organization

```rust
// Unit tests - in same file as code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_test() {
        // Can test private functions
    }
}

// Integration tests - in tests/ directory
// tests/integration_test.rs
use my_crate;

#[test]
fn external_test() {
    // Can only test public API
}
```

---

## Slide 22: Integration Tests

**Project structure:**
```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

```rust
// tests/integration_test.rs
use my_project;

#[test]
fn test_public_api() {
    assert_eq!(my_project::add(2, 3), 5);
}
```

---

## Slide 23: Test Helper Modules

```rust
// tests/common/mod.rs
pub fn setup() {
    // Common setup code
}

// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    common::setup();
    // Test code
}
```

---

## Slide 24: Documentation Tests

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Doc examples are automatically tested!**

```bash
cargo test --doc
```

---

## Slide 25: Test Best Practices

```
┌─────────────────────────────────────────────┐
│           Testing Best Practices            │
├─────────────────────────────────────────────┤
│ • Test one thing per test                   │
│ • Use descriptive test names                │
│ • Test edge cases                           │
│ • Test error conditions                     │
│ • Keep tests fast                           │
│ • Use setup/teardown for common code        │
│ • Test public API (integration tests)       │
│ • Test internals (unit tests)               │
└─────────────────────────────────────────────┘
```

---

## Slide 26: Key Takeaways

1. **Lifetimes** prevent dangling references
2. **`'a`** syntax annotates lifetime relationships
3. **Elision rules** let compiler infer common cases
4. **`'static`** lives for program duration
5. **`#[test]`** marks test functions
6. **`assert!`, `assert_eq!`, `assert_ne!`** for assertions
7. **`#[should_panic]`** for panic tests
8. **Integration tests** go in `tests/` directory

---

## Slide 27: Lab Preview

**Lab 12: Lifetimes and Testing** (30 minutes)

You will:
- Write functions with lifetime annotations
- Create structs that hold references
- Write unit tests with assertions
- Test for expected panics
- Organize tests in modules
- Create integration tests

---

## Slide 28: Course Summary

**What You've Learned:**

- Rust fundamentals (variables, functions, control flow)
- Ownership, borrowing, and references
- Structs, enums, and pattern matching
- Error handling with Result and Option
- Collections and iterators
- Modules and crates
- Generics, traits, and lifetimes
- Testing

**You're now ready to write safe, fast Rust code!**

---

## Slide 29: Next Steps

**Continue Learning:**
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings: https://github.com/rust-lang/rustlings
- Exercism Rust Track: https://exercism.org/tracks/rust

**Build Projects:**
- Command-line tools
- Web services (actix-web, axum)
- Systems programming
- WebAssembly

---

## Questions?

**Thank you for completing Introduction to Rust!**

**Resources:**
- The Rust Book: https://doc.rust-lang.org/book/
- Official Website: https://www.rust-lang.org/
- Community: https://www.rust-lang.org/community
