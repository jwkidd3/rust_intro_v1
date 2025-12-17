# Lab 1: Environment Setup

**Duration:** 30 minutes

## Objectives

- Install Rust using rustup
- Configure VS Code with rust-analyzer
- Create your first Cargo project
- Build and run a program
- Explore Cargo commands

## Prerequisites

- Computer with internet access
- Administrator privileges for software installation

---

## Exercise 1: Install Rust (10 min)

### Step 1: Install rustup

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
1. Download rustup-init.exe from https://rustup.rs
2. Run the installer
3. Follow the prompts (accept defaults)

### Step 2: Configure your shell

After installation, either restart your terminal or run:
```bash
source $HOME/.cargo/env
```

### Step 3: Verify installation

```bash
rustc --version
cargo --version
rustup --version
```

You should see version numbers for each tool.

---

## Exercise 2: Set Up VS Code (5 min)

### Step 1: Install VS Code

Download from https://code.visualstudio.com/ if not already installed.

### Step 2: Install rust-analyzer extension

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "rust-analyzer"
4. Click Install

### Step 3: Verify extension

The extension will activate automatically when you open a Rust file.

---

## Exercise 3: Create Your First Project (10 min)

### Step 1: Create a new project

```bash
cargo new hello_rust
cd hello_rust
```

### Step 2: Explore project structure

```bash
ls -la
```

You should see:
```
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### Step 3: Examine Cargo.toml

Open `Cargo.toml` in VS Code:
```bash
code .
```

Notice the package metadata:
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Step 4: Examine main.rs

Open `src/main.rs`:
```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Exercise 4: Build and Run (5 min)

### Step 1: Build the project

```bash
cargo build
```

Notice the `target/debug/` directory was created.

### Step 2: Run the executable directly

```bash
./target/debug/hello_rust
```

### Step 3: Build and run in one command

```bash
cargo run
```

### Step 4: Check for errors without building

```bash
cargo check
```

This is faster than `cargo build` - useful during development.

### Step 5: Build for release

```bash
cargo build --release
```

Check `target/release/` for the optimized binary.

---

## Exercise 5: Modify and Experiment (Bonus)

### Step 1: Modify the program

Edit `src/main.rs`:
```rust
fn main() {
    println!("Hello, Rust!");
    println!("Welcome to the course!");

    let name = "Student";
    println!("Hello, {}!", name);
}
```

### Step 2: Run the modified program

```bash
cargo run
```

### Step 3: Try introducing an error

Change the code to have a typo:
```rust
fn main() {
    printl!("Hello");  // Wrong macro name
}
```

Run `cargo check` and observe the error message.

Fix the error and verify with `cargo run`.

---

## Verification Checklist

- [ ] `rustc --version` shows installed version
- [ ] `cargo --version` shows installed version
- [ ] VS Code has rust-analyzer extension installed
- [ ] Created `hello_rust` project with `cargo new`
- [ ] Successfully ran `cargo build`
- [ ] Successfully ran `cargo run`
- [ ] Successfully ran `cargo check`
- [ ] Modified the program and re-ran it

---

## Common Issues

**"command not found: cargo"**
- Restart your terminal or run `source $HOME/.cargo/env`

**rust-analyzer not working**
- Make sure you opened the project folder (not just a file)
- Try restarting VS Code

**Permission denied on macOS/Linux**
- The curl command should work without sudo
- If issues persist, check https://rustup.rs for alternatives

---

## Summary

You have successfully:
- Installed the Rust toolchain using rustup
- Set up VS Code with rust-analyzer
- Created and built a Rust project with Cargo
- Learned essential Cargo commands

**Next:** Lab 2a - Variables and Types
