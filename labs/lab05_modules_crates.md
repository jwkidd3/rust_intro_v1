# Lab 5: Modules and Crates

**Duration:** 35 minutes

## Objectives

- Create modules in a single file
- Split modules into multiple files
- Control visibility with pub
- Add external crate dependencies
- Build a simple library crate

## Prerequisites

- Completed Labs 1-4

---

## Exercise 1: Modules in a Single File (10 min)

### Step 1: Create a new project

```bash
cargo new lab05_modules
cd lab05_modules
code .
```

### Step 2: Define modules

Replace `src/main.rs`:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // Private function
    fn helper() {
        println!("I'm a helper");
    }

    pub mod advanced {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn divide(a: i32, b: i32) -> i32 {
            a / b
        }
    }
}

fn main() {
    // Using absolute paths
    let sum = crate::math::add(5, 3);
    println!("5 + 3 = {}", sum);

    // Using relative paths
    let diff = math::subtract(10, 4);
    println!("10 - 4 = {}", diff);

    // Nested module
    let product = math::advanced::multiply(6, 7);
    println!("6 * 7 = {}", product);
}
```

### Step 3: Use the `use` keyword

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub mod advanced {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

// Bring into scope
use math::add;
use math::advanced::multiply;

fn main() {
    // Now can use directly
    let sum = add(5, 3);
    let product = multiply(6, 7);

    println!("5 + 3 = {}", sum);
    println!("6 * 7 = {}", product);
}
```

---

## Exercise 2: Modules in Separate Files (10 min)

### Step 1: Create module file structure

Create the file `src/math.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub mod advanced {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> i32 {
        a / b
    }
}
```

### Step 2: Update main.rs

```rust
mod math;  // Loads from src/math.rs

use math::{add, subtract};
use math::advanced::{multiply, divide};

fn main() {
    println!("5 + 3 = {}", add(5, 3));
    println!("10 - 4 = {}", subtract(10, 4));
    println!("6 * 7 = {}", multiply(6, 7));
    println!("20 / 4 = {}", divide(20, 4));
}
```

### Step 3: Move nested module to its own file

Create directory `src/math/` and file `src/math/advanced.rs`:

```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}
```

Update `src/math.rs`:

```rust
pub mod advanced;  // Loads from src/math/advanced.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

Your file structure should now be:
```
src/
├── main.rs
├── math.rs
└── math/
    └── advanced.rs
```

### Step 4: Test it

Update `src/main.rs` to use the new function:

```rust
mod math;

use math::{add, subtract};
use math::advanced::{multiply, divide, power};

fn main() {
    println!("5 + 3 = {}", add(5, 3));
    println!("10 - 4 = {}", subtract(10, 4));
    println!("6 * 7 = {}", multiply(6, 7));
    println!("20 / 4 = {}", divide(20, 4));
    println!("2^10 = {}", power(2, 10));
}
```

Run with `cargo run`.

---

## Exercise 3: Using External Crates (10 min)

### Step 1: Add a dependency

Edit `Cargo.toml`:

```toml
[package]
name = "lab05_modules"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

### Step 2: Use the crate

Update `src/main.rs`:

```rust
mod math;

use rand::Rng;
use math::add;

fn main() {
    // Using our module
    println!("5 + 3 = {}", add(5, 3));

    // Using external crate
    let mut rng = rand::thread_rng();

    let random_number: i32 = rng.gen_range(1..=100);
    println!("Random number (1-100): {}", random_number);

    // Generate several random numbers
    println!("Five random numbers:");
    for _ in 0..5 {
        let n: i32 = rng.gen_range(1..=10);
        print!("{} ", n);
    }
    println!();
}
```

Run `cargo build` first (downloads the crate), then `cargo run`.

---

## Exercise 4: Create a Library Crate (5 min)

### Step 1: Create a new library

```bash
cd ..
cargo new mylib --lib
cd mylib
code .
```

### Step 2: Implement the library

Edit `src/lib.rs`:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = mylib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub mod utils {
    /// Returns the larger of two numbers.
    pub fn max(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }

    /// Returns the smaller of two numbers.
    pub fn min(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }
}
```

### Step 3: Test the library

The default library template includes a test. Run it:

```bash
cargo test
```

### Step 4: Generate documentation

```bash
cargo doc --open
```

This opens the generated documentation in your browser.

---

## Challenge Exercise (Bonus)

Create a module structure for a simple geometry library:

```
src/
├── lib.rs (or main.rs)
└── geometry/
    ├── mod.rs
    ├── shapes.rs
    └── calculations.rs
```

- `shapes.rs` should contain structs for `Rectangle` and `Circle`
- `calculations.rs` should contain functions `area_rectangle` and `area_circle`
- Everything should be properly exported with `pub`

---

## Verification Checklist

- [ ] Created modules in a single file
- [ ] Used `pub` to control visibility
- [ ] Used `use` to bring items into scope
- [ ] Split modules into separate files
- [ ] Created nested module directories
- [ ] Added external crate dependency (rand)
- [ ] Used functions from external crate
- [ ] Created a library crate
- [ ] Generated documentation with `cargo doc`

---

## Summary

You have learned:
- `mod` declares a module, `pub` makes items public
- `use` brings items into scope for easier access
- Modules can be in the same file or separate files
- File structure: `mod foo;` loads `foo.rs` or `foo/mod.rs`
- External crates are added in `Cargo.toml` under `[dependencies]`
- `cargo new --lib` creates a library crate
- `cargo doc` generates documentation

**Next:** Lab 6 - Error Handling
