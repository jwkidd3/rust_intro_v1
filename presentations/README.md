# Introduction to Rust - Presentations

This directory contains the slide decks for the 6-module Introduction to Rust course.

## Presentation Files

| Module | File | Duration | Topics |
|--------|------|----------|--------|
| 1 | `module-01-getting-started.html` | 25 min | Rust overview, installation, Cargo, Hello World |
| 2 | `module-02-language-essentials.html` | 85 min | Variables, types, control flow, functions, collections |
| 3 | `module-03-organizing-code.html` | 30 min | Modules, packages, crates, Cargo |
| 4 | `module-04-error-handling.html` | 30 min | panic!, Result, ? operator |
| 5 | `module-05-object-orientation.html` | 60 min | Structs, methods, traits, generics, patterns |
| 6 | `module-06-functional-programming.html` | 55 min | Closures, capture modes, iterators |

**Total Presentation Time:** ~4.75 hours

## Course Structure

The course is organized into 6 modules aligned with the PDF outline:

### Module 1: Getting Started with Rust
- What is Rust and why use it
- Installing Rust with rustup
- Cargo overview
- Hello World walkthrough
- IDE setup with rust-analyzer

### Module 2: Rust Language Essentials
- Variables, mutability, and constants
- Scalar and compound data types
- Control flow: if/else, loops
- Functions and expressions
- Collections (Vec, String, HashMap)

### Module 3: Organizing Rust Code
- Modules and visibility
- Packages and crates
- Using external dependencies
- Cargo features

### Module 4: Error Handling
- panic! for unrecoverable errors
- Result<T, E> type
- The ? operator
- Error propagation patterns

### Module 5: Object Orientation
- Structs and tuple structs
- Methods and associated functions
- Traits and default implementations
- Generics and trait bounds
- Common design patterns (Builder, Newtype)

### Module 6: Functional Programming
- Closures and capture modes
- Fn, FnMut, FnOnce traits
- Iterators and lazy evaluation
- Iterator adapters (map, filter, take, skip)
- Consumers (collect, sum, fold, find)

## Presentation Format

Each presentation is built using reveal.js 4.5.0 with:
- White theme with clean styling
- Syntax highlighting via highlight.js
- Vertical slide sections for organized content
- Slide numbers displayed as current/total

### Viewing Presentations

Open any `.html` file directly in a web browser.

**Navigation:**
- **Arrow keys** or **Space**: Navigate between slides
- **Down arrow**: Navigate to vertical sub-slides
- **Esc**: Overview mode
- **F**: Fullscreen mode
- **S**: Speaker notes (if available)

## Alignment with Labs

Each presentation module is designed to precede its corresponding lab(s):

| Presentation | Lab(s) |
|--------------|--------|
| Module 1: Getting Started | Lab 1: Environment Setup |
| Module 2: Language Essentials | Lab 2: Variables and Types |
| Module 2: Language Essentials | Lab 3: Functions and Control Flow |
| Module 2: Language Essentials | Lab 4: Collections |
| Module 3: Organizing Code | Lab 5: Modules and Crates |
| Module 4: Error Handling | Lab 6: Error Handling |
| Module 5: Object Orientation | Lab 7: Structs and Methods |
| Module 5: Object Orientation | Lab 8: Traits and Generics |
| Module 6: Functional Programming | Lab 9: Closures |
| Module 6: Functional Programming | Lab 10: Iterators |

## Code Examples

All code examples in the presentations are valid Rust code that can be:
- Copied and run directly
- Used as starting points for labs
- Modified for experimentation

## Prerequisites

Before presenting:
1. Ensure Rust toolchain is installed (`rustup`)
2. Have VS Code with rust-analyzer ready for demos
3. Review the corresponding lab exercises
4. Test all code examples

## Contributing

When modifying presentations:
- Maintain consistent slide structure and styling
- Test all code examples
- Update the README if adding new modules
- Ensure concepts are introduced before being used in labs
