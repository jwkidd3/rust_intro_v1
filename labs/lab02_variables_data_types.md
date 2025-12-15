# Lab 2: Variables and Data Types

## Duration: 25 minutes

## Objectives
- Understand variable declaration and mutability
- Work with scalar data types (integers, floats, booleans, characters)
- Use compound data types (tuples and arrays)
- Apply type annotations and inference

## Prerequisites
- Completed Lab 1
- Rust environment setup

## Part 1: Variables and Mutability (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab02_variables
cd lab02_variables
```

### Exercise 2: Immutable Variables

Edit `src/main.rs`:

```rust
fn main() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    // This would cause a compile error:
    // x = 6;  // Uncomment to see the error
}
```

Run and observe:
```bash
cargo run
```

### Exercise 3: Mutable Variables

Update `src/main.rs`:

```rust
fn main() {
    // Use 'mut' to make a variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is now: {}", x);
}
```

### Exercise 4: Constants

Add constants to your program:

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Maximum points: {}", MAX_POINTS);
    println!("Pi is approximately: {}", PI);

    let mut score = 0;
    score = MAX_POINTS;
    println!("Your score: {}", score);
}
```

### Exercise 5: Shadowing

```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);

    let x = x + 1;  // Shadows the previous x
    println!("x is now: {}", x);

    let x = x * 2;  // Shadows again
    println!("x is now: {}", x);

    // Shadowing allows type change
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);
}
```

## Part 2: Scalar Data Types (10 minutes)

### Exercise 6: Integer Types

Create a new file or update `main.rs`:

```rust
fn main() {
    // Signed integers (can be negative)
    let a: i8 = -128;
    let b: i16 = 32_000;
    let c: i32 = 2_000_000;  // Default integer type
    let d: i64 = 9_000_000_000;

    // Unsigned integers (positive only)
    let e: u8 = 255;
    let f: u32 = 4_000_000;

    // Different literal formats
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';  // u8 only

    println!("Signed: {}, {}, {}, {}", a, b, c, d);
    println!("Unsigned: {}, {}", e, f);
    println!("Literals: {}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);
}
```

### Exercise 7: Floating-Point and Boolean Types

```rust
fn main() {
    // Floating-point numbers
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // Boolean type
    let t = true;
    let f: bool = false;

    println!("True: {}, False: {}", t, f);
}
```

### Exercise 8: Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_emoji = '❤';
    let japanese = '日';

    println!("Characters: {}, {}, {}, {}", c, z, heart_emoji, japanese);

    // char is 4 bytes (Unicode scalar value)
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}
```

## Part 3: Compound Types (5 minutes)

### Exercise 9: Tuples

```rust
fn main() {
    // Tuple with mixed types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("By index: {}, {}, {}", five_hundred, six_point_four, one);

    // Unit tuple (empty tuple)
    let unit: () = ();
}
```

### Exercise 10: Arrays

```rust
fn main() {
    // Array: fixed length, same type
    let a = [1, 2, 3, 4, 5];

    // With type annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize with same value
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    // Access elements
    let first = a[0];
    let second = a[1];

    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);
    println!("First: {}, Second: {}", first, second);

    // Array length
    println!("Length of a: {}", a.len());

    // Access all elements (iteration covered in Lab 3)
    println!("All elements: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}
```

## Verification Steps

### Checklist
- [ ] Created mutable and immutable variables
- [ ] Defined and used constants
- [ ] Used shadowing to redefine variables
- [ ] Worked with integer types (i8, i32, u8, u32, etc.)
- [ ] Used floating-point and boolean types
- [ ] Created and accessed tuples
- [ ] Created and accessed arrays

## Lab Questions

1. What is the difference between `let` and `let mut`?
2. What is shadowing and how does it differ from mutability?
3. What is the default integer type in Rust?
4. What is the difference between a tuple and an array?

## Answers

1. `let` creates an immutable binding (value cannot change). `let mut` creates a mutable binding (value can be reassigned).

2. **Shadowing** creates a new variable with the same name, allowing type changes. **Mutability** allows changing the value of an existing variable but not its type.

3. The default integer type is **i32** (32-bit signed integer).

4. **Tuples** can contain different types and have fixed length. **Arrays** contain same-type elements with fixed length. Tuples are accessed by `.0`, `.1`, etc.; arrays by `[0]`, `[1]`, etc.

## Common Issues

**Issue: "cannot assign twice to immutable variable"**
```
Solution: Add 'mut' keyword: let mut x = 5;
```

**Issue: "index out of bounds"**
```
Solution: Rust arrays are zero-indexed. An array of length 5 has valid indices 0-4.
```

## Next Steps

In Lab 3, you will:
- Define and call functions
- Use control flow with if/else
- Work with loops (loop, while, for)

## Completion

You have completed Lab 2 when you can:
- Explain the difference between mutable and immutable variables
- Use constants and shadowing appropriately
- Work with all scalar types
- Create and access tuples and arrays
