# Introduction to Rust - Labs

This directory contains all 12 hands-on labs for the Introduction to Rust 2-day course.

## Lab Overview

| Lab | Title | Duration | Topics |
|-----|-------|----------|--------|
| 01 | Environment Setup | 20 min | Install Rust, Cargo basics, first project |
| 02 | Variables and Data Types | 25 min | Variables, mutability, scalar/compound types |
| 03 | Functions and Control Flow | 25 min | Functions, if/else, loops |
| 04 | Ownership Basics | 25 min | Ownership rules, move semantics, Copy/Clone |
| 05 | References and Borrowing | 25 min | References, borrowing rules, slices |
| 06 | Structs and Methods | 25 min | Structs, impl blocks, associated functions |
| 07 | Enums and Pattern Matching | 25 min | Enums, match, Option, if let |
| 08 | Error Handling | 25 min | panic!, Result, ? operator |
| 09 | Collections and Iterators | 25 min | Vec, String, HashMap, closures, iterators |
| 10 | Modules and Crates | 25 min | Module system, visibility, external crates |
| 11 | Generics and Traits | 25 min | Generic types, trait definitions, trait bounds |
| 12 | Lifetimes and Testing | 30 min | Lifetime annotations, unit/integration tests |

**Total Lab Time:** ~5 hours

## Lab Descriptions

### Lab 1: Environment Setup
**File:** `lab01_environment_setup.md`

Set up your Rust development environment:
- Install Rust via rustup
- Configure IDE with rust-analyzer
- Create and run first Cargo project
- Understand project structure

---

### Lab 2: Variables and Data Types
**File:** `lab02_variables_data_types.md`

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

### Lab 4: Ownership Basics
**File:** `lab04_ownership.md`

Understand Rust's unique ownership system:
- The three ownership rules
- Move semantics with heap data
- Copy vs Clone traits
- Ownership in function calls

---

### Lab 5: References and Borrowing
**File:** `lab05_references_borrowing.md`

Work with references safely:
- Immutable and mutable references
- Borrowing rules
- String and array slices
- The slice type &str

---

### Lab 6: Structs and Methods
**File:** `lab06_structs_methods.md`

Structure your data:
- Defining structs
- Field init shorthand
- Implementing methods with impl
- Associated functions (constructors)

---

### Lab 7: Enums and Pattern Matching
**File:** `lab07_enums_pattern_matching.md`

Handle variants and patterns:
- Enum definitions with data
- Match expressions
- Option<T> for nullable values
- if let for concise matching

---

### Lab 8: Error Handling
**File:** `lab08_error_handling.md`

Handle errors gracefully:
- panic! for unrecoverable errors
- Result<T, E> for recoverable errors
- The ? operator
- Custom error types

---

### Lab 9: Collections and Iterators
**File:** `lab09_collections.md`

Process data efficiently:
- Vec<T> for dynamic arrays
- String manipulation
- HashMap for key-value storage
- Iterator methods and closures

---

### Lab 10: Modules and Crates
**File:** `lab10_modules_crates.md`

Organize your code:
- Module system and visibility
- Separating into files
- Using external crates
- Creating library crates

---

### Lab 11: Generics and Traits
**File:** `lab11_generics_traits.md`

Write reusable code:
- Generic functions and structs
- Defining and implementing traits
- Trait bounds
- Standard library traits

---

### Lab 12: Lifetimes and Testing
**File:** `lab12_lifetimes_testing.md`

Ensure validity and correctness:
- Lifetime annotations
- Lifetime elision rules
- Writing unit tests
- Integration tests

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
5. **Answer the questions** - Test your understanding

## Lab Format

Each lab includes:
- **Duration** - Expected completion time
- **Objectives** - Learning goals
- **Prerequisites** - Required knowledge
- **Exercises** - Hands-on coding tasks
- **Verification** - Checklist to confirm completion
- **Questions** - Test your understanding
- **Answers** - Explanations
- **Common Issues** - Troubleshooting
- **Next Steps** - Preview of next lab

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
5. **Ask questions** - Review the Q&A sections

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
