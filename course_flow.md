# Introduction to Rust - Course Flow

## 2-Day Schedule Overview

### Day 1: Rust Fundamentals (Modules 1-6)
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 1 | Getting Started with Rust | Lab 1: Environment Setup | 45 min + 20 min |
| 10:05 | | *Break* | | 15 min |
| 10:20 | 2 | Variables and Data Types | Lab 2: Variables and Data Types | 45 min + 25 min |
| 11:30 | 3 | Functions and Control Flow | Lab 3: Functions and Control Flow | 45 min + 25 min |
| 12:40 | | *Lunch* | | 60 min |
| 1:40 | 4 | Ownership Basics | Lab 4: Ownership | 50 min + 25 min |
| 2:55 | | *Break* | | 15 min |
| 3:10 | 5 | References and Borrowing | Lab 5: References and Borrowing | 45 min + 25 min |
| 4:20 | 6 | Structs and Methods | Lab 6: Structs and Methods | 50 min + 25 min |
| 5:35 | | *Day 1 End* | | |

### Day 2: Advanced Topics (Modules 7-12)
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 7 | Enums and Pattern Matching | Lab 7: Enums and Pattern Matching | 50 min + 25 min |
| 10:15 | | *Break* | | 15 min |
| 10:30 | 8 | Error Handling | Lab 8: Error Handling | 45 min + 25 min |
| 11:40 | 9 | Collections and Iterators | Lab 9: Collections | 55 min + 25 min |
| 1:00 | | *Lunch* | | 60 min |
| 2:00 | 10 | Modules and Crates | Lab 10: Modules and Crates | 45 min + 25 min |
| 3:10 | | *Break* | | 15 min |
| 3:25 | 11 | Generics and Traits | Lab 11: Generics and Traits | 55 min + 25 min |
| 4:45 | 12 | Lifetimes and Testing | Lab 12: Lifetimes and Testing | 55 min + 30 min |
| 6:10 | | *Course End / Q&A* | | |

---

## Time Summary

| | Presentations | Labs | Breaks/Lunch | Total |
|---|---------------|------|--------------|-------|
| Day 1 | 4h 40min | 2h 25min | 1h 30min | ~8.5h |
| Day 2 | 5h 5min | 2h 35min | 1h 30min | ~9h |
| **Total** | **9h 45min** | **5h** | **3h** | **~17.5h** |

---

## Detailed Flow

### Day 1

#### Module 1: Getting Started with Rust
**Presentation (45 min):**
- Installing Rust and toolchain (rustup, cargo, rustc)
- IDE setup and rust-analyzer
- Hello World and project structure
- Cargo basics: new, build, run, check

**Lab 1: Environment Setup (20 min)**
- Install Rust using rustup
- Configure IDE with rust-analyzer
- Create a new project with `cargo new`
- Build and run programs with Cargo

---

#### Module 2: Variables and Data Types
**Presentation (45 min):**
- Variables and mutability
- Constants and shadowing
- Scalar types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)

**Lab 2: Variables and Data Types (25 min)**
- Declare mutable and immutable variables
- Use constants and shadowing
- Work with scalar and compound types
- Apply type annotations

---

#### Module 3: Functions and Control Flow
**Presentation (45 min):**
- Function parameters and return values
- Statements vs expressions
- if/else conditional expressions
- Loops: loop, while, for

**Lab 3: Functions and Control Flow (25 min)**
- Write functions with parameters and returns
- Use if/else as expressions
- Implement various loop types
- Iterate over collections

---

#### Module 4: Ownership Basics
**Presentation (50 min):**
- What is ownership and why it matters
- The three ownership rules
- Move semantics and Copy trait
- Stack vs heap allocation

**Lab 4: Ownership (25 min)**
- Understand move semantics
- Use Clone for deep copies
- Track ownership through function calls
- Identify ownership errors

---

#### Module 5: References and Borrowing
**Presentation (45 min):**
- References as non-owning pointers
- Mutable vs immutable references
- Borrowing rules
- The slice type

**Lab 5: References and Borrowing (25 min)**
- Create immutable and mutable references
- Apply borrowing rules
- Work with string and array slices
- Pass references to functions

---

#### Module 6: Structs and Methods
**Presentation (50 min):**
- Defining and instantiating structs
- Field init shorthand and update syntax
- Methods with impl blocks
- Associated functions

**Lab 6: Structs and Methods (25 min)**
- Define structs with named fields
- Implement methods using impl
- Create associated functions (constructors)
- Use tuple structs

---

### Day 2

#### Module 7: Enums and Pattern Matching
**Presentation (50 min):**
- Enum definitions with data
- Pattern matching with match
- The Option enum
- if let for concise matching

**Lab 7: Enums and Pattern Matching (25 min)**
- Define enums with variants
- Use match for exhaustive handling
- Work with Option<T>
- Apply if let syntax

---

#### Module 8: Error Handling
**Presentation (45 min):**
- Recoverable vs unrecoverable errors
- panic! and when to use it
- Result<T, E> and the ? operator
- Error propagation patterns

**Lab 8: Error Handling (25 min)**
- Use panic!, unwrap, and expect
- Handle errors with Result
- Propagate errors with ?
- Create custom error types

---

#### Module 9: Collections and Iterators
**Presentation (55 min):**
- Vec<T>, String, and HashMap
- Closures and capture modes
- Iterator trait and methods
- Chaining iterator operations

**Lab 9: Collections (25 min)**
- Create and manipulate vectors
- Work with String and &str
- Use HashMap for key-value storage
- Chain iterator methods with closures

---

#### Module 10: Modules and Crates
**Presentation (45 min):**
- Module system and visibility
- Separating modules into files
- Using external crates (crates.io)
- Creating library crates

**Lab 10: Modules and Crates (25 min)**
- Organize code into modules
- Control visibility with pub
- Add external crate dependencies
- Create a library crate

---

#### Module 11: Generics and Traits
**Presentation (55 min):**
- Generic types and functions
- Defining and implementing traits
- Trait bounds and where clauses
- Standard library traits

**Lab 11: Generics and Traits (25 min)**
- Write generic functions and structs
- Define and implement traits
- Use trait bounds
- Implement Display, Debug, Clone

---

#### Module 12: Lifetimes and Testing
**Presentation (55 min):**
- Why lifetimes exist
- Lifetime annotation syntax
- Lifetime elision rules
- Unit and integration tests

**Lab 12: Lifetimes and Testing (30 min)**
- Add lifetime annotations
- Understand elision rules
- Write unit tests with #[test]
- Create integration tests

---

## Notes for Instructors

- **Pacing:** Day 2 is slightly longer; consider adjusting break times if needed
- **Ownership/Borrowing:** Modules 4-5 are critical - allow extra time for questions
- **Labs:** Can be shortened if running behind; core exercises are marked in each lab
- **Optional:** If ahead of schedule, cover smart pointers or concurrency basics
