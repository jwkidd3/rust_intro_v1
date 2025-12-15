# Module 1: Getting Started with Rust

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**Getting Started with Rust**

- Welcome to Introduction to Rust
- 2-Day Hands-on Course
- Module 1 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand what Rust is and why it matters
- Install Rust using rustup
- Set up your development environment
- Create and run your first Rust program
- Understand Cargo basics

---

## Slide 3: What is Rust?

**A systems programming language focused on:**

- **Safety** - Memory safety without garbage collection
- **Speed** - Zero-cost abstractions, no runtime
- **Concurrency** - Fearless concurrent programming

**Created by:** Mozilla Research (2010)
**First stable release:** 2015
**Current adoption:** Firefox, Linux kernel, Discord, Cloudflare, AWS

---

## Slide 4: Why Rust?

**The Problem with C/C++:**
- Manual memory management
- Buffer overflows, use-after-free bugs
- Data races in concurrent code
- ~70% of security vulnerabilities are memory safety issues

**Rust's Solution:**
- Compile-time memory safety guarantees
- No null pointers, no dangling references
- Thread safety guaranteed by the compiler
- Same performance as C/C++

---

## Slide 5: Rust's Key Features

```
┌─────────────────────────────────────────────┐
│            Rust's Three Pillars             │
├─────────────────┬─────────────┬─────────────┤
│     Safety      │    Speed    │ Concurrency │
├─────────────────┼─────────────┼─────────────┤
│ • Ownership     │ • No GC     │ • No data   │
│ • Borrowing     │ • Zero-cost │   races     │
│ • Lifetimes     │   abstracts │ • Send/Sync │
│ • No null       │ • LLVM      │   traits    │
└─────────────────┴─────────────┴─────────────┘
```

---

## Slide 6: The Rust Toolchain

**rustup** - Toolchain installer and manager
- Installs and updates Rust
- Manages multiple versions
- Handles cross-compilation targets

**rustc** - The Rust compiler
- Compiles Rust source code
- Produces native executables

**cargo** - Package manager and build tool
- Creates projects
- Manages dependencies
- Builds, tests, and runs code

---

## Slide 7: Installing Rust

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
- Download rustup-init.exe from https://rustup.rs
- Run the installer

**Verify installation:**
```bash
rustc --version
cargo --version
```

---

## Slide 8: IDE Setup

**Recommended: VS Code + rust-analyzer**

1. Install Visual Studio Code
2. Install rust-analyzer extension
3. Open a Rust project folder

**Features:**
- Code completion
- Inline type hints
- Error highlighting
- Go to definition
- Refactoring tools

**Alternatives:** IntelliJ IDEA + Rust plugin, vim/neovim + rust.vim

---

## Slide 9: Your First Rust Program

**Create a new project:**
```bash
cargo new hello_rust
cd hello_rust
```

**Project structure:**
```
hello_rust/
├── Cargo.toml    # Project manifest
└── src/
    └── main.rs   # Source code
```

---

## Slide 10: Cargo.toml

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# External crates go here
```

**Key sections:**
- `[package]` - Project metadata
- `[dependencies]` - External libraries
- `edition` - Rust edition (2015, 2018, 2021)

---

## Slide 11: Hello World

**src/main.rs:**
```rust
fn main() {
    println!("Hello, world!");
}
```

**Key observations:**
- `fn` defines a function
- `main` is the entry point
- `println!` is a macro (note the `!`)
- Statements end with semicolons
- Curly braces define blocks

---

## Slide 12: Building and Running

**Build only:**
```bash
cargo build
```
Creates: `./target/debug/hello_rust`

**Build and run:**
```bash
cargo run
```

**Check without building:**
```bash
cargo check
```
Faster - useful during development

---

## Slide 13: Debug vs Release Builds

**Debug build (default):**
```bash
cargo build
```
- Fast compilation
- No optimizations
- Includes debug symbols
- Output: `target/debug/`

**Release build:**
```bash
cargo build --release
```
- Slower compilation
- Full optimizations
- Smaller, faster binary
- Output: `target/release/`

---

## Slide 14: Cargo Commands Summary

| Command | Description |
|---------|-------------|
| `cargo new <name>` | Create new project |
| `cargo build` | Compile the project |
| `cargo run` | Compile and run |
| `cargo check` | Check for errors (fast) |
| `cargo test` | Run tests |
| `cargo doc` | Generate documentation |
| `cargo update` | Update dependencies |
| `cargo clean` | Remove build artifacts |

---

## Slide 15: Comments in Rust

```rust
fn main() {
    // This is a line comment

    /* This is a
       block comment */

    /// This is a documentation comment
    /// It supports Markdown!

    //! This documents the enclosing item
    //! (used at the top of modules)
}
```

---

## Slide 16: println! Macro

```rust
fn main() {
    // Basic output
    println!("Hello, world!");

    // With placeholders
    println!("The answer is {}", 42);

    // Multiple values
    println!("{} + {} = {}", 2, 3, 5);

    // Debug formatting
    println!("Debug: {:?}", (1, 2, 3));

    // Named arguments
    println!("{name} is {age} years old", name="Alice", age=30);
}
```

---

## Slide 17: Rust Editions

**What are editions?**
- Periodic releases that can introduce breaking changes
- Opt-in via `Cargo.toml`
- Code from different editions can interoperate

**Available editions:**
- **2015** - Original Rust
- **2018** - Module system changes, async/await
- **2021** - Latest, recommended for new projects

```toml
[package]
edition = "2021"
```

---

## Slide 18: The Rust Ecosystem

**crates.io** - Package registry
- 100,000+ crates available
- Easy to publish and consume

**docs.rs** - Documentation hosting
- Auto-generated docs for all crates

**The Rust Book** - Official learning resource
- https://doc.rust-lang.org/book/

**Rust Playground** - Online compiler
- https://play.rust-lang.org/

---

## Slide 19: Key Takeaways

1. **Rust provides memory safety without garbage collection**
2. **rustup manages the Rust toolchain**
3. **Cargo is the build tool and package manager**
4. **Projects have a standard structure** (Cargo.toml + src/)
5. **Use `cargo run` for quick iteration**
6. **Use `cargo check` for fast error checking**

---

## Slide 20: Lab Preview

**Lab 1: Environment Setup** (20 minutes)

You will:
- Install Rust using rustup
- Configure VS Code with rust-analyzer
- Create your first Cargo project
- Build and run a program
- Modify and re-run the program

---

## Questions?

**Resources:**
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Official Website: https://www.rust-lang.org/
