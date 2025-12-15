# Module 2: Variables and Data Types

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**Variables and Data Types**

- Understanding Rust's Type System
- Module 2 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Declare variables and understand mutability
- Use constants and shadowing
- Work with scalar types (integers, floats, booleans, characters)
- Use compound types (tuples and arrays)
- Understand type inference and annotations

---

## Slide 3: Variables in Rust

**Variables are immutable by default!**

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);

    // x = 6;  // ERROR: cannot assign twice to immutable variable
}
```

**Why?**
- Prevents accidental mutations
- Enables compiler optimizations
- Makes code easier to reason about

---

## Slide 4: Mutable Variables

**Use `mut` to make a variable mutable:**

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);

    x = 6;  // OK: x is mutable
    println!("x = {}", x);
}
```

**Best practice:** Only use `mut` when necessary

---

## Slide 5: Constants

**Constants are always immutable:**

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Max points: {}", MAX_POINTS);
    println!("Pi: {}", PI);
}
```

**Rules for constants:**
- Must have type annotation
- Can only be set to constant expressions
- Can be declared in any scope (including global)
- Use SCREAMING_SNAKE_CASE naming

---

## Slide 6: Constants vs Variables

| Feature | `let` | `const` |
|---------|-------|---------|
| Mutable | With `mut` | Never |
| Type annotation | Optional | Required |
| Computed at | Runtime | Compile time |
| Scope | Block | Any (including global) |
| Shadowing | Allowed | Not allowed |

---

## Slide 7: Shadowing

**Create a new variable with the same name:**

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // Shadows previous x
    let x = x * 2;      // Shadows again

    println!("x = {}", x);  // Prints: x = 12
}
```

**Shadowing allows type changes:**
```rust
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize
```

---

## Slide 8: Shadowing vs Mutability

```rust
// Shadowing - creates new variable
let x = 5;
let x = "hello";  // OK: different type allowed

// Mutability - same variable, same type
let mut y = 5;
y = 6;            // OK: same type
// y = "hello";   // ERROR: type mismatch
```

**Use shadowing when:**
- You want to transform a value
- You want to change types
- You want to reuse a name in a new scope

---

## Slide 9: Scalar Types Overview

**Four primary scalar types:**

```
┌─────────────────────────────────────────────┐
│              Scalar Types                    │
├───────────┬───────────┬─────────┬───────────┤
│  Integer  │   Float   │ Boolean │ Character │
├───────────┼───────────┼─────────┼───────────┤
│ i8, i16   │ f32       │ bool    │ char      │
│ i32, i64  │ f64       │         │           │
│ i128      │           │         │           │
│ u8, u16   │           │         │           │
│ u32, u64  │           │         │           │
│ u128      │           │         │           │
│ isize     │           │         │           │
│ usize     │           │         │           │
└───────────┴───────────┴─────────┴───────────┘
```

---

## Slide 10: Integer Types

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

**Default:** `i32` (best general performance)

**Range:**
- Signed: -(2^(n-1)) to 2^(n-1) - 1
- Unsigned: 0 to 2^n - 1

---

## Slide 11: Integer Literals

```rust
fn main() {
    let decimal = 98_222;      // Underscores for readability
    let hex = 0xff;            // Hexadecimal
    let octal = 0o77;          // Octal
    let binary = 0b1111_0000;  // Binary
    let byte = b'A';           // Byte (u8 only)

    // Type suffix
    let x = 42u32;             // u32
    let y = 100_i64;           // i64
}
```

---

## Slide 12: Floating-Point Types

**Two floating-point types:**

```rust
fn main() {
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32
}
```

| Type | Precision | Size |
|------|-----------|------|
| f32 | Single | 32 bits |
| f64 | Double | 64 bits |

**Default:** `f64` (modern CPUs, same speed as f32)

---

## Slide 13: Numeric Operations

```rust
fn main() {
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let integer_div = 5 / 3;  // Results in 1

    // Remainder
    let remainder = 43 % 5;
}
```

---

## Slide 14: Boolean Type

```rust
fn main() {
    let t = true;
    let f: bool = false;

    // Booleans are one byte
    println!("Size: {} byte", std::mem::size_of::<bool>());

    // Used in conditions
    if t {
        println!("It's true!");
    }
}
```

**Values:** `true` or `false`
**Size:** 1 byte

---

## Slide 15: Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart = '❤';
    let emoji = '🦀';
    let kanji = '日';

    println!("Size: {} bytes", std::mem::size_of::<char>());
}
```

**Key points:**
- Use single quotes (not double)
- 4 bytes (Unicode scalar value)
- Represents any Unicode character
- Range: U+0000 to U+D7FF and U+E000 to U+10FFFF

---

## Slide 16: Compound Types

**Types that group multiple values:**

```
┌─────────────────────────────────────────────┐
│            Compound Types                    │
├─────────────────────┬───────────────────────┤
│       Tuples        │        Arrays         │
├─────────────────────┼───────────────────────┤
│ • Fixed length      │ • Fixed length        │
│ • Different types   │ • Same type           │
│ • Access by .0, .1  │ • Access by [0], [1]  │
│ • Destructuring     │ • Stack allocated     │
└─────────────────────┴───────────────────────┘
```

---

## Slide 17: Tuples

```rust
fn main() {
    // Create a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("y = {}", y);

    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Unit tuple (empty)
    let unit: () = ();
}
```

---

## Slide 18: Arrays

```rust
fn main() {
    // Create an array
    let a = [1, 2, 3, 4, 5];

    // With type annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize with same value
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    // Access elements
    let first = a[0];
    let second = a[1];

    // Get length
    println!("Length: {}", a.len());
}
```

---

## Slide 19: Arrays vs Tuples

| Feature | Array | Tuple |
|---------|-------|-------|
| Element types | Same | Different |
| Length | Fixed | Fixed |
| Access syntax | `arr[0]` | `tup.0` |
| Type syntax | `[T; N]` | `(T1, T2, ...)` |
| Use case | Lists of same items | Group related values |

```rust
let array: [i32; 3] = [1, 2, 3];
let tuple: (i32, &str, f64) = (1, "hello", 3.14);
```

---

## Slide 20: Type Inference

**Rust can often infer types:**

```rust
fn main() {
    let x = 5;           // Inferred as i32
    let y = 2.0;         // Inferred as f64
    let z = true;        // Inferred as bool
    let s = "hello";     // Inferred as &str

    // Sometimes you need to annotate
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

**When to annotate:**
- Compiler can't infer
- You want a specific type
- Improved readability

---

## Slide 21: Type Annotations

```rust
fn main() {
    // After variable name
    let x: i32 = 5;
    let y: f64 = 3.14;

    // For arrays
    let arr: [u8; 4] = [1, 2, 3, 4];

    // For tuples
    let tup: (i32, f64, bool) = (42, 3.14, true);

    // Using turbofish syntax
    let parsed = "42".parse::<i32>().unwrap();
}
```

---

## Slide 22: Overflow Behavior

**In debug mode:** Rust checks for overflow and panics

**In release mode:** Wraps around (two's complement)

```rust
fn main() {
    let x: u8 = 255;
    // let y = x + 1;  // Panics in debug, wraps in release

    // Explicit handling:
    let wrapped = x.wrapping_add(1);     // 0
    let checked = x.checked_add(1);       // None
    let saturated = x.saturating_add(1);  // 255
    let (val, overflow) = x.overflowing_add(1); // (0, true)
}
```

---

## Slide 23: Key Takeaways

1. **Variables are immutable by default** - use `mut` for mutability
2. **Constants** are compile-time values with required type annotations
3. **Shadowing** creates new variables, allows type changes
4. **Scalar types:** integers, floats, booleans, characters
5. **Compound types:** tuples (mixed types), arrays (same type)
6. **Type inference** works most of the time
7. **Annotate types** when needed for clarity or compiler help

---

## Slide 24: Lab Preview

**Lab 2: Variables and Data Types** (25 minutes)

You will:
- Declare mutable and immutable variables
- Use constants and shadowing
- Work with integer, float, boolean, and char types
- Create and access tuples and arrays
- Apply type annotations

---

## Questions?

**Next Module:** Functions and Control Flow
