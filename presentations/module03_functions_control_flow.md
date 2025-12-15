# Module 3: Functions and Control Flow

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**Functions and Control Flow**

- Building Blocks of Rust Programs
- Module 3 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Define and call functions
- Understand parameters and return values
- Distinguish between statements and expressions
- Use if/else conditional expressions
- Work with loops (loop, while, for)

---

## Slide 3: Functions in Rust

**Function definition syntax:**

```rust
fn function_name(parameter: Type) -> ReturnType {
    // function body
}
```

**Example:**
```rust
fn main() {
    println!("Hello from main!");
    greet();
}

fn greet() {
    println!("Hello, Rustacean!");
}
```

---

## Slide 4: Function Naming Convention

**Use snake_case for functions:**

```rust
// Good
fn calculate_area() { }
fn get_user_name() { }
fn is_valid() { }

// Bad
fn calculateArea() { }   // camelCase
fn GetUserName() { }     // PascalCase
fn IS_VALID() { }        // SCREAMING_CASE
```

---

## Slide 5: Function Parameters

**Parameters must have type annotations:**

```rust
fn print_value(x: i32) {
    println!("The value is: {}", x);
}

fn print_point(x: i32, y: i32) {
    println!("Point: ({}, {})", x, y);
}

fn greet(name: &str, age: u32) {
    println!("{} is {} years old", name, age);
}

fn main() {
    print_value(42);
    print_point(3, 7);
    greet("Alice", 30);
}
```

---

## Slide 6: Return Values

**Use `->` to specify return type:**

```rust
fn five() -> i32 {
    5  // No semicolon - this is the return value
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = five();
    let sum = add(3, 4);
    println!("x = {}, sum = {}", x, sum);
}
```

---

## Slide 7: Return Keyword

**Early return with `return`:**

```rust
fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x;  // Early return
    }
    x  // Implicit return
}

fn find_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
```

---

## Slide 8: Statements vs Expressions

**Statements:** Perform actions, don't return values

**Expressions:** Evaluate to a value

```rust
fn main() {
    // Statement - ends with semicolon, no value
    let x = 5;

    // Expression - evaluates to a value
    let y = {
        let a = 3;
        a + 1  // No semicolon - returns 4
    };

    println!("y = {}", y);  // Prints: y = 4
}
```

---

## Slide 9: Everything is an Expression

**Most things in Rust are expressions:**

```rust
fn main() {
    // Function calls are expressions
    let x = add(2, 3);

    // Blocks are expressions
    let y = { 10 + 20 };

    // if/else is an expression
    let z = if true { 1 } else { 2 };

    // Even loops can be expressions
    let result = loop {
        break 42;
    };
}

fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## Slide 10: The Semicolon

**Semicolon turns expression into statement:**

```rust
fn returns_value() -> i32 {
    5      // Expression - returns 5
}

fn returns_nothing() {
    5;     // Statement - returns ()
}

fn main() {
    let a = returns_value();    // a = 5
    let b = returns_nothing();  // b = ()
}
```

**Common mistake:**
```rust
fn oops() -> i32 {
    5;  // ERROR: expected i32, found ()
}
```

---

## Slide 11: if Expressions

**Basic if/else:**

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number < 10 {
        println!("between 5 and 10");
    } else {
        println!("10 or greater");
    }
}
```

**Note:** Condition must be `bool` (no truthy/falsy)

```rust
// if number { }      // ERROR: expected bool
if number != 0 { }    // OK
```

---

## Slide 12: if as an Expression

**Assign result of if to a variable:**

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("number = {}", number);  // Prints: 5
}
```

**Both branches must return same type:**
```rust
// ERROR: `if` and `else` have incompatible types
let value = if true { 5 } else { "six" };
```

---

## Slide 13: Loop Types

**Rust has three loop types:**

```
┌─────────────────────────────────────────────┐
│               Loop Types                     │
├───────────┬───────────────┬─────────────────┤
│   loop    │    while      │      for        │
├───────────┼───────────────┼─────────────────┤
│ Infinite  │ Conditional   │ Iterator-based  │
│ break to  │ Checks before │ Most common     │
│ exit      │ each iteration│ Safest          │
└───────────┴───────────────┴─────────────────┘
```

---

## Slide 14: The loop Keyword

**Infinite loop until `break`:**

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 5 {
            break;
        }
    }
}
```

---

## Slide 15: Returning Values from loop

**`break` can return a value:**

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Returns 20
        }
    };

    println!("Result: {}", result);
}
```

---

## Slide 16: Loop Labels

**Break out of nested loops:**

```rust
fn main() {
    let mut count = 0;

    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;           // Breaks inner loop
            }
            if count == 2 {
                break 'outer;    // Breaks outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
}
```

---

## Slide 17: while Loops

**Loop while condition is true:**

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```

**Output:**
```
3!
2!
1!
LIFTOFF!
```

---

## Slide 18: for Loops

**Iterate over a collection:**

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value: {}", element);
    }
}
```

**Advantages over while:**
- No index management
- No bounds checking needed
- No off-by-one errors
- More idiomatic Rust

---

## Slide 19: Ranges in for Loops

**Use ranges to iterate over numbers:**

```rust
fn main() {
    // Exclusive range (1, 2, 3)
    for i in 1..4 {
        println!("{}", i);
    }

    // Inclusive range (1, 2, 3, 4)
    for i in 1..=4 {
        println!("{}", i);
    }

    // Reverse
    for i in (1..4).rev() {
        println!("{}", i);  // 3, 2, 1
    }
}
```

---

## Slide 20: break and continue

**Control loop execution:**

```rust
fn main() {
    for i in 1..10 {
        if i == 3 {
            continue;  // Skip rest of this iteration
        }
        if i == 7 {
            break;     // Exit the loop entirely
        }
        println!("{}", i);
    }
}
```

**Output:** 1, 2, 4, 5, 6

---

## Slide 21: Choosing the Right Loop

| Use Case | Loop Type |
|----------|-----------|
| Retry until success | `loop` |
| Unknown iterations with condition | `while` |
| Fixed iterations | `for` with range |
| Iterate over collection | `for` |
| Need loop index | `for` with `.enumerate()` |
| Infinite with periodic check | `loop` with `break` |

---

## Slide 22: Functions Without Return Value

**Functions return `()` (unit) by default:**

```rust
fn say_hello() {
    println!("Hello!");
}

// Equivalent to:
fn say_hello_explicit() -> () {
    println!("Hello!");
}

fn main() {
    let result = say_hello();
    // result is ()
}
```

---

## Slide 23: Key Takeaways

1. **Functions use `fn`** with snake_case names
2. **Parameters require type annotations**
3. **Return type specified with `->`**
4. **Statements** end with `;`, **expressions** don't
5. **`if`** is an expression - can assign its result
6. **Three loop types:** `loop`, `while`, `for`
7. **`for` loops** are preferred for iteration
8. **`break`** can return values from `loop`

---

## Slide 24: Lab Preview

**Lab 3: Functions and Control Flow** (25 minutes)

You will:
- Write functions with parameters
- Create functions with return values
- Use if/else expressions
- Implement loop, while, and for loops
- Use break and continue
- Iterate over arrays and ranges

---

## Questions?

**Next Module:** Ownership Basics
