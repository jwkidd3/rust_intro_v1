# Module 2: Rust Language Essentials

**Duration:** 85 minutes (split into 3 parts)

---

# Part 1: Types and Variables (30 min)

---

## Slide 1: Title

**Rust Language Essentials - Part 1**

- Types and Variables
- Module 2a of 6

---

## Slide 2: Variables in Rust

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

---

## Slide 3: Mutable Variables

**Use `mut` to make a variable mutable:**

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);

    x = 6;  // OK: x is mutable
    println!("x = {}", x);
}
```

---

## Slide 4: Constants

**Constants are always immutable:**

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Max points: {}", MAX_POINTS);
}
```

**Rules:**
- Must have type annotation
- Use SCREAMING_SNAKE_CASE
- Can be declared in any scope

---

## Slide 5: Shadowing

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

## Slide 6: Integer Types

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

**Default:** `i32`

---

## Slide 7: Integer Literals

```rust
fn main() {
    let decimal = 98_222;      // Underscores for readability
    let hex = 0xff;            // Hexadecimal
    let octal = 0o77;          // Octal
    let binary = 0b1111_0000;  // Binary
    let byte = b'A';           // Byte (u8 only)

    let x = 42u32;             // Type suffix
}
```

---

## Slide 8: Floating-Point Types

```rust
fn main() {
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32
}
```

| Type | Precision |
|------|-----------|
| f32 | Single (32 bits) |
| f64 | Double (64 bits) |

---

## Slide 9: Numeric Operations

```rust
fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let integer_div = 5 / 3;  // Results in 1
    let remainder = 43 % 5;
}
```

---

## Slide 10: Boolean and Character

```rust
fn main() {
    // Boolean - 1 byte
    let t = true;
    let f: bool = false;

    // Character - 4 bytes (Unicode)
    let c = 'z';
    let heart = '❤';
    let emoji = '🦀';
}
```

---

## Slide 11: Tuples

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;

    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
}
```

---

## Slide 12: Arrays

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    println!("Length: {}", a.len());
}
```

---

## Slide 13: Type Inference

```rust
fn main() {
    let x = 5;           // Inferred as i32
    let y = 2.0;         // Inferred as f64
    let z = true;        // Inferred as bool

    // Sometimes you need to annotate
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

---

# Part 2: Control Flow and Functions (30 min)

---

## Slide 14: Title

**Rust Language Essentials - Part 2**

- Conditional Logic, Iteration, Functions
- Module 2b of 6

---

## Slide 15: if Expressions

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

**Note:** Condition must be `bool`

---

## Slide 16: if as an Expression

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number = {}", number);
}
```

**Both branches must return same type**

---

## Slide 17: loop

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

## Slide 18: while Loop

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

---

## Slide 19: for Loop

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value: {}", element);
    }

    // With range
    for i in 1..4 {
        println!("{}", i);  // 1, 2, 3
    }

    // Inclusive range
    for i in 1..=4 {
        println!("{}", i);  // 1, 2, 3, 4
    }
}
```

---

## Slide 20: break and continue

```rust
fn main() {
    for i in 1..10 {
        if i == 3 {
            continue;  // Skip this iteration
        }
        if i == 7 {
            break;     // Exit the loop
        }
        println!("{}", i);
    }
}
```

**Output:** 1, 2, 4, 5, 6

---

## Slide 21: Functions

```rust
fn main() {
    greet();
    print_value(42);
    let sum = add(3, 4);
    println!("Sum: {}", sum);
}

fn greet() {
    println!("Hello!");
}

fn print_value(x: i32) {
    println!("Value: {}", x);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}
```

---

## Slide 22: Statements vs Expressions

```rust
fn main() {
    // Statement - ends with semicolon, no value
    let x = 5;

    // Expression - evaluates to a value
    let y = {
        let a = 3;
        a + 1  // No semicolon - returns 4
    };

    println!("y = {}", y);
}
```

---

## Slide 23: Return Values

```rust
fn five() -> i32 {
    5  // Implicit return
}

fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x;  // Early return
    }
    x
}
```

---

# Part 3: Collections (25 min)

---

## Slide 24: Title

**Rust Language Essentials - Part 3**

- Collections
- Module 2c of 6

---

## Slide 25: Vec<T> - Dynamic Array

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Or use vec! macro
    let v2 = vec![1, 2, 3];

    println!("{:?}", v);
}
```

---

## Slide 26: Accessing Vector Elements

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];           // Panics if out of bounds
    let third = v.get(2);       // Returns Option<&T>

    match v.get(2) {
        Some(value) => println!("Third: {}", value),
        None => println!("No third element"),
    }
}
```

---

## Slide 27: Iterating Over Vectors

```rust
fn main() {
    let v = vec![10, 20, 30];

    for value in &v {
        println!("{}", value);
    }

    // Mutable iteration
    let mut v2 = vec![1, 2, 3];
    for value in &mut v2 {
        *value *= 2;
    }
}
```

---

## Slide 28: String

```rust
fn main() {
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");

    // From literal
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    println!("{}", s);
}
```

---

## Slide 29: String Operations

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved

    // Or use format!
    let s4 = String::from("Hello");
    let s5 = String::from("world");
    let s6 = format!("{}, {}!", s4, s5);
}
```

---

## Slide 30: HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team = String::from("Blue");
    let score = scores.get(&team);  // Option<&V>

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

---

## Slide 31: Key Takeaways

1. **Variables are immutable by default** - use `mut` for mutability
2. **Scalar types:** integers, floats, booleans, characters
3. **Compound types:** tuples (mixed), arrays (same type, fixed)
4. **if/else are expressions** - can return values
5. **Three loop types:** `loop`, `while`, `for`
6. **Functions** use `fn`, return with no semicolon
7. **Collections:** `Vec<T>`, `String`, `HashMap<K,V>`

---

## Slide 32: Lab Preview

**Lab 2: Variables and Types** (35 min)
- Work with variables, mutability, types

**Lab 3: Functions and Control Flow** (35 min)
- Write functions, use conditionals and loops

**Lab 4: Collections** (30 min)
- Use Vec, String, and HashMap

---

## Questions?
