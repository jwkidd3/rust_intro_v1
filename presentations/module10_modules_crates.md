# Module 10: Modules and Crates

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**Modules and Crates**

- Organizing Rust Code
- Module 10 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand Rust's module system
- Organize code with modules
- Control visibility with pub
- Use the use keyword for imports
- Work with multiple files
- Understand crates and Cargo.toml

---

## Slide 3: The Module System

```
┌─────────────────────────────────────────────┐
│           Rust's Module System              │
├─────────────┬───────────────────────────────┤
│   Crates    │ Compilation units, binary     │
│             │ or library                    │
├─────────────┼───────────────────────────────┤
│  Modules    │ Organize code within a crate  │
├─────────────┼───────────────────────────────┤
│   Paths     │ Name items in the module tree │
├─────────────┼───────────────────────────────┤
│   use       │ Bring paths into scope        │
├─────────────┼───────────────────────────────┤
│   pub       │ Make items public             │
└─────────────┴───────────────────────────────┘
```

---

## Slide 4: Crate Types

**Binary Crate:**
- Has `main.rs`
- Compiles to executable
- Has `fn main()`

**Library Crate:**
- Has `lib.rs`
- Compiles to library
- No `main()` function
- Meant to be used by other crates

```bash
cargo new my_binary          # Binary crate
cargo new my_library --lib   # Library crate
```

---

## Slide 5: Defining Modules

```rust
// In src/main.rs or src/lib.rs

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

---

## Slide 6: Module Tree

```
crate (root)
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

**Like a filesystem directory structure**

---

## Slide 7: Paths to Items

**Two types of paths:**

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    // Absolute path (from crate root)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (from current module)
    front_of_house::hosting::add_to_waitlist();
}
```

---

## Slide 8: Visibility and pub

**Items are private by default:**

```rust
mod front_of_house {
    mod hosting {  // Private module
        fn add_to_waitlist() {}  // Private function
    }
}

fn main() {
    // ERROR: module `hosting` is private
    // front_of_house::hosting::add_to_waitlist();
}
```

---

## Slide 9: Making Items Public

```rust
mod front_of_house {
    pub mod hosting {        // Public module
        pub fn add_to_waitlist() {}  // Public function
    }
}

fn main() {
    // Now works!
    front_of_house::hosting::add_to_waitlist();
}
```

**Both the module AND the function must be pub**

---

## Slide 10: Visibility Rules

```
┌─────────────────────────────────────────────┐
│           Visibility Rules                   │
├─────────────────────────────────────────────┤
│ • Child modules can see parent's items      │
│ • Parent modules can't see child's private  │
│ • Siblings can see each other if pub        │
│ • pub makes visible to parent and beyond    │
└─────────────────────────────────────────────┘
```

```rust
mod parent {
    fn private_parent() {}

    mod child {
        fn private_child() {
            super::private_parent();  // OK: child sees parent
        }
    }
}
```

---

## Slide 11: super Keyword

**Access parent module:**

```rust
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // Go up to parent
    }

    fn cook_order() {}
}

fn deliver_order() {}
```

---

## Slide 12: Struct Visibility

**Struct fields are private by default:**

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // Public field
        seasonal_fruit: String, // Private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn main() {
    let meal = back_of_house::Breakfast::summer("Rye");
    println!("Toast: {}", meal.toast);
    // println!("{}", meal.seasonal_fruit);  // ERROR: private
}
```

---

## Slide 13: Enum Visibility

**All variants are public if enum is public:**

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,    // Public
        Salad,   // Public
    }
}

fn main() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

---

## Slide 14: The use Keyword

**Bring paths into scope:**

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

---

## Slide 15: use with as

**Rename imports:**

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
```

---

## Slide 16: Re-exporting with pub use

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Re-export for external code
pub use front_of_house::hosting;

// Now external code can use:
// my_crate::hosting::add_to_waitlist()
```

---

## Slide 17: Nested use

**Combine multiple imports:**

```rust
// Instead of:
use std::cmp::Ordering;
use std::io;

// Write:
use std::{cmp::Ordering, io};

// Or with self:
use std::io::{self, Write};
// Brings in std::io and std::io::Write
```

---

## Slide 18: Glob Import

**Import everything:**

```rust
use std::collections::*;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut set: HashSet<i32> = HashSet::new();
}
```

**Use sparingly - can cause naming conflicts**

---

## Slide 19: Separating Modules into Files

**Single file module:**
```
src/
├── main.rs
└── front_of_house.rs
```

```rust
// src/main.rs
mod front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

## Slide 20: Module Directory Structure

**For nested modules:**
```
src/
├── main.rs
└── front_of_house/
    ├── mod.rs
    └── hosting.rs
```

```rust
// src/main.rs
mod front_of_house;

// src/front_of_house/mod.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

---

## Slide 21: Modern File Layout (Rust 2018+)

**Alternative without mod.rs:**
```
src/
├── main.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

```rust
// src/main.rs
mod front_of_house;

// src/front_of_house.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

---

## Slide 22: External Dependencies

**Cargo.toml:**
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

**Using in code:**
```rust
use rand::Rng;

fn main() {
    let n: i32 = rand::thread_rng().gen_range(1..=100);
}
```

---

## Slide 23: Crate Documentation

```rust
//! # My Crate
//!
//! This crate does amazing things.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Generate docs: `cargo doc --open`**

---

## Slide 24: Key Takeaways

1. **Crates** are compilation units (binary or library)
2. **Modules** organize code within crates
3. Items are **private by default** - use `pub` to expose
4. **use** brings items into scope
5. **Paths** can be absolute (`crate::`) or relative
6. **Files** can contain modules or be modules
7. **Dependencies** go in Cargo.toml

---

## Slide 25: Lab Preview

**Lab 10: Modules and Crates** (25 minutes)

You will:
- Create modules in a single file
- Split modules into multiple files
- Use visibility modifiers (pub)
- Import items with use
- Create a library crate
- Add and use external dependencies

---

## Questions?

**Next Module:** Generics and Traits
