# Lab 4: Collections

**Duration:** 30 minutes

## Objectives

- Create and manipulate vectors
- Work with String operations
- Use HashMap for key-value storage
- Iterate over collections

## Prerequisites

- Completed Labs 2 and 3

---

## Setup

Create a new project:
```bash
cargo new lab02c_collections
cd lab02c_collections
code .
```

---

## Exercise 1: Vectors (10 min)

### Step 1: Creating vectors

```rust
fn main() {
    // Empty vector with type annotation
    let v1: Vec<i32> = Vec::new();
    println!("v1: {:?}", v1);

    // Using vec! macro
    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    // Building a vector
    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);
    println!("v3: {:?}", v3);
}
```

### Step 2: Accessing elements

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Direct indexing (can panic)
    let third = v[2];
    println!("Third element: {}", third);

    // Safe access with get()
    match v.get(2) {
        Some(value) => println!("Third element (safe): {}", value),
        None => println!("No third element"),
    }

    // Out of bounds
    match v.get(100) {
        Some(value) => println!("Element 100: {}", value),
        None => println!("Index 100 is out of bounds"),
    }
}
```

### Step 3: Modifying vectors

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    println!("Initial: {:?}", v);

    // Add elements
    v.push(4);
    v.push(5);
    println!("After push: {:?}", v);

    // Remove last element
    let last = v.pop();
    println!("Popped: {:?}", last);
    println!("After pop: {:?}", v);

    // Remove at index
    let removed = v.remove(1);
    println!("Removed at index 1: {}", removed);
    println!("After remove: {:?}", v);

    // Insert at index
    v.insert(1, 10);
    println!("After insert at 1: {:?}", v);
}
```

### Step 4: Iterating over vectors

```rust
fn main() {
    let v = vec![10, 20, 30];

    // Immutable iteration
    println!("Immutable iteration:");
    for value in &v {
        println!("  {}", value);
    }

    // Mutable iteration
    let mut v2 = vec![1, 2, 3];
    println!("Before doubling: {:?}", v2);
    for value in &mut v2 {
        *value *= 2;
    }
    println!("After doubling: {:?}", v2);
}
```

---

## Exercise 2: Strings (10 min)

### Step 1: Creating strings

```rust
fn main() {
    // Empty string
    let mut s1 = String::new();

    // From string literal
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    // Building a string
    s1.push_str("Hello");
    s1.push(' ');
    s1.push_str("World");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}
```

### Step 2: String operations

```rust
fn main() {
    let mut s = String::from("Hello");

    // Append
    s.push_str(", World");
    s.push('!');
    println!("After append: {}", s);

    // Length
    println!("Length: {} bytes", s.len());
    println!("Character count: {}", s.chars().count());

    // Check content
    println!("Contains 'World': {}", s.contains("World"));
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));

    // Replace
    let s2 = s.replace("World", "Rust");
    println!("After replace: {}", s2);
}
```

### Step 3: String concatenation

```rust
fn main() {
    // Using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed
    println!("Using +: {}", s3);
    // println!("{}", s1);  // Error: s1 was moved

    // Using format! macro (doesn't move)
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{}, {}!", s4, s5);
    println!("Using format!: {}", s6);
    println!("s4 is still valid: {}", s4);
}
```

### Step 4: Iterating over strings

```rust
fn main() {
    let s = String::from("Hello");

    // Iterate over characters
    println!("Characters:");
    for c in s.chars() {
        println!("  {}", c);
    }

    // Iterate over bytes
    println!("Bytes:");
    for b in s.bytes() {
        println!("  {}", b);
    }
}
```

---

## Exercise 3: HashMap (10 min)

### Step 1: Creating and inserting

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores: {:?}", scores);
}
```

### Step 2: Accessing values

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Get returns Option<&V>
    let team = String::from("Blue");
    match scores.get(&team) {
        Some(score) => println!("{} team score: {}", team, score),
        None => println!("{} team not found", team),
    }

    // Non-existent key
    match scores.get("Green") {
        Some(score) => println!("Green team score: {}", score),
        None => println!("Green team not found"),
    }
}
```

### Step 3: Iterating over HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Green"), 30);

    // Iterate over key-value pairs
    println!("All scores:");
    for (team, score) in &scores {
        println!("  {}: {}", team, score);
    }
}
```

### Step 4: Updating values

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Insert
    scores.insert(String::from("Blue"), 10);
    println!("After insert: {:?}", scores);

    // Overwrite
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // Only insert if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("After or_insert: {:?}", scores);
}
```

### Step 5: Word count example

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world hello rust";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", word_count);
}
```

---

## Challenge Exercise (Bonus)

Create a simple contact book that stores names and phone numbers:

```rust
use std::collections::HashMap;

fn main() {
    let mut contacts: HashMap<String, String> = HashMap::new();

    // Add some contacts
    add_contact(&mut contacts, "Alice", "555-1234");
    add_contact(&mut contacts, "Bob", "555-5678");
    add_contact(&mut contacts, "Charlie", "555-9012");

    // Look up a contact
    lookup(&contacts, "Alice");
    lookup(&contacts, "David");

    // List all contacts
    list_contacts(&contacts);
}

fn add_contact(contacts: &mut HashMap<String, String>, name: &str, phone: &str) {
    // Your implementation
}

fn lookup(contacts: &HashMap<String, String>, name: &str) {
    // Your implementation
}

fn list_contacts(contacts: &HashMap<String, String>) {
    // Your implementation
}
```

---

## Verification Checklist

- [ ] Created vectors using Vec::new() and vec![]
- [ ] Accessed vector elements with indexing and get()
- [ ] Modified vectors with push, pop, remove, insert
- [ ] Iterated over vectors (immutable and mutable)
- [ ] Created strings and performed operations
- [ ] Concatenated strings with + and format!
- [ ] Created HashMap and inserted values
- [ ] Accessed HashMap values with get()
- [ ] Iterated over HashMap entries
- [ ] Used entry().or_insert() for conditional insert

---

## Summary

You have learned:
- `Vec<T>` is a growable array, use `vec![]` or `Vec::new()`
- Access vectors with `[]` (panics) or `.get()` (returns Option)
- `String` is heap-allocated, growable UTF-8 text
- Use `format!()` for concatenation without moving ownership
- `HashMap<K, V>` stores key-value pairs
- Use `.entry().or_insert()` for conditional insertion

**Next:** Lab 5 - Modules and Crates
