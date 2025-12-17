# Introduction to Rust - Course Flow

## 2-Day Schedule Overview

### Day 1: Rust Fundamentals (Modules 1-6)
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 1 | Getting Started with Rust | Lab 1: Environment Setup | 20 min + 25 min |
| 9:45 | 2 | Variables and Data Types | Lab 2: Variables and Data Types | 25 min + 30 min |
| 10:40 | | *Break* | | 15 min |
| 10:55 | 3 | Functions and Control Flow | Lab 3: Functions and Control Flow | 25 min + 30 min |
| 11:50 | 4 | Ownership Basics | Lab 4: Ownership | 30 min + 30 min |
| 12:50 | | *Lunch* | | 45 min |
| 1:35 | 5 | References and Borrowing | Lab 5: References and Borrowing | 25 min + 30 min |
| 2:30 | | *Break* | | 10 min |
| 2:40 | 6 | Structs and Methods | Lab 6: Structs and Methods | 25 min + 30 min |
| 3:35 | | Day 1 Review & Q&A | | 25 min |
| 4:00 | | *Day 1 End* | | |

### Day 2: Advanced Topics (Modules 7-12)
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 7 | Enums and Pattern Matching | Lab 7: Enums and Pattern Matching | 25 min + 30 min |
| 9:55 | 8 | Error Handling | Lab 8: Error Handling | 25 min + 30 min |
| 10:50 | | *Break* | | 15 min |
| 11:05 | 9 | Collections and Iterators | Lab 9: Collections | 30 min + 30 min |
| 12:05 | | *Lunch* | | 45 min |
| 12:50 | 10 | Modules and Crates | Lab 10: Modules and Crates | 25 min + 30 min |
| 1:45 | 11 | Generics and Traits | Lab 11: Generics and Traits | 30 min + 30 min |
| 2:45 | | *Break* | | 10 min |
| 2:55 | 12 | Lifetimes and Testing | Lab 12: Lifetimes and Testing | 30 min + 30 min |
| 3:55 | | Course Wrap-up & Q&A | | 20 min |
| 4:15 | | *Day 2 End* | | |

---

## Time Summary

| | Presentations | Labs | Breaks/Lunch | Total |
|---|---------------|------|--------------|-------|
| Day 1 | 2h 30min | 2h 55min | 1h 35min | 7h |
| Day 2 | 2h 45min | 3h | 1h 30min | 7h 15min |
| **Total** | **5h 15min** | **5h 55min** | **~3h** | **~14h** |

---

## Detailed Flow

### Day 1

#### Module 1: Getting Started with Rust
**Presentation (20 min):**
- Installing Rust and toolchain (rustup, cargo, rustc)
- IDE setup and rust-analyzer
- Hello World and Cargo basics

**Lab 1: Environment Setup (25 min)**
- Install Rust using rustup
- Configure IDE with rust-analyzer
- Create a new project with `cargo new`
- Build and run programs with Cargo

---

#### Module 2: Variables and Data Types
**Presentation (25 min):**
- Variables and mutability
- Constants and shadowing
- Scalar types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)

**Lab 2: Variables and Data Types (30 min)**
- Declare mutable and immutable variables
- Use constants and shadowing
- Work with scalar and compound types
- Apply type annotations

---

#### Module 3: Functions and Control Flow
**Presentation (25 min):**
- Function parameters and return values
- Statements vs expressions
- if/else conditional expressions
- Loops: loop, while, for

**Lab 3: Functions and Control Flow (30 min)**
- Write functions with parameters and returns
- Use if/else as expressions
- Implement various loop types
- Iterate over collections

---

#### Module 4: Ownership Basics
**Presentation (30 min):**
- What is ownership and why it matters
- The three ownership rules
- Move semantics and Copy trait
- Stack vs heap allocation

**Lab 4: Ownership (30 min)**
- Understand move semantics
- Use Clone for deep copies
- Track ownership through function calls
- Identify ownership errors

---

#### Module 5: References and Borrowing
**Presentation (25 min):**
- References as non-owning pointers
- Mutable vs immutable references
- Borrowing rules
- The slice type

**Lab 5: References and Borrowing (30 min)**
- Create immutable and mutable references
- Apply borrowing rules
- Work with string and array slices
- Pass references to functions

---

#### Module 6: Structs and Methods
**Presentation (25 min):**
- Defining and instantiating structs
- Methods with impl blocks
- Associated functions

**Lab 6: Structs and Methods (30 min)**
- Define structs with named fields
- Implement methods using impl
- Create associated functions (constructors)
- Use tuple structs

---

### Day 2

#### Module 7: Enums and Pattern Matching
**Presentation (25 min):**
- Enum definitions with data
- Pattern matching with match
- The Option enum
- if let for concise matching

**Lab 7: Enums and Pattern Matching (30 min)**
- Define enums with variants
- Use match for exhaustive handling
- Work with Option<T>
- Apply if let syntax

---

#### Module 8: Error Handling
**Presentation (25 min):**
- Recoverable vs unrecoverable errors
- panic! and when to use it
- Result<T, E> and the ? operator

**Lab 8: Error Handling (30 min)**
- Use panic!, unwrap, and expect
- Handle errors with Result
- Propagate errors with ?
- Create functions returning Result

---

#### Module 9: Collections and Iterators
**Presentation (30 min):**
- Vec<T>, String, and HashMap
- Closures
- Iterator trait and methods

**Lab 9: Collections (30 min)**
- Create and manipulate vectors
- Work with String and &str
- Use HashMap for key-value storage
- Chain iterator methods with closures

---

#### Module 10: Modules and Crates
**Presentation (25 min):**
- Module system and visibility
- Separating modules into files
- Using external crates

**Lab 10: Modules and Crates (30 min)**
- Organize code into modules
- Control visibility with pub
- Add external crate dependencies
- Create a library crate

---

#### Module 11: Generics and Traits
**Presentation (30 min):**
- Generic types and functions
- Defining and implementing traits
- Trait bounds
- Standard library traits

**Lab 11: Generics and Traits (30 min)**
- Write generic functions and structs
- Define and implement traits
- Use trait bounds
- Implement Display, Debug, Clone

---

#### Module 12: Lifetimes and Testing
**Presentation (30 min):**
- Why lifetimes exist
- Lifetime annotation syntax
- Unit and integration tests

**Lab 12: Lifetimes and Testing (30 min)**
- Add lifetime annotations
- Write unit tests with #[test]
- Create integration tests

---

## Notes for Instructors

- **Labs are priority** - If running short on time, trim presentation not lab
- **Ownership/Borrowing:** Modules 4-5 are critical - allow extra lab time if needed
- **Live coding:** During presentations, keep examples brief; students will practice in labs
- **Breaks:** Can be shortened by 5 min if needed
- **Optional Topics:** Smart pointers, concurrency - only if significantly ahead
