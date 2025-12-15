# Lab 12: Lifetimes and Testing

## Duration: 30 minutes

## Objectives
- Understand lifetime annotations
- Apply lifetime elision rules
- Write unit tests with #[test]
- Create integration tests
- Use assert macros

## Prerequisites
- Completed Lab 11
- Understanding of references and borrowing

## Part 1: Lifetimes (15 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab12_lifetimes
cd lab12_lifetimes
```

### Exercise 2: Why Lifetimes Exist

Edit `src/main.rs`:

```rust
fn main() {
    // This would be a dangling reference:
    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // x doesn't live long enough
    // }
    // println!("{}", r);  // ERROR: r references dropped value

    // Correct version:
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
```

### Exercise 3: Lifetime Annotations

```rust
// Without lifetime annotation - won't compile
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// With lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);
    }

    // This works because result's lifetime is limited to inner scope
    // where both string1 and string2 are valid
}
```

### Exercise 4: Different Lifetimes

```rust
// Return lifetime tied only to first parameter
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Multiple lifetime parameters
fn multi<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y: {}", y);
    x
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let result = first(&s1, &s2);
    println!("First: {}", result);

    let result = multi(&s1, &s2);
    println!("Multi: {}", result);
}
```

### Exercise 5: Structs with Lifetimes

```rust
// Struct holding a reference needs lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Lifetime elision: output lifetime = &self lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());
    println!("Part: {}", excerpt.announce_and_return_part("Important!"));
}
```

### Exercise 6: Lifetime Elision Rules

```rust
// Rule 1: Each reference parameter gets its own lifetime
// fn foo(x: &str) -> fn foo<'a>(x: &'a str)

// Rule 2: If exactly one input lifetime, assign to all outputs
// fn foo(x: &str) -> &str  becomes  fn foo<'a>(x: &'a str) -> &'a str

// Rule 3: If &self or &mut self, self's lifetime is assigned to outputs

// These don't need explicit lifetimes due to elision:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);
}
```

### Exercise 7: Static Lifetime

```rust
fn main() {
    // 'static lifetime: lives for entire program
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // String literals are always 'static
    let hello: &'static str = "Hello, world!";

    // Be careful: 'static means data lives forever,
    // not that the reference is valid forever
}
```

## Part 2: Testing (15 minutes)

### Exercise 8: Unit Tests

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
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
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), None);
    }
}
```

Run tests:
```bash
cargo test
```

### Exercise 9: More Assert Macros

```rust
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_area() {
        let rect = Rectangle { width: 10, height: 5 };
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn test_not_equal() {
        assert_ne!(2 + 2, 5);
    }
}
```

### Exercise 10: Testing Panics and Results

```rust
pub fn divide_strict(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_divide_by_zero_panics() {
        divide_strict(10, 0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero_message() {
        divide_strict(10, 0);
    }

    #[test]
    fn test_parse_valid_number() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("42")?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = parse_number("not a number");
        assert!(result.is_err());
    }
}
```

### Exercise 11: Integration Tests

Create a library crate:
```bash
cargo new --lib mylib
cd mylib
```

**src/lib.rs:**
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub mod utils {
    pub fn is_positive(n: i32) -> bool {
        n > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

**Create tests directory and file:**
```bash
mkdir tests
```

**tests/integration_test.rs:**
```rust
use mylib;

#[test]
fn integration_test_add() {
    assert_eq!(mylib::add(2, 3), 5);
}

#[test]
fn integration_test_multiply() {
    assert_eq!(mylib::multiply(3, 4), 12);
}

#[test]
fn integration_test_utils() {
    assert!(mylib::utils::is_positive(5));
    assert!(!mylib::utils::is_positive(-5));
}

mod common;

#[test]
fn test_with_setup() {
    common::setup();
    assert_eq!(mylib::add(1, 1), 2);
}
```

**tests/common/mod.rs:**
```rust
pub fn setup() {
    // Common setup code for tests
    println!("Setting up test environment");
}
```

Run tests:
```bash
cargo test
cargo test --test integration_test  # Run specific test file
```

### Exercise 12: Test Organization

```rust
pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0 }
    }

    pub fn add(&mut self, n: i32) -> &mut Self {
        self.value += n;
        self
    }

    pub fn subtract(&mut self, n: i32) -> &mut Self {
        self.value -= n;
        self
    }

    pub fn result(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut calc = Calculator::new();
    calc.add(5).add(3).subtract(2);
    println!("Result: {}", calc.result());
}

#[cfg(test)]
mod tests {
    use super::*;

    // Group related tests
    mod addition {
        use super::*;

        #[test]
        fn add_positive() {
            let mut calc = Calculator::new();
            calc.add(5);
            assert_eq!(calc.result(), 5);
        }

        #[test]
        fn add_negative() {
            let mut calc = Calculator::new();
            calc.add(-5);
            assert_eq!(calc.result(), -5);
        }
    }

    mod subtraction {
        use super::*;

        #[test]
        fn subtract_positive() {
            let mut calc = Calculator::new();
            calc.subtract(5);
            assert_eq!(calc.result(), -5);
        }
    }

    mod chaining {
        use super::*;

        #[test]
        fn chain_operations() {
            let mut calc = Calculator::new();
            calc.add(10).subtract(3).add(5);
            assert_eq!(calc.result(), 12);
        }
    }

    // Ignored test (run with: cargo test -- --ignored)
    #[test]
    #[ignore]
    fn expensive_test() {
        // This test takes a long time
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }
}
```

Run specific tests:
```bash
cargo test              # Run all tests
cargo test addition     # Run tests with "addition" in name
cargo test -- --ignored # Run ignored tests
cargo test -- --nocapture # Show println! output
```

## Verification Steps

### Checklist
- [ ] Understand why lifetimes are needed
- [ ] Can write lifetime annotations on functions
- [ ] Can add lifetimes to structs
- [ ] Understand lifetime elision rules
- [ ] Can write unit tests with #[test]
- [ ] Used assert!, assert_eq!, assert_ne!
- [ ] Tested panics with #[should_panic]
- [ ] Created integration tests in tests/ directory
- [ ] Organized tests with modules

## Lab Questions

1. What do lifetime annotations actually do?
2. What are the three lifetime elision rules?
3. What is the difference between unit tests and integration tests?
4. How do you run a specific test?

## Answers

1. Lifetime annotations **describe relationships** between lifetimes of references. They don't change how long values live; they help the compiler verify reference validity.

2. **Elision rules**: (1) Each input reference gets its own lifetime. (2) If one input lifetime, it's assigned to all outputs. (3) If `&self` or `&mut self`, self's lifetime is assigned to outputs.

3. **Unit tests** test private functions, live in the same file, use `#[cfg(test)]`. **Integration tests** test public API, live in `tests/` directory, test like external code.

4. Use `cargo test <name>` where name is any substring of the test function name.

## Common Issues

**Issue: "lifetime may not live long enough"**
```
Solution: Add explicit lifetime annotations showing the relationship between input and output lifetimes.
```

**Issue: "cannot find function in this scope" in tests**
```
Solution: Use `use super::*;` or import specific items from the parent module.
```

## Course Summary

Congratulations! You've completed the Introduction to Rust course. You've learned:
- Rust syntax and fundamentals
- Ownership, borrowing, and lifetimes
- Structs, enums, and pattern matching
- Error handling with Result
- Collections and iterators
- Modules and crates
- Generics and traits
- Testing

## Next Steps
- Build a CLI application
- Explore async Rust
- Learn about smart pointers
- Contribute to open source Rust projects

## Completion

You have completed Lab 12 when you can:
- Explain why lifetimes exist
- Write lifetime annotations when needed
- Apply elision rules to simplify code
- Write comprehensive tests for Rust code
