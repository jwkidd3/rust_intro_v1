# Lab 2b: Functions and Control Flow

**Duration:** 35 minutes

## Objectives

- Write functions with parameters and return values
- Use if/else as expressions
- Implement loop, while, and for loops
- Iterate over ranges and collections
- Use break and continue

## Prerequisites

- Completed Lab 2a

---

## Setup

Create a new project or continue in the same project:
```bash
cargo new lab02b_functions
cd lab02b_functions
code .
```

---

## Exercise 1: Functions (10 min)

### Step 1: Basic functions

```rust
fn main() {
    greet();
    greet_person("Alice");
    greet_person("Bob");
}

fn greet() {
    println!("Hello!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}
```

### Step 2: Functions with return values

```rust
fn main() {
    let x = five();
    println!("five() returned: {}", x);

    let sum = add(3, 4);
    println!("3 + 4 = {}", sum);

    let product = multiply(5, 6);
    println!("5 * 6 = {}", product);
}

fn five() -> i32 {
    5  // No semicolon = return value
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return also works
}
```

### Step 3: Statements vs expressions

```rust
fn main() {
    // This is a statement (no value)
    let x = 5;

    // This block is an expression (has a value)
    let y = {
        let a = 3;
        a + 1  // No semicolon - this is the value
    };

    println!("x = {}", x);
    println!("y = {}", y);

    // Common mistake - adding semicolon
    let z = {
        let a = 3;
        a + 1;  // With semicolon, returns ()
    };
    println!("z = {:?}", z);  // Prints: z = ()
}
```

---

## Exercise 2: Conditional Expressions (10 min)

### Step 1: Basic if/else

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("Number is less than 5");
    } else if number < 10 {
        println!("Number is between 5 and 9");
    } else {
        println!("Number is 10 or greater");
    }
}
```

### Step 2: if as an expression

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {}", number);

    // Using with functions
    let x = 10;
    let description = if x > 0 { "positive" } else { "non-positive" };
    println!("{} is {}", x, description);
}
```

### Step 3: Create a function using if expression

```rust
fn main() {
    println!("abs(-5) = {}", absolute(-5));
    println!("abs(10) = {}", absolute(10));
    println!("abs(0) = {}", absolute(0));

    println!("max(3, 7) = {}", max(3, 7));
    println!("max(10, 2) = {}", max(10, 2));
}

fn absolute(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

---

## Exercise 3: Loops (15 min)

### Step 1: loop with break

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 5 {
            break;
        }
    }

    println!("Loop ended with counter = {}", counter);
}
```

### Step 2: Returning values from loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);  // 20
}
```

### Step 3: while loop

```rust
fn main() {
    // Countdown
    let mut number = 5;

    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```

### Step 4: for loop with ranges

```rust
fn main() {
    // Exclusive range (1, 2, 3)
    println!("Exclusive range 1..4:");
    for i in 1..4 {
        println!("  {}", i);
    }

    // Inclusive range (1, 2, 3, 4)
    println!("Inclusive range 1..=4:");
    for i in 1..=4 {
        println!("  {}", i);
    }

    // Reverse
    println!("Reverse (4..1).rev():");
    for i in (1..4).rev() {
        println!("  {}", i);
    }
}
```

### Step 5: for loop with arrays

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    println!("Array elements:");
    for element in a {
        println!("  {}", element);
    }

    // With index using enumerate
    println!("With indices:");
    for (index, element) in a.iter().enumerate() {
        println!("  a[{}] = {}", index, element);
    }
}
```

### Step 6: break and continue

```rust
fn main() {
    println!("Using continue to skip 3:");
    for i in 1..=5 {
        if i == 3 {
            continue;
        }
        println!("  {}", i);
    }

    println!("Using break to stop at 3:");
    for i in 1..=5 {
        if i == 3 {
            break;
        }
        println!("  {}", i);
    }
}
```

---

## Challenge Exercise (Bonus)

### Fibonacci Function

Write a function that calculates the nth Fibonacci number:

```rust
fn main() {
    for i in 0..=10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    // Your implementation here
    // fib(0) = 0, fib(1) = 1
    // fib(n) = fib(n-1) + fib(n-2)
    0  // Replace with your code
}
```

Expected output:
```
fib(0) = 0
fib(1) = 1
fib(2) = 1
fib(3) = 2
fib(4) = 3
fib(5) = 5
...
```

---

## Verification Checklist

- [ ] Created functions with parameters
- [ ] Created functions with return values
- [ ] Used if/else for conditional logic
- [ ] Used if/else as an expression to assign values
- [ ] Implemented a loop with break
- [ ] Returned a value from a loop
- [ ] Used while for conditional looping
- [ ] Used for with ranges (exclusive and inclusive)
- [ ] Iterated over an array with for
- [ ] Used break and continue

---

## Summary

You have learned:
- Functions use `fn`, parameters require types, return type after `->`
- Expressions return values (no semicolon), statements don't
- if/else can be used as expressions
- `loop` runs forever until `break`, can return values
- `while` loops while condition is true
- `for` is preferred for iteration (ranges, collections)

**Next:** Lab 2c - Collections
