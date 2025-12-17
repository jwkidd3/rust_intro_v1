# Rust Programming – Introduction (Part 1) - Course Flow

## 2-Day Schedule Overview

### Day 1: Getting Started, Language Essentials, Organizing Code
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 1 | Getting Started with Rust | Lab 1: Environment Setup | 25 min + 30 min |
| 9:55 | 2a | Rust Language Essentials (Part 1) | Lab 2a: Variables and Types | 30 min + 35 min |
| 11:00 | | *Break* | | 15 min |
| 11:15 | 2b | Rust Language Essentials (Part 2) | Lab 2b: Functions and Control Flow | 30 min + 35 min |
| 12:35 | | *Lunch* | | 45 min |
| 1:20 | 2c | Rust Language Essentials (Part 3) | Lab 2c: Collections | 25 min + 30 min |
| 2:15 | | *Break* | | 10 min |
| 2:25 | 3 | Organizing Rust Code | Lab 3: Modules and Crates | 30 min + 35 min |
| 3:30 | | Day 1 Review & Q&A | | 30 min |
| 4:00 | | *Day 1 End* | | |

### Day 2: Error Handling, Object Orientation, Functional Programming
| Time | Module | Presentation | Lab | Duration |
|------|--------|--------------|-----|----------|
| 9:00 | 4 | Error Handling | Lab 4: Error Handling | 30 min + 35 min |
| 10:05 | 5a | Object Orientation (Part 1) | Lab 5a: Structs and Methods | 30 min + 35 min |
| 11:10 | | *Break* | | 15 min |
| 11:25 | 5b | Object Orientation (Part 2) | Lab 5b: Traits and Generics | 30 min + 35 min |
| 12:30 | | *Lunch* | | 45 min |
| 1:15 | 6a | Functional Programming (Part 1) | Lab 6a: Closures | 25 min + 30 min |
| 2:10 | | *Break* | | 10 min |
| 2:20 | 6b | Functional Programming (Part 2) | Lab 6b: Iterators | 30 min + 35 min |
| 3:25 | | Course Wrap-up & Q&A | | 35 min |
| 4:00 | | *Day 2 End* | | |

---

## Time Summary

| | Presentations | Labs | Breaks/Lunch | Total |
|---|---------------|------|--------------|-------|
| Day 1 | 2h 20min | 2h 45min | 1h 40min | 7h |
| Day 2 | 2h 25min | 2h 50min | 1h 45min | 7h |
| **Total** | **4h 45min** | **5h 35min** | **~3.5h** | **~14h** |

---

## Detailed Flow

### Day 1

#### Module 1: Getting Started with Rust
**Presentation (25 min):**
- What is Rust and why it matters
- What can I do with Rust (use cases)
- What tools do I need (rustup, cargo, rustc)
- IDE setup and rust-analyzer
- Hello World walkthrough

**Lab 1: Environment Setup (30 min)**
- Install Rust using rustup
- Configure IDE with rust-analyzer
- Create a new project with `cargo new`
- Build and run programs with Cargo
- Explore project structure

---

#### Module 2a: Rust Language Essentials (Part 1 - Types and Variables)
**Presentation (30 min):**
- Variables and mutability
- Constants and shadowing
- Scalar types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)
- Type annotations and inference

**Lab 2a: Variables and Types (35 min)**
- Declare mutable and immutable variables
- Use constants and shadowing
- Work with scalar types
- Create and access tuples and arrays
- Apply type annotations

---

#### Module 2b: Rust Language Essentials (Part 2 - Control Flow and Functions)
**Presentation (30 min):**
- Conditional logic (if/else expressions)
- Iteration (loop, while, for)
- Functions and parameters
- Return values
- Statements vs expressions

**Lab 2b: Functions and Control Flow (35 min)**
- Write functions with parameters and returns
- Use if/else as expressions
- Implement loop, while, and for loops
- Iterate over ranges and collections
- Use break and continue

---

#### Module 2c: Rust Language Essentials (Part 3 - Collections)
**Presentation (25 min):**
- Vec<T> - growable arrays
- String and &str
- HashMap<K, V>
- Common collection operations

**Lab 2c: Collections (30 min)**
- Create and manipulate vectors
- Work with String operations
- Use HashMap for key-value storage
- Iterate over collections

---

#### Module 3: Organizing Rust Code
**Presentation (30 min):**
- Modules and visibility (pub)
- Packages and crates
- The Cargo dependency manager
- Using external crates (crates.io)
- Project structure best practices

**Lab 3: Modules and Crates (35 min)**
- Create modules in a single file
- Split modules into multiple files
- Control visibility with pub
- Add external crate dependencies
- Build a library crate

---

### Day 2

#### Module 4: Error Handling
**Presentation (30 min):**
- Overview of error handling philosophy
- Recoverable errors with Result<T, E>
- Unrecoverable errors with panic!
- The ? operator for propagation
- unwrap, expect, and alternatives

**Lab 4: Error Handling (35 min)**
- Use panic! for unrecoverable errors
- Handle errors with Result and match
- Propagate errors with ?
- Use unwrap and expect appropriately
- Create functions returning Result

---

#### Module 5a: Object Orientation (Part 1 - Structs and Methods)
**Presentation (30 min):**
- Defining structures (structs)
- Tuple structs and unit structs
- Implementing functionality (impl blocks)
- Methods with self, &self, &mut self
- Associated functions (constructors)

**Lab 5a: Structs and Methods (35 min)**
- Define structs with named fields
- Create tuple structs
- Implement methods using impl
- Create associated functions
- Use derive for Debug, Clone

---

#### Module 5b: Object Orientation (Part 2 - Traits and Generics)
**Presentation (30 min):**
- Specifying traits
- Implementing traits for types
- Trait bounds
- Generic types and functions
- Design patterns in Rust (Builder, Newtype)

**Lab 5b: Traits and Generics (35 min)**
- Define custom traits
- Implement traits for structs
- Write generic functions
- Use trait bounds
- Implement common patterns

---

#### Module 6a: Functional Programming (Part 1 - Closures)
**Presentation (25 min):**
- Concepts of functional programming
- Anonymous functions
- Closures and syntax
- Capture modes (borrow, mutable borrow, move)

**Lab 6a: Closures (30 min)**
- Write basic closures
- Use closures with different capture modes
- Pass closures to functions
- Store closures in variables

---

#### Module 6b: Functional Programming (Part 2 - Iterators)
**Presentation (30 min):**
- Iterator trait and methods
- Iterator adapters (map, filter, take, skip)
- Consuming iterators (collect, sum, fold)
- Chaining operations
- Patterns and techniques

**Lab 6b: Iterators (35 min)**
- Create and consume iterators
- Use map and filter
- Chain iterator methods
- Use fold for accumulation
- Process data with functional style

---

## Notes for Instructors

- **Labs are priority** - If running short on time, trim presentation not lab
- **Module 2 is split into 3 parts** - Can combine if students are experienced
- **Ownership concepts** - Introduce briefly as needed; detailed coverage is optional
- **Design patterns** - Keep examples practical and simple
- **Breaks:** Can be shortened by 5 min if needed
