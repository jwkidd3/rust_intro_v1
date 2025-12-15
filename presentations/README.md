# Introduction to Rust - Presentations

This directory contains the slide decks for the 12-module Introduction to Rust course.

## Presentation Files

| Module | File | Duration | Topics |
|--------|------|----------|--------|
| 1 | `module01_getting_started.md` | 45 min | Rust overview, installation, Cargo, Hello World |
| 2 | `module02_variables_data_types.md` | 45 min | Variables, mutability, scalar/compound types |
| 3 | `module03_functions_control_flow.md` | 45 min | Functions, if/else, loops |
| 4 | `module04_ownership.md` | 50 min | Ownership rules, move semantics, Copy/Clone |
| 5 | `module05_references_borrowing.md` | 45 min | References, borrowing rules, slices |
| 6 | `module06_structs_methods.md` | 50 min | Structs, tuple structs, impl blocks, methods |
| 7 | `module07_enums_pattern_matching.md` | 50 min | Enums, Option, match, if let |
| 8 | `module08_error_handling.md` | 45 min | panic!, Result, ?, error propagation |
| 9 | `module09_collections_iterators.md` | 55 min | Vec, String, HashMap, iterators, closures |
| 10 | `module10_modules_crates.md` | 45 min | Modules, visibility, use, crates |
| 11 | `module11_generics_traits.md` | 55 min | Generics, traits, trait bounds, derive |
| 12 | `module12_lifetimes_testing.md` | 55 min | Lifetimes, unit tests, integration tests |

**Total Presentation Time:** ~9 hours

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

Each presentation is designed to precede its corresponding lab:

| Presentation | Lab |
|--------------|-----|
| Module 1: Getting Started | Lab 1: Environment Setup |
| Module 2: Variables and Data Types | Lab 2: Variables and Data Types |
| Module 3: Functions and Control Flow | Lab 3: Functions and Control Flow |
| Module 4: Ownership Basics | Lab 4: Ownership |
| Module 5: References and Borrowing | Lab 5: References and Borrowing |
| Module 6: Structs and Methods | Lab 6: Structs and Methods |
| Module 7: Enums and Pattern Matching | Lab 7: Enums and Pattern Matching |
| Module 8: Error Handling | Lab 8: Error Handling |
| Module 9: Collections and Iterators | Lab 9: Collections and Iterators |
| Module 10: Modules and Crates | Lab 10: Modules and Crates |
| Module 11: Generics and Traits | Lab 11: Generics and Traits |
| Module 12: Lifetimes and Testing | Lab 12: Lifetimes and Testing |

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
