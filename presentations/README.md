# Introduction to Rust - Presentations

This directory contains the slide decks for the 6-module Introduction to Rust course.

## Presentation Files

| Module | File | Duration | Topics |
|--------|------|----------|--------|
| 1 | `module01_getting_started.md` | 45 min | Rust overview, installation, Cargo, Hello World |
| 2 | `module02_language_essentials.md` | 75 min | Variables, types, control flow, functions, collections |
| 3 | `module03_organizing_code.md` | 30 min | Modules, packages, crates, Cargo |
| 4 | `module04_error_handling.md` | 35 min | panic!, Result, ? operator |
| 5 | `module05_object_orientation.md` | 60 min | Structs, methods, traits, generics, patterns |
| 6 | `module06_functional_programming.md` | 45 min | Closures, capture modes, iterators |

**Total Presentation Time:** ~4.75 hours

## Course Structure

The course is organized into 6 modules aligned with the PDF outline:

### Module 1: Getting Started with Rust
- What is Rust and why use it
- Installing Rust with rustup
- Cargo overview
- Hello World walkthrough
- IDE setup with rust-analyzer

### Module 2: Rust Language Essentials (3 parts)
- **Part 1:** Variables, data types
- **Part 2:** Functions, control flow
- **Part 3:** Collections (Vec, String, HashMap)

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

### Module 5: Object Orientation (2 parts)
- **Part 1:** Structs, tuple structs, methods, associated functions
- **Part 2:** Traits, generics, trait bounds, common patterns

### Module 6: Functional Programming (2 parts)
- **Part 1:** Closures, capture modes, Fn traits
- **Part 2:** Iterators, adapters, consumers

## Presentation Format

Each presentation is written in Markdown with slide separators (`---`). The format includes:

- **Title slide** with module name and course position
- **Objectives slide** listing learning goals
- **Content slides** with code examples and diagrams
- **Key takeaways** summary slide
- **Lab preview** slide describing the hands-on exercise
- **Questions** slide with next module reference

## Using These Presentations

### With Presentation Software

These Markdown files can be converted to slides using:
- [Marp](https://marp.app/) - Markdown Presentation Ecosystem
- [reveal.js](https://revealjs.com/) - HTML Presentation Framework
- [Slidev](https://sli.dev/) - Presentation Slides for Developers
- [Deckset](https://www.deckset.com/) - macOS presentation app

### Direct Markdown Viewing

The presentations are also readable directly as Markdown documents, making them useful as:
- Reference material
- Study guides
- Quick lookups during labs

## Code Examples

All code examples in the presentations are valid Rust code that can be:
- Copied and run directly
- Used as starting points for labs
- Modified for experimentation

## Diagrams

ASCII diagrams are used throughout to illustrate:
- Memory layouts (stack vs heap)
- Data structures
- Ownership and borrowing relationships
- Module hierarchies

## Alignment with Labs

Each presentation module is designed to precede its corresponding lab(s):

| Presentation | Lab(s) |
|--------------|--------|
| Module 1: Getting Started | Lab 1: Environment Setup |
| Module 2: Language Essentials (Part 1) | Lab 2: Variables and Types |
| Module 2: Language Essentials (Part 2) | Lab 3: Functions and Control Flow |
| Module 2: Language Essentials (Part 3) | Lab 4: Collections |
| Module 3: Organizing Code | Lab 5: Modules and Crates |
| Module 4: Error Handling | Lab 6: Error Handling |
| Module 5: Object Orientation (Part 1) | Lab 7: Structs and Methods |
| Module 5: Object Orientation (Part 2) | Lab 8: Traits and Generics |
| Module 6: Functional Programming (Part 1) | Lab 9: Closures |
| Module 6: Functional Programming (Part 2) | Lab 10: Iterators |

## Prerequisites

Before presenting:
1. Ensure Rust toolchain is installed (`rustup`)
2. Have VS Code with rust-analyzer ready for demos
3. Review the corresponding lab exercises
4. Test all code examples

## Contributing

When modifying presentations:
- Maintain consistent slide structure
- Test all code examples
- Update the README if adding new modules
- Ensure concepts are introduced before being used in labs
