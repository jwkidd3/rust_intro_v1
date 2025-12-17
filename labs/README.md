# Introduction to Rust - Labs

This directory contains all hands-on labs for the Introduction to Rust 2-day course.

## Lab Overview

| Lab | Title | Duration | Topics |
|-----|-------|----------|--------|
| 01 | Environment Setup | 30 min | Install Rust, Cargo basics, first project |
| 02 | Variables and Types | 35 min | Variables, mutability, scalar/compound types |
| 03 | Functions and Control Flow | 35 min | Functions, if/else, loops |
| 04 | Collections | 30 min | Vectors, Strings, HashMaps |
| 05 | Modules and Crates | 35 min | Module system, visibility, external crates |
| 06 | Error Handling | 35 min | panic!, Result, ? operator |
| 07 | Structs and Methods | 35 min | Structs, impl blocks, associated functions |
| 08 | Traits and Generics | 35 min | Traits, generics, trait bounds, patterns |
| 09 | Closures | 30 min | Closure syntax, capture modes, Fn traits |
| 10 | Iterators | 35 min | Iterator methods, map, filter, fold |

**Total Lab Time:** ~5.5 hours

## Lab Descriptions

### Lab 1: Environment Setup
**File:** `lab01_environment_setup.md`

Set up your Rust development environment:
- Install Rust via rustup
- Configure IDE with rust-analyzer
- Create and run first Cargo project
- Understand project structure

---

### Lab 2: Variables and Types
**File:** `lab02_variables_types.md`

Learn Rust's type system:
- Mutable vs immutable variables
- Constants and shadowing
- Integer, float, boolean, char types
- Tuples and arrays

---

### Lab 3: Functions and Control Flow
**File:** `lab03_functions_control_flow.md`

Master Rust control structures:
- Function parameters and returns
- Statements vs expressions
- if/else as expressions
- loop, while, and for loops

---

### Lab 4: Collections
**File:** `lab04_collections.md`

Work with Rust's collection types:
- Vec<T> for dynamic arrays
- String manipulation
- HashMap for key-value storage
- Iterating over collections

---

### Lab 5: Modules and Crates
**File:** `lab05_modules_crates.md`

Organize your code:
- Module system and visibility
- Separating into files
- Using external crates from crates.io
- Cargo.toml configuration

---

### Lab 6: Error Handling
**File:** `lab06_error_handling.md`

Handle errors gracefully:
- panic! for unrecoverable errors
- Result<T, E> for recoverable errors
- The ? operator for propagation
- unwrap, expect, and unwrap_or

---

### Lab 7: Structs and Methods
**File:** `lab07_structs_methods.md`

Structure your data:
- Defining structs with named fields
- Tuple structs
- Implementing methods with impl
- Associated functions (constructors)
- Debug, Clone, PartialEq derives

---

### Lab 8: Traits and Generics
**File:** `lab08_traits_generics.md`

Write reusable, polymorphic code:
- Defining and implementing traits
- Default implementations
- Generic functions and structs
- Trait bounds and where clauses
- Builder and Newtype patterns

---

### Lab 9: Closures
**File:** `lab09_closures.md`

Master functional programming with closures:
- Closure syntax variations
- Capturing environment variables
- Move keyword for ownership transfer
- Fn, FnMut, FnOnce traits
- Closures as function parameters

---

### Lab 10: Iterators
**File:** `lab10_iterators.md`

Process data functionally:
- Creating iterators (iter, iter_mut, into_iter)
- map and filter transformations
- take, skip, enumerate, zip
- find, any, all predicates
- fold for accumulation
- sum, product, count, max, min

---

## Prerequisites

### Software Requirements
- Internet connection for installation
- Terminal/Command Prompt access
- IDE (VS Code recommended)
- 4GB+ RAM

### Knowledge Requirements
- Basic programming experience in any language
- Familiarity with command line usage
- Understanding of basic data structures

## Getting Started

1. **Start with Lab 1** - Set up your environment
2. **Progress sequentially** - Labs build on each other
3. **Complete all exercises** - Don't skip ahead
4. **Run the code** - Type it, don't copy-paste
5. **Use the verification checklist** - Confirm completion

## Lab Format

Each lab includes:
- **Duration** - Expected completion time
- **Objectives** - Learning goals
- **Prerequisites** - Required prior labs
- **Setup** - Project creation steps
- **Exercises** - Hands-on coding tasks with solutions
- **Challenge Exercise** - Optional bonus work
- **Verification Checklist** - Confirm all skills practiced
- **Summary** - Key takeaways

## Running the Labs

Each lab creates a new Cargo project:

```bash
cargo new labXX_name
cd labXX_name
# Edit src/main.rs
cargo run
```

## Tips for Success

1. **Type the code** - Muscle memory helps learning
2. **Read error messages** - Rust's compiler is helpful
3. **Experiment** - Try variations of examples
4. **Use `cargo check`** - Fast feedback without full compile
5. **Complete the checklist** - Verify each skill

## Common Commands

```bash
cargo new project_name    # Create new project
cargo build              # Compile project
cargo run                # Compile and run
cargo check              # Fast syntax check
cargo test               # Run tests
cargo doc --open         # Generate and view docs
```

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

## Troubleshooting

### Rust not found
```bash
source $HOME/.cargo/env
# Or restart your terminal
```

### rust-analyzer not working
1. Open project folder (not individual files)
2. Ensure extension is enabled
3. Restart VS Code

### Compilation errors
1. Read the error message carefully
2. Check line numbers indicated
3. Look at the suggested fix (often provided)

## After the Course

Continue learning with:
- Build a CLI tool with `clap`
- Explore async Rust with `tokio`
- Learn about smart pointers
- Contribute to open source

Good luck with your Rust journey!
