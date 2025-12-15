# Introduction to Rust - Course Flow

| Module | Presentation Topic | Lab | Duration |
|--------|-------------------|-----|----------|
| 1 | Getting Started with Rust | Lab 1: Environment Setup | 20 min |
| 2 | Variables and Data Types | Lab 2: Variables and Data Types | 25 min |
| 3 | Functions and Control Flow | Lab 3: Functions and Control Flow | 25 min |
| 4 | Ownership Basics | Lab 4: Ownership | 25 min |
| 5 | References and Borrowing | Lab 5: References and Borrowing | 25 min |
| 6 | Structs and Methods | Lab 6: Structs and Methods | 25 min |
| 7 | Enums and Pattern Matching | Lab 7: Enums and Pattern Matching | 25 min |
| 8 | Error Handling | Lab 8: Error Handling | 25 min |
| 9 | Collections and Iterators | Lab 9: Collections | 25 min |
| 10 | Modules and Crates | Lab 10: Modules and Crates | 25 min |
| 11 | Generics and Traits | Lab 11: Generics and Traits | 25 min |
| 12 | Lifetimes and Testing | Lab 12: Lifetimes and Testing | 30 min |

**Total Lab Time:** ~5 hours

---

## Detailed Flow

### Module 1: Getting Started with Rust
**Presentation:**
- Installing Rust and toolchain (rustup, cargo, rustc)
- IDE setup and rust-analyzer
- Hello World and project structure
- Cargo basics: new, build, run, check

**Lab 1: Environment Setup** (20 min)
- Install Rust using rustup
- Configure IDE with rust-analyzer
- Create a new project with `cargo new`
- Build and run programs with Cargo

---

### Module 2: Variables and Data Types
**Presentation:**
- Variables and mutability
- Constants and shadowing
- Scalar types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)

**Lab 2: Variables and Data Types** (25 min)
- Declare mutable and immutable variables
- Use constants and shadowing
- Work with scalar and compound types
- Apply type annotations

---

### Module 3: Functions and Control Flow
**Presentation:**
- Function parameters and return values
- Statements vs expressions
- if/else conditional expressions
- Loops: loop, while, for

**Lab 3: Functions and Control Flow** (25 min)
- Write functions with parameters and returns
- Use if/else as expressions
- Implement various loop types
- Iterate over collections

---

### Module 4: Ownership Basics
**Presentation:**
- What is ownership and why it matters
- The three ownership rules
- Move semantics and Copy trait
- Stack vs heap allocation

**Lab 4: Ownership** (25 min)
- Understand move semantics
- Use Clone for deep copies
- Track ownership through function calls
- Identify ownership errors

---

### Module 5: References and Borrowing
**Presentation:**
- References as non-owning pointers
- Mutable vs immutable references
- Borrowing rules
- The slice type

**Lab 5: References and Borrowing** (25 min)
- Create immutable and mutable references
- Apply borrowing rules
- Work with string and array slices
- Pass references to functions

---

### Module 6: Structs and Methods
**Presentation:**
- Defining and instantiating structs
- Field init shorthand and update syntax
- Methods with impl blocks
- Associated functions

**Lab 6: Structs and Methods** (25 min)
- Define structs with named fields
- Implement methods using impl
- Create associated functions (constructors)
- Use tuple structs

---

### Module 7: Enums and Pattern Matching
**Presentation:**
- Enum definitions with data
- Pattern matching with match
- The Option enum
- if let for concise matching

**Lab 7: Enums and Pattern Matching** (25 min)
- Define enums with variants
- Use match for exhaustive handling
- Work with Option<T>
- Apply if let syntax

---

### Module 8: Error Handling
**Presentation:**
- Recoverable vs unrecoverable errors
- panic! and when to use it
- Result<T, E> and the ? operator
- Error propagation patterns

**Lab 8: Error Handling** (25 min)
- Use panic!, unwrap, and expect
- Handle errors with Result
- Propagate errors with ?
- Create custom error types

---

### Module 9: Collections and Iterators
**Presentation:**
- Vec<T>, String, and HashMap
- Closures and capture modes
- Iterator trait and methods
- Chaining iterator operations

**Lab 9: Collections** (25 min)
- Create and manipulate vectors
- Work with String and &str
- Use HashMap for key-value storage
- Chain iterator methods with closures

---

### Module 10: Modules and Crates
**Presentation:**
- Module system and visibility
- Separating modules into files
- Using external crates (crates.io)
- Creating library crates

**Lab 10: Modules and Crates** (25 min)
- Organize code into modules
- Control visibility with pub
- Add external crate dependencies
- Create a library crate

---

### Module 11: Generics and Traits
**Presentation:**
- Generic types and functions
- Defining and implementing traits
- Trait bounds and where clauses
- Standard library traits

**Lab 11: Generics and Traits** (25 min)
- Write generic functions and structs
- Define and implement traits
- Use trait bounds
- Implement Display, Debug, Clone

---

### Module 12: Lifetimes and Testing
**Presentation:**
- Why lifetimes exist
- Lifetime annotation syntax
- Lifetime elision rules
- Unit and integration tests

**Lab 12: Lifetimes and Testing** (30 min)
- Add lifetime annotations
- Understand elision rules
- Write unit tests with #[test]
- Create integration tests
