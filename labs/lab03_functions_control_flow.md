# Lab 3: Functions and Control Flow

## Duration: 25 minutes

## Objectives
- Define and call functions with parameters
- Understand return values and expressions
- Use if/else conditional statements
- Work with loops (loop, while, for)

## Prerequisites
- Completed Lab 2
- Understanding of variables and data types

## Part 1: Functions (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab03_functions
cd lab03_functions
```

### Exercise 2: Basic Functions

Edit `src/main.rs`:

```rust
fn main() {
    println!("Hello from main!");

    another_function();
    greet();
}

fn another_function() {
    println!("Hello from another_function!");
}

fn greet() {
    println!("Welcome to Rust!");
}
```

### Exercise 3: Functions with Parameters

```rust
fn main() {
    print_value(5);
    print_labeled_value(42, "answer");
    print_point(3, 7);
}

fn print_value(x: i32) {
    println!("The value is: {}", x);
}

fn print_labeled_value(value: i32, label: &str) {
    println!("The {} is: {}", label, value);
}

fn print_point(x: i32, y: i32) {
    println!("Point: ({}, {})", x, y);
}
```

### Exercise 4: Functions with Return Values

```rust
fn main() {
    let x = five();
    println!("five() returned: {}", x);

    let y = plus_one(5);
    println!("plus_one(5) returned: {}", y);

    let sum = add(3, 4);
    println!("add(3, 4) returned: {}", sum);
}

fn five() -> i32 {
    5  // No semicolon - this is an expression that returns
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Exercise 5: Statements vs Expressions

```rust
fn main() {
    // Statement: performs action, no return value
    let x = 5;  // This is a statement

    // Expression: evaluates to a value
    let y = {
        let x = 3;
        x + 1  // No semicolon - expression returns 4
    };
    println!("y = {}", y);

    // Function calls are expressions
    let z = add(2, 3);
    println!("z = {}", z);

    // Blocks can be expressions
    let result = {
        let a = 10;
        let b = 20;
        a + b
    };
    println!("result = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Part 2: Control Flow (15 minutes)

### Exercise 6: if/else Expressions

```rust
fn main() {
    let number = 7;

    // Basic if/else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Multiple conditions
    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is 5");
    } else if number < 10 {
        println!("number is between 5 and 10");
    } else {
        println!("number is 10 or greater");
    }

    // if is an expression - can be used in let
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("x = {}", x);
}
```

### Exercise 7: The loop Keyword

```rust
fn main() {
    // Infinite loop (use Ctrl+C to stop if you run this alone)
    // loop {
    //     println!("again!");
    // }

    // Loop with break
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter = {}", counter);
        if counter == 5 {
            break;
        }
    }

    // Return value from loop
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;  // Returns 20
        }
    };
    println!("Loop result: {}", result);
}
```

### Exercise 8: Loop Labels

```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  remaining = {}", remaining);
            if remaining == 9 {
                break;  // Breaks inner loop
            }
            if count == 2 {
                break 'counting_up;  // Breaks outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

### Exercise 9: while Loops

```rust
fn main() {
    // Basic while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // while with array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("Value at index {}: {}", index, a[index]);
        index += 1;
    }
}
```

### Exercise 10: for Loops

```rust
fn main() {
    // Iterate over array (preferred method)
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Element: {}", element);
    }

    // Iterate with index
    for (index, value) in a.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }

    // Range (exclusive end)
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    // Range (inclusive end)
    for number in 1..=3 {
        println!("Inclusive: {}", number);
    }

    // Reverse range
    for number in (1..4).rev() {
        println!("Countdown: {}", number);
    }
}
```

### Exercise 11: Putting It Together

Create a simple number guessing feedback function:

```rust
fn main() {
    let secret = 42;
    let guesses = [30, 50, 42, 45];

    for guess in guesses {
        let result = check_guess(guess, secret);
        println!("Guess {}: {}", guess, result);
    }
}

fn check_guess(guess: i32, secret: i32) -> &'static str {
    if guess < secret {
        "Too low!"
    } else if guess > secret {
        "Too high!"
    } else {
        "Correct!"
    }
}
```

## Verification Steps

### Checklist
- [ ] Created functions with no parameters
- [ ] Created functions with multiple parameters
- [ ] Created functions with return values
- [ ] Understand statements vs expressions
- [ ] Used if/else conditions
- [ ] Used if as an expression in let statements
- [ ] Created loops with loop, while, and for
- [ ] Used break and continue in loops
- [ ] Iterated over arrays and ranges with for

## Lab Questions

1. What is the difference between a statement and an expression in Rust?
2. How do you return a value from a function?
3. What is the difference between `1..5` and `1..=5`?
4. How do you break out of a nested loop?

## Answers

1. **Statements** perform actions but don't return values (end with semicolons). **Expressions** evaluate to a value (no semicolon at the end when used as return value).

2. Either use the `return` keyword, or place an expression without a semicolon as the last line of the function body.

3. `1..5` is exclusive (1, 2, 3, 4). `1..=5` is inclusive (1, 2, 3, 4, 5).

4. Use **loop labels**: `'outer: loop { ... break 'outer; ... }`

## Common Issues

**Issue: "expected `()`, found `i32`"**
```
Solution: You likely added a semicolon after the return expression.
Wrong: fn five() -> i32 { 5; }
Right: fn five() -> i32 { 5 }
```

**Issue: "mismatched types in if/else"**
```
Solution: Both branches of if/else must return the same type.
```

## Next Steps

In Lab 4, you will:
- Learn Rust's ownership system
- Understand move semantics
- Work with the stack and heap

## Completion

You have completed Lab 3 when you can:
- Write functions with parameters and return values
- Use if/else as both statements and expressions
- Choose the appropriate loop type for different scenarios
- Iterate over collections using for loops
