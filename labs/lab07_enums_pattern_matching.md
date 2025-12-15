# Lab 7: Enums and Pattern Matching

## Duration: 25 minutes

## Objectives
- Define enums with various variant types
- Use pattern matching with match
- Work with the Option enum
- Use if let for concise matching

## Prerequisites
- Completed Lab 6
- Understanding of structs

## Part 1: Defining Enums (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab07_enums
cd lab07_enums
```

### Exercise 2: Basic Enum Definition

Edit `src/main.rs`:

```rust
// Simple enum with unit variants
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;

    // Use match to handle all variants
    match dir {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }
}
```

### Exercise 3: Enums with Data

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Multiple values (like tuple)
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages {
        process_message(msg);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message received");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

### Exercise 4: Enum Methods

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(s) => println!("Writing: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to ({}, {}, {})", r, g, b)
            }
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    msg.call();

    let msg2 = Message::Move { x: 5, y: 10 };
    msg2.call();
}
```

## Part 2: The Option Enum (10 minutes)

### Exercise 5: Understanding Option

```rust
fn main() {
    // Option is defined in standard library:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}
```

### Exercise 6: Working with Option

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // get() returns Option<&T>
    let third = numbers.get(2);
    let tenth = numbers.get(10);

    match third {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element"),
    }

    match tenth {
        Some(value) => println!("Tenth element: {}", value),
        None => println!("No tenth element"),
    }
}
```

### Exercise 7: Option Methods

```rust
fn main() {
    let some_value: Option<i32> = Some(5);
    let no_value: Option<i32> = None;

    // unwrap_or: provide default value
    println!("some: {}", some_value.unwrap_or(0));
    println!("none: {}", no_value.unwrap_or(0));

    // is_some and is_none
    println!("some_value is_some: {}", some_value.is_some());
    println!("no_value is_none: {}", no_value.is_none());

    // Transform Option value using match
    let doubled = match some_value {
        Some(x) => Some(x * 2),
        None => None,
    };
    println!("doubled: {:?}", doubled);
}
```

### Exercise 8: Practical Option Usage

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        3 => Some(String::from("Charlie")),
        _ => None,
    }
}

fn main() {
    for id in 1..=5 {
        match find_user(id) {
            Some(name) => println!("User {}: {}", id, name),
            None => println!("User {} not found", id),
        }
    }
}
```

## Part 3: Pattern Matching (5 minutes)

### Exercise 9: Match with Catch-All

```rust
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  // Catch-all binding
    }

    let dice_roll = 5;
    match dice_roll {
        3 => println!("Three!"),
        7 => println!("Seven!"),
        _ => (),  // _ ignores the value, () does nothing
    }
}

fn add_fancy_hat() {
    println!("Adding fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn move_player(spaces: u8) {
    println!("Moving {} spaces", spaces);
}
```

### Exercise 10: if let for Concise Matching

```rust
fn main() {
    let config_max: Option<u8> = Some(3);

    // Verbose match
    match config_max {
        Some(max) => println!("Maximum is {}", max),
        _ => (),
    }

    // Concise if let
    if let Some(max) = config_max {
        println!("Maximum is {}", max);
    }

    // if let with else
    let coin = Coin::Quarter;
    if let Coin::Quarter = coin {
        println!("It's a quarter!");
    } else {
        println!("Not a quarter");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
```

### Exercise 11: Pattern Matching in Functions

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("Circle with radius {}", radius),
            Shape::Rectangle { width, height } => {
                format!("Rectangle {}x{}", width, height)
            }
            Shape::Triangle { base, height } => {
                format!("Triangle with base {} and height {}", base, height)
            }
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    for shape in &shapes {
        println!("{}: area = {:.2}", shape.describe(), shape.area());
    }
}
```

## Verification Steps

### Checklist
- [ ] Defined enums with unit variants
- [ ] Defined enums with data in variants
- [ ] Implemented methods on enums
- [ ] Used Option with Some and None
- [ ] Used match to handle all enum variants
- [ ] Used catch-all patterns (_ and other)
- [ ] Used if let for single-pattern matching
- [ ] Extracted data from enum variants in match arms

## Lab Questions

1. What is the difference between `_` and a named catch-all like `other`?
2. Why does Rust have Option instead of null?
3. What happens if you don't handle all variants in a match?
4. When should you use if let instead of match?

## Answers

1. `_` ignores the value entirely. A named catch-all like `other` binds the value so you can use it in the match arm.

2. **Option** makes absence explicit in the type system. The compiler forces you to handle the None case, preventing null pointer exceptions at compile time rather than runtime.

3. Rust's match is **exhaustive** - the compiler will error if you don't handle all variants. Use `_` or `other` as a catch-all for remaining cases.

4. Use **if let** when you only care about one pattern and want to ignore the rest. Use **match** when you need to handle multiple patterns or want exhaustiveness checking.

## Common Issues

**Issue: "non-exhaustive patterns"**
```
Solution: Add a catch-all arm with _ => or handle all variants explicitly.
```

**Issue: "cannot move out of borrowed content"**
```
Solution: Use ref in the pattern: Some(ref value) or match on a reference.
```

## Next Steps

In Lab 8, you will:
- Work with Result for error handling
- Use the ? operator
- Create custom error types

## Completion

You have completed Lab 7 when you can:
- Define enums with various data types
- Use match for exhaustive pattern matching
- Work with Option<T> confidently
- Choose between match and if let appropriately
