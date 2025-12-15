# Lab 10: Modules and Crates

## Duration: 25 minutes

## Objectives
- Organize code with modules
- Control visibility with pub
- Use external crates from crates.io
- Create a library crate

## Prerequisites
- Completed Lab 9
- Understanding of structs and functions

## Part 1: Modules (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab10_modules
cd lab10_modules
```

### Exercise 2: Inline Modules

Edit `src/main.rs`:

```rust
// Define a module inline
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }

        pub fn seat_at_table() {
            println!("Seating at table");
        }
    }

    mod serving {
        fn take_order() {
            println!("Taking order");
        }

        fn serve_order() {
            println!("Serving order");
        }

        fn take_payment() {
            println!("Taking payment");
        }
    }
}

fn main() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();

    // Cannot access private module:
    // front_of_house::serving::take_order();  // ERROR
}
```

### Exercise 3: The use Keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
    }
}

// Bring module into scope
use front_of_house::hosting;

// Or bring function directly (less idiomatic for functions)
use front_of_house::hosting::add_to_waitlist;

fn main() {
    // Using the shorter path
    hosting::add_to_waitlist();

    // Or directly
    add_to_waitlist();
}
```

### Exercise 4: pub use for Re-exporting

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
    }
}

// Re-export for external code
pub use crate::front_of_house::hosting;

fn main() {
    // Now external code can use: your_crate::hosting::add_to_waitlist()
    hosting::add_to_waitlist();
}
```

## Part 2: Separating Modules into Files (10 minutes)

### Exercise 5: Module in Separate File

Create the following file structure:

**src/main.rs:**
```rust
mod garden;  // Declares module, looks for garden.rs or garden/mod.rs

fn main() {
    garden::plant_vegetables();
    garden::vegetables::Asparagus::describe();
}
```

**src/garden.rs:**
```rust
pub mod vegetables;  // Declares submodule

pub fn plant_vegetables() {
    println!("Planting vegetables in the garden");
}
```

**src/garden/vegetables.rs:**
```rust
pub struct Asparagus {
    pub height: u32,
}

impl Asparagus {
    pub fn describe() {
        println!("Asparagus is a perennial vegetable");
    }
}
```

Create the files:
```bash
mkdir -p src/garden
```

Then create and run:
```bash
cargo run
```

### Exercise 6: Complete Module Structure

Create a more complete example:

**src/main.rs:**
```rust
mod restaurant;

use restaurant::front_of_house::hosting;
use restaurant::back_of_house;

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    back_of_house::cook::prepare_meal();
}
```

**src/restaurant.rs:**
```rust
pub mod front_of_house;
pub mod back_of_house;
```

**src/restaurant/front_of_house.rs:**
```rust
pub mod hosting;
```

**src/restaurant/front_of_house/hosting.rs:**
```rust
pub fn add_to_waitlist() {
    println!("Added to waitlist");
}

pub fn seat_at_table() {
    println!("Seated at table");
}
```

**src/restaurant/back_of_house.rs:**
```rust
pub mod cook;

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,  // Private field
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
```

**src/restaurant/back_of_house/cook.rs:**
```rust
pub fn prepare_meal() {
    println!("Preparing meal in kitchen");
}
```

Create directories and run:
```bash
mkdir -p src/restaurant/front_of_house src/restaurant/back_of_house
cargo run
```

## Part 3: External Crates (5 minutes)

### Exercise 7: Using External Crates

Update `Cargo.toml`:
```toml
[package]
name = "lab10_modules"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

Update `src/main.rs`:
```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate random numbers
    let n1: u32 = rng.gen();
    let n2: i32 = rng.gen_range(-100..100);
    let n3: f64 = rng.gen_range(0.0..1.0);

    println!("Random u32: {}", n1);
    println!("Random i32 (-100 to 100): {}", n2);
    println!("Random f64 (0.0 to 1.0): {:.4}", n3);

    // Random boolean
    let coin: bool = rng.gen();
    println!("Coin flip: {}", if coin { "Heads" } else { "Tails" });
}
```

Run:
```bash
cargo run
```

### Exercise 8: Creating a Library Crate

Create a new library:
```bash
cargo new --lib mymath
cd mymath
```

Edit `src/lib.rs`:
```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = mymath::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub mod advanced {
    /// Calculates factorial of n.
    pub fn factorial(n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n * factorial(n - 1),
        }
    }

    /// Checks if a number is prime.
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

// Tests will be covered in Lab 12
```

To verify your library works, create a `src/main.rs` in the same project:

```rust
use mymath::{add, multiply, advanced};

fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!("3 * 4 = {}", multiply(3, 4));
    println!("5! = {}", advanced::factorial(5));
    println!("Is 7 prime? {}", advanced::is_prime(7));
}
```

Run with:
```bash
cargo run
```

To use this library in another project, add to that project's `Cargo.toml`:
```toml
[dependencies]
mymath = { path = "../mymath" }
```

## Verification Steps

### Checklist
- [ ] Created inline modules with mod
- [ ] Made items public with pub
- [ ] Used the use keyword to bring items into scope
- [ ] Created modules in separate files
- [ ] Created nested module structure
- [ ] Added external crate dependency
- [ ] Created a library crate

## Lab Questions

1. What is the difference between `mod` and `use`?
2. When should you use `pub use`?
3. Where does Rust look for module files?
4. What is the difference between a binary and library crate?

## Answers

1. `mod` **declares** a module (creates or references it). `use` brings paths into scope for shorter access.

2. Use `pub use` to re-export items, making them accessible to external code without requiring them to know the internal module structure.

3. For `mod foo;`, Rust looks for either `foo.rs` or `foo/mod.rs` (older style). Submodules go in a directory matching the parent module name.

4. A **binary crate** has a `main.rs` and produces an executable. A **library crate** has a `lib.rs` and is used by other crates.

## Common Issues

**Issue: "file not found for module"**
```
Solution: Ensure the file name matches the module name, and it's in the correct directory.
```

**Issue: "unresolved import"**
```
Solution: Check that the item is marked pub and the path is correct.
```

## Next Steps

In Lab 11, you will:
- Write generic functions and structs
- Define and implement traits
- Use trait bounds

## Completion

You have completed Lab 10 when you can:
- Organize code into modules
- Control visibility with pub
- Use external crates from crates.io
- Create a library crate
