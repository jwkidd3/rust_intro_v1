# Module 1: Getting Started with Rust

**Duration:** 25 minutes

---

## Slide 1: Title

**Getting Started with Rust**

- Introduction to Rust Programming
- Module 1 of 6

---

## Slide 2: What is Rust?

**A systems programming language focused on:**

- **Safety** - Memory safety without garbage collection
- **Speed** - Zero-cost abstractions, no runtime overhead
- **Concurrency** - Fearless concurrent programming

**Created by:** Mozilla Research (2010)
**First stable release:** 2015

---

## Slide 3: What Can I Do with Rust?

**Systems Programming:**
- Operating systems, device drivers
- Embedded systems

**Web Development:**
- Backend services (actix-web, axum)
- WebAssembly

**Command Line Tools:**
- ripgrep, exa, bat

**Production Use:**
- Firefox, Linux kernel, Discord, Cloudflare, AWS

---

## Slide 4: Why Rust?

**The Problem with C/C++:**
- Manual memory management
- Buffer overflows, use-after-free bugs
- ~70% of security vulnerabilities are memory issues

**Rust's Solution:**
- Compile-time memory safety guarantees
- No null pointers, no dangling references
- Same performance as C/C++

---

## Slide 5: What Tools Do I Need?

**rustup** - Toolchain installer and manager
- Installs and updates Rust
- Manages multiple versions

**rustc** - The Rust compiler
- Compiles Rust source code
- Produces native executables

**cargo** - Package manager and build tool
- Creates projects
- Manages dependencies
- Builds, tests, and runs code

---

## Slide 6: Installing Rust

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
- Download rustup-init.exe from https://rustup.rs

**Verify installation:**
```bash
rustc --version
cargo --version
```

---

## Slide 7: IDE Setup

**Recommended: VS Code + rust-analyzer**

1. Install Visual Studio Code
2. Install rust-analyzer extension
3. Open a Rust project folder

**Features:**
- Code completion
- Inline type hints
- Error highlighting
- Go to definition

**Alternatives:** IntelliJ IDEA + Rust plugin, vim/neovim

---

## Slide 8: Creating a Project

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

## Slide 9: Cargo.toml

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

## Slide 10: Hello World

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

---

## Slide 11: Building and Running

**Build only:**
```bash
cargo build
```

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

## Slide 12: Debug vs Release

**Debug build (default):**
```bash
cargo build
```
- Fast compilation, no optimizations
- Output: `target/debug/`

**Release build:**
```bash
cargo build --release
```
- Slower compilation, full optimizations
- Output: `target/release/`

---

## Slide 13: Cargo Commands Summary

| Command | Description |
|---------|-------------|
| `cargo new <name>` | Create new project |
| `cargo build` | Compile the project |
| `cargo run` | Compile and run |
| `cargo check` | Check for errors (fast) |
| `cargo test` | Run tests |
| `cargo doc` | Generate documentation |

---

## Slide 14: Key Takeaways

1. **Rust provides memory safety without garbage collection**
2. **rustup** manages the Rust toolchain
3. **Cargo** is the build tool and package manager
4. **Projects have a standard structure** (Cargo.toml + src/)
5. **Use `cargo run`** for quick iteration
6. **Use `cargo check`** for fast error checking

---

## Slide 15: Lab Preview

**Lab 1: Environment Setup** (30 min)

You will:
- Install Rust using rustup
- Configure VS Code with rust-analyzer
- Create your first Cargo project
- Build and run a program
- Explore Cargo commands

---

## Questions?

**Resources:**
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
