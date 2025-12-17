# Module 3: Organizing Rust Code

**Duration:** 30 minutes

---

## Slide 1: Title

**Organizing Rust Code**

- Modules, Packages, Crates, and Cargo
- Module 3 of 6

---

## Slide 2: The Module System

```
┌─────────────────────────────────────────────┐
│           Rust's Module System              │
├─────────────┬───────────────────────────────┤
│   Packages  │ Cargo feature, contains crates│
├─────────────┼───────────────────────────────┤
│   Crates    │ Compilation unit (binary/lib) │
├─────────────┼───────────────────────────────┤
│   Modules   │ Organize code within a crate  │
├─────────────┼───────────────────────────────┤
│   Paths     │ Name items in the module tree │
└─────────────┴───────────────────────────────┘
```

---

## Slide 3: Packages and Crates

**Package:**
- Created by `cargo new`
- Contains a `Cargo.toml`
- Can have multiple crates

**Crate Types:**
- **Binary crate** - has `main.rs`, compiles to executable
- **Library crate** - has `lib.rs`, used by other code

```bash
cargo new my_app          # Binary crate
cargo new my_lib --lib    # Library crate
```

---

## Slide 4: Defining Modules

```rust
// In src/main.rs or src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }

    mod serving {
        fn take_order() {}
    }
}
```

---

## Slide 5: Module Tree

```
crate (root)
 └── front_of_house
     ├── hosting
     │   └── add_to_waitlist
     └── serving
         └── take_order
```

**Like a filesystem directory structure**

---

## Slide 6: Visibility with pub

**Items are private by default:**

```rust
mod front_of_house {
    pub mod hosting {        // Public module
        pub fn add_to_waitlist() {}  // Public function
        fn seat_at_table() {}        // Private function
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();  // OK
    // front_of_house::hosting::seat_at_table(); // ERROR: private
}
```

---

## Slide 7: Paths to Items

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    // Absolute path (from crate root)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

---

## Slide 8: The use Keyword

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

## Slide 9: use with as

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

## Slide 10: Separating Modules into Files

**Project structure:**
```
src/
├── main.rs
└── front_of_house.rs
```

**src/main.rs:**
```rust
mod front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

**src/front_of_house.rs:**
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

## Slide 11: Nested Module Files

**For deeper nesting:**
```
src/
├── main.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

**src/front_of_house.rs:**
```rust
pub mod hosting;
```

**src/front_of_house/hosting.rs:**
```rust
pub fn add_to_waitlist() {}
```

---

## Slide 12: Cargo - The Build Tool

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

---

## Slide 13: Adding Dependencies

**Find crates at https://crates.io**

```toml
[dependencies]
rand = "0.8"
```

**Using in code:**
```rust
use rand::Rng;

fn main() {
    let n: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Random: {}", n);
}
```

---

## Slide 14: Cargo Commands

| Command | Description |
|---------|-------------|
| `cargo new` | Create new project |
| `cargo build` | Compile project |
| `cargo run` | Compile and run |
| `cargo test` | Run tests |
| `cargo doc --open` | Generate and view docs |
| `cargo update` | Update dependencies |
| `cargo add <crate>` | Add a dependency |

---

## Slide 15: Project Structure Best Practices

```
my_project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs        # Binary entry point
│   ├── lib.rs         # Library root (optional)
│   └── module_name/   # Module directory
│       └── mod.rs
├── tests/             # Integration tests
├── examples/          # Example programs
└── benches/           # Benchmarks
```

---

## Slide 16: Re-exporting with pub use

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Re-export for cleaner public API
pub use front_of_house::hosting;

// External code can now use:
// my_crate::hosting::add_to_waitlist()
```

---

## Slide 17: Key Takeaways

1. **Packages** contain crates, managed by Cargo
2. **Crates** are compilation units (binary or library)
3. **Modules** organize code with `mod`
4. Items are **private by default** - use `pub` to expose
5. **use** brings items into scope
6. **Cargo.toml** defines dependencies
7. **crates.io** is the package registry

---

## Slide 18: Lab Preview

**Lab 3: Modules and Crates** (35 min)

You will:
- Create modules in a single file
- Split modules into multiple files
- Control visibility with pub
- Add external crate dependencies
- Build a simple library crate

---

## Questions?
