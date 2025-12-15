# Lab 9: Collections and Iterators

## Duration: 25 minutes

## Objectives
- Work with Vec<T>, String, and HashMap
- Use iterators for processing collections
- Apply closures with iterator methods
- Chain iterator operations

## Prerequisites
- Completed Lab 8
- Understanding of ownership and references

## Part 1: Vectors (8 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab09_collections
cd lab09_collections
```

### Exercise 2: Creating and Modifying Vectors

Edit `src/main.rs`:

```rust
fn main() {
    // Create vectors
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // Mutable vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);

    // Remove last element
    let last = v.pop();
    println!("Popped: {:?}, Vector: {:?}", last, v);

    // Access elements
    let third = &v2[2];      // Panics if out of bounds
    println!("Third element: {}", third);

    let third = v2.get(2);   // Returns Option<&T>
    match third {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element"),
    }

    // Length and capacity
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());
}
```

### Exercise 3: Iterating Over Vectors

```rust
fn main() {
    let v = vec![100, 32, 57];

    // Immutable iteration
    println!("Immutable iteration:");
    for i in &v {
        println!("  {}", i);
    }

    // Mutable iteration
    let mut v = vec![100, 32, 57];
    println!("Before: {:?}", v);
    for i in &mut v {
        *i += 50;  // Dereference to modify
    }
    println!("After adding 50: {:?}", v);

    // Taking ownership
    let v = vec![String::from("a"), String::from("b")];
    for s in v {
        println!("Owned: {}", s);
    }
    // v is no longer accessible here
}
```

### Exercise 4: Vectors with Enums

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Store different types using enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
```

## Part 2: Strings (7 minutes)

### Exercise 5: Creating and Growing Strings

```rust
fn main() {
    // Create strings
    let mut s = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    // Append to string
    s.push_str("Hello");
    s.push(' ');
    s.push_str("World");
    println!("{}", s);

    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed
    println!("{}", s3);
    // println!("{}", s1);  // ERROR: s1 was moved

    // format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("s1 still valid: {}", s1);  // s1 is still valid
}
```

### Exercise 6: String Iteration

```rust
fn main() {
    let hello = String::from("Hello");

    // Iterate over characters
    println!("Characters:");
    for c in hello.chars() {
        println!("  {}", c);
    }

    // Iterate over bytes
    println!("Bytes:");
    for b in hello.bytes() {
        println!("  {}", b);
    }

    // String length vs character count
    let emoji = String::from("Hello 🦀");
    println!("String: {}", emoji);
    println!("Byte length: {}", emoji.len());
    println!("Char count: {}", emoji.chars().count());
}
```

## Part 3: HashMaps (5 minutes)

### Exercise 7: Creating and Using HashMaps

```rust
use std::collections::HashMap;

fn main() {
    // Create HashMap
    let mut scores = HashMap::new();

    // Insert values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue team score: {:?}", score);

    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Update: overwrite
    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);

    // Update: only insert if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("After or_insert: {:?}", scores);
}
```

### Exercise 8: Word Counter

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

## Part 4: Iterators and Closures (5 minutes)

### Exercise 9: Closures

```rust
fn main() {
    // Basic closure
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // Closure with type annotations
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("add(2, 3) = {}", add(2, 3));

    // Closure capturing environment
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("Is 4 equal to x? {}", equal_to_x(4));

    // Closure with move
    let s = String::from("hello");
    let print_s = move || println!("{}", s);
    print_s();
    // println!("{}", s);  // ERROR: s was moved into closure
}
```

### Exercise 10: Iterator Methods

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter: keep elements matching condition
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // fold: reduce to single value
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    // Chaining operations
    let result: i32 = numbers
        .iter()
        .filter(|x| *x % 2 == 0)  // Keep evens
        .map(|x| x * x)            // Square them
        .sum();                     // Sum them
    println!("Sum of squared evens: {}", result);

    // find: get first match
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    // any and all
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
}
```

### Exercise 11: Practical Iterator Example

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
        Person { name: String::from("Diana"), age: 28 },
    ];

    // Find people over 27
    let over_27: Vec<&Person> = people
        .iter()
        .filter(|p| p.age > 27)
        .collect();
    println!("Over 27: {:?}", over_27);

    // Get names of people over 27
    let names: Vec<&String> = people
        .iter()
        .filter(|p| p.age > 27)
        .map(|p| &p.name)
        .collect();
    println!("Names over 27: {:?}", names);

    // Calculate average age
    let total_age: u32 = people.iter().map(|p| p.age).sum();
    let avg_age = total_age as f64 / people.len() as f64;
    println!("Average age: {:.1}", avg_age);

    // Find oldest person
    let oldest = people.iter().max_by_key(|p| p.age);
    println!("Oldest: {:?}", oldest);
}
```

## Verification Steps

### Checklist
- [ ] Created and modified vectors
- [ ] Iterated over vectors (immutable and mutable)
- [ ] Created and manipulated strings
- [ ] Used format! macro for concatenation
- [ ] Created and used HashMaps
- [ ] Wrote closures with different capture modes
- [ ] Used map, filter, fold, and other iterator methods
- [ ] Chained iterator operations

## Lab Questions

1. What is the difference between `&v` and `v.iter()` in a for loop?
2. Why can't you index into a String with integers?
3. What does `entry().or_insert()` do on a HashMap?
4. What is the difference between `iter()`, `iter_mut()`, and `into_iter()`?

## Answers

1. They're effectively the same - `&v` is syntactic sugar that calls `v.iter()` for immutable iteration.

2. Rust strings are UTF-8 encoded. Characters can be 1-4 bytes, so indexing by byte position could land in the middle of a character. Use `.chars()` or `.bytes()` instead.

3. `entry()` gets an Entry for the key. `or_insert()` returns a mutable reference to the value if key exists, or inserts the given value and returns a reference to it.

4. `iter()` yields `&T` (borrows). `iter_mut()` yields `&mut T` (mutable borrows). `into_iter()` yields `T` (takes ownership).

## Common Issues

**Issue: "cannot borrow as mutable"**
```
Solution: Use iter_mut() instead of iter(), or declare the collection as mut.
```

**Issue: "value moved in previous iteration"**
```
Solution: Use &collection in the for loop, or call .iter() explicitly.
```

## Next Steps

In Lab 10, you will:
- Organize code with modules
- Use external crates
- Create library crates

## Completion

You have completed Lab 9 when you can:
- Use Vec, String, and HashMap effectively
- Write closures and pass them to iterator methods
- Chain iterator operations for data processing
- Choose the right iteration method for your needs
