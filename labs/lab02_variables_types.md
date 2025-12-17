# Lab 2: Variables and Types

**Duration:** 35 minutes

## Objectives

- Declare mutable and immutable variables
- Use constants and shadowing
- Work with scalar types (integers, floats, booleans, characters)
- Create and access tuples and arrays
- Apply type annotations

## Prerequisites

- Completed Lab 1 (Rust installed, VS Code configured)

---

## Setup

Create a new project for this lab:
```bash
cargo new lab02_variables
cd lab02_variables
code .
```

---

## Exercise 1: Variables and Mutability (10 min)

### Step 1: Immutable variables

Replace the contents of `src/main.rs`:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // Uncomment the next line to see the error
    // x = 6;
}
```

Run with `cargo run`. Then uncomment the line `x = 6;` and run again to see the compiler error.

### Step 2: Mutable variables

Fix the code using `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is now: {}", x);
}
```

### Step 3: Constants

Add constants to your program:

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Maximum points: {}", MAX_POINTS);
    println!("Pi: {}", PI);

    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
}
```

---

## Exercise 2: Shadowing (5 min)

### Step 1: Basic shadowing

```rust
fn main() {
    let x = 5;
    println!("Original x: {}", x);

    let x = x + 1;
    println!("After first shadow: {}", x);

    let x = x * 2;
    println!("After second shadow: {}", x);
}
```

### Step 2: Shadowing with type change

```rust
fn main() {
    let spaces = "   ";
    println!("spaces (string): '{}'", spaces);

    let spaces = spaces.len();
    println!("spaces (length): {}", spaces);
}
```

---

## Exercise 3: Scalar Types (10 min)

### Step 1: Integer types

```rust
fn main() {
    // Default is i32
    let a = 42;

    // Explicit types
    let b: i8 = -128;
    let c: u8 = 255;
    let d: i64 = 1_000_000_000;

    // Different literals
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("a = {}", a);
    println!("b (i8) = {}", b);
    println!("c (u8) = {}", c);
    println!("d (i64) = {}", d);
    println!("hex = {}", hex);
    println!("octal = {}", octal);
    println!("binary = {}", binary);
    println!("byte = {}", byte);
}
```

### Step 2: Floating-point types

```rust
fn main() {
    let x = 2.0;      // f64 by default
    let y: f32 = 3.0; // f32

    println!("x (f64) = {}", x);
    println!("y (f32) = {}", y);

    // Arithmetic
    let sum = 5.0 + 10.0;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43.0 % 5.0;

    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("remainder = {}", remainder);
}
```

### Step 3: Booleans and characters

```rust
fn main() {
    let t = true;
    let f: bool = false;

    println!("t = {}, f = {}", t, f);

    let c = 'z';
    let z: char = 'Z';
    let heart = '❤';
    let crab = '🦀';

    println!("Characters: {} {} {} {}", c, z, heart, crab);
}
```

---

## Exercise 4: Compound Types (10 min)

### Step 1: Tuples

```rust
fn main() {
    // Create a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("By index: {}, {}, {}", five_hundred, six_point_four, one);

    // Unit tuple
    let unit: () = ();
    println!("Unit tuple: {:?}", unit);
}
```

### Step 2: Arrays

```rust
fn main() {
    // Create arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);

    // Access elements
    let first = a[0];
    let second = a[1];

    println!("First: {}, Second: {}", first, second);
    println!("Length of a: {}", a.len());
}
```

### Step 3: Array bounds checking

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // This will compile but panic at runtime
    // Uncomment to test:
    // let index = 10;
    // let element = a[index];

    // Safe access with get()
    let element = a.get(10);
    match element {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds!"),
    }
}
```

---

## Challenge Exercise (Bonus)

Create a program that:
1. Declares a constant for the number of days in a week
2. Creates a tuple containing your name (as &str), age (u32), and height in meters (f64)
3. Creates an array of your favorite 5 numbers
4. Prints all values with descriptive messages

```rust
const DAYS_IN_WEEK: u32 = 7;

fn main() {
    // Your code here
}
```

---

## Verification Checklist

- [ ] Demonstrated immutable vs mutable variables
- [ ] Used constants with SCREAMING_SNAKE_CASE
- [ ] Used shadowing to reuse variable names
- [ ] Worked with integers, floats, booleans, and characters
- [ ] Created and accessed tuples
- [ ] Created and accessed arrays
- [ ] Applied explicit type annotations

---

## Summary

You have learned:
- Variables are immutable by default; use `mut` for mutability
- Constants use `const` and require type annotations
- Shadowing allows reusing names and changing types
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples (mixed types), arrays (same type, fixed size)

**Next:** Lab 3 - Functions and Control Flow
