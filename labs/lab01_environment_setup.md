# Lab 1: Environment Setup and First Rust Program

## Duration: 20 minutes

## Objectives
- Install Rust using rustup
- Configure your IDE with rust-analyzer
- Create your first Rust project with Cargo
- Build and run a Rust program

## Prerequisites
- Internet connection for installation
- IDE of your choice (VS Code recommended)
- Terminal/Command Prompt access

## Part 1: Installing Rust (10 minutes)

### Step 1: Install Rust via rustup

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run `rustup-init.exe` from https://rustup.rs

### Step 2: Follow the Installation Prompts

Select option 1 for default installation:
```
1) Proceed with installation (default)
```

### Step 3: Configure Your Shell

After installation, either restart your terminal or run:
```bash
source $HOME/.cargo/env
```

### Step 4: Verify Installation

```bash
rustc --version
cargo --version
rustup --version
```

**Expected Output:**
```
rustc 1.XX.0 (commit hash YYYY-MM-DD)
cargo 1.XX.0 (commit hash YYYY-MM-DD)
rustup 1.XX.0 (commit hash YYYY-MM-DD)
```

### Step 5: Install IDE Extension

**VS Code:**
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "rust-analyzer"
4. Click Install

## Part 2: First Rust Project (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new hello_rust
cd hello_rust
```

### Exercise 2: Explore Project Structure

```
hello_rust/
├── Cargo.toml    # Project manifest (dependencies, metadata)
└── src/
    └── main.rs   # Main source file
```

**View `Cargo.toml`:**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**View `src/main.rs`:**
```rust
fn main() {
    println!("Hello, world!");
}
```

### Exercise 3: Build and Run

**Method 1: Build then Run**
```bash
cargo build
./target/debug/hello_rust
```

**Method 2: Build and Run (Combined)**
```bash
cargo run
```

**Expected Output:**
```
Hello, world!
```

### Exercise 4: Modify the Program

Edit `src/main.rs`:

```rust
fn main() {
    println!("Hello, Rust!");
    println!("Welcome to the Rust programming language.");

    // This is a comment
    println!("Learning Rust is fun!");
}
```

Run the modified program:
```bash
cargo run
```

**Expected Output:**
```
Hello, Rust!
Welcome to the Rust programming language.
Learning Rust is fun!
```

### Exercise 5: Use cargo check

```bash
cargo check
```

This compiles but doesn't produce an executable - faster for checking code validity.

### Exercise 6: Create a Release Build

```bash
cargo build --release
```

The optimized binary is in `./target/release/hello_rust`

## Verification Steps

### Checklist
- [ ] Rust installed successfully (`rustc --version` works)
- [ ] Cargo installed successfully (`cargo --version` works)
- [ ] rust-analyzer extension installed in IDE
- [ ] Created hello_rust project with `cargo new`
- [ ] Program compiles and runs with `cargo run`
- [ ] Modified program outputs custom messages
- [ ] Understand difference between `cargo build` and `cargo build --release`

## Lab Questions

1. What is the purpose of rustup?
2. What file contains your project's dependencies?
3. What is the difference between `cargo build` and `cargo build --release`?
4. What does `cargo check` do?

## Answers

1. **rustup** is Rust's toolchain installer and version manager. It installs and manages Rust compilers, cargo, and other tools.

2. **Cargo.toml** contains project metadata and dependencies.

3. `cargo build` creates a debug build with no optimizations (faster compilation). `cargo build --release` creates an optimized build for production (slower compilation, faster execution).

4. `cargo check` quickly checks if code compiles without producing an executable - useful for rapid feedback during development.

## Common Issues

**Issue: "command not found: cargo"**
```
Solution: Restart your terminal or run: source $HOME/.cargo/env
```

**Issue: rust-analyzer not working in VS Code**
```
Solution:
1. Ensure you opened the project folder (not individual files)
2. Restart VS Code
3. Check that rust-analyzer extension is enabled
```

## Next Steps

In Lab 2, you will:
- Learn about variables and mutability
- Work with Rust's data types
- Understand type inference and annotations

## Completion

You have completed Lab 1 when you can:
- Verify Rust installation with `rustc --version`
- Create a new project with `cargo new`
- Build and run a program with `cargo run`
- Make changes and see them reflected in output
