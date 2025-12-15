# Module 7: Enums and Pattern Matching

**Duration:** 50 minutes
**Type:** Presentation

---

## Slide 1: Title

**Enums and Pattern Matching**

- Powerful Type System Features
- Module 7 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Define and use enums
- Attach data to enum variants
- Understand Option<T> for null safety
- Use match expressions for pattern matching
- Apply if let for concise matching
- Work with common enum patterns

---

## Slide 3: What is an Enum?

**A type with a fixed set of possible values:**

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let heading = Direction::North;
}
```

**Each possible value is called a "variant"**

---

## Slide 4: Using Enums

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(direction: Direction) {
    // We'll handle this with match soon!
}

fn main() {
    let heading = Direction::North;
    move_player(heading);
    move_player(Direction::East);
}
```

---

## Slide 5: Enums with Data

**Variants can hold data:**

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Multiple values
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
}
```

---

## Slide 6: Enum vs Structs

**These are equivalent:**

```rust
// Using separate structs
struct QuitMessage;
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String);

// Using one enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

**Advantage of enum:** One type to handle all variants

---

## Slide 7: Methods on Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        // Handle the message
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

---

## Slide 8: The Option Enum

**Rust's replacement for null:**

```rust
// Defined in standard library
enum Option<T> {
    None,
    Some(T),
}
```

**No null in Rust! Use Option instead.**

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
}
```

---

## Slide 9: Why Option?

**The billion-dollar mistake: null pointers**

```rust
fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y;  // ERROR: can't add i32 and Option<i32>

    // You must handle the None case!
}
```

**Compiler forces you to handle absence of value**

---

## Slide 10: Using Option

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result = divide(10, 2);  // Some(5)
    let error = divide(10, 0);   // None
}
```

---

## Slide 11: Pattern Matching with match

**The match expression:**

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

## Slide 12: Match is Exhaustive

**Must handle all variants:**

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe(d: Direction) -> &'static str {
    match d {
        Direction::North => "going north",
        Direction::South => "going south",
        // ERROR: missing East and West!
    }
}
```

**Compiler error if patterns are incomplete**

---

## Slide 13: Match with Binding

**Extract data from variants:**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
    }
}
```

---

## Slide 14: Matching Option

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);   // Some(6)
    let none = plus_one(None);  // None
}
```

---

## Slide 15: The Catch-All Pattern

**Use `_` for remaining cases:**

```rust
fn describe_number(n: u32) {
    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }
}
```

**`_` matches anything and doesn't bind**

---

## Slide 16: Binding with other

**Catch-all that uses the value:**

```rust
fn describe_number(n: u32) {
    match n {
        1 => println!("one"),
        2 => println!("two"),
        other => println!("number is {}", other),
    }
}
```

---

## Slide 17: Match Arms with Code Blocks

```rust
fn process(x: Option<i32>) {
    match x {
        None => {
            println!("Got nothing");
            // Can have multiple statements
        }
        Some(n) => {
            println!("Got a number");
            println!("The number is: {}", n);
        }
    }
}
```

---

## Slide 18: if let Syntax

**Concise handling of one pattern:**

```rust
fn main() {
    let some_value = Some(3);

    // Verbose match
    match some_value {
        Some(x) => println!("Got: {}", x),
        _ => (),
    }

    // Concise if let
    if let Some(x) = some_value {
        println!("Got: {}", x);
    }
}
```

---

## Slide 19: if let with else

```rust
fn main() {
    let coin = Coin::Quarter;

    // Using match
    match coin {
        Coin::Quarter => println!("Quarter!"),
        _ => println!("Not a quarter"),
    }

    // Using if let with else
    if let Coin::Quarter = coin {
        println!("Quarter!");
    } else {
        println!("Not a quarter");
    }
}
```

---

## Slide 20: while let

**Loop while pattern matches:**

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

**Output:** 3, 2, 1

---

## Slide 21: Matching Multiple Patterns

**Use `|` for multiple patterns:**

```rust
fn describe(n: i32) {
    match n {
        1 | 2 | 3 => println!("small"),
        4 | 5 | 6 => println!("medium"),
        _ => println!("large"),
    }
}
```

---

## Slide 22: Matching Ranges

**Use `..=` for inclusive ranges:**

```rust
fn describe(n: i32) {
    match n {
        1..=3 => println!("small"),
        4..=6 => println!("medium"),
        _ => println!("large"),
    }
}

fn letter_type(c: char) {
    match c {
        'a'..='z' => println!("lowercase"),
        'A'..='Z' => println!("uppercase"),
        _ => println!("other"),
    }
}
```

---

## Slide 23: Match Guards

**Add conditions with `if`:**

```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than 5: {}", x),
        Some(x) => println!("greater or equal to 5: {}", x),
        None => println!("no value"),
    }
}
```

---

## Slide 24: Key Takeaways

1. **Enums** define types with a fixed set of variants
2. **Variants can hold data** of different types
3. **Option<T>** replaces null - forces handling of absence
4. **match** is exhaustive - must handle all variants
5. **`_`** is catch-all, `other` catches and binds
6. **if let** is concise for single-pattern matching
7. **Patterns can have guards** with `if`

---

## Slide 25: Lab Preview

**Lab 7: Enums and Pattern Matching** (30 minutes)

You will:
- Create enums with various variant types
- Use Option<T> for optional values
- Write match expressions
- Use if let for concise matching
- Implement enum methods
- Work with nested patterns

---

## Questions?

**Next Module:** Error Handling
