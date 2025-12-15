# Module 9: Collections and Iterators

**Duration:** 55 minutes
**Type:** Presentation

---

## Slide 1: Title

**Collections and Iterators**

- Dynamic Data Structures
- Module 9 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Work with Vec<T> (dynamic arrays)
- Use String for text manipulation
- Utilize HashMap<K, V> for key-value storage
- Understand the Iterator trait
- Chain iterator methods
- Use closures with iterators

---

## Slide 3: Collections Overview

```
┌─────────────────────────────────────────────┐
│           Standard Collections              │
├─────────────┬─────────────┬─────────────────┤
│   Vec<T>    │   String    │  HashMap<K,V>   │
├─────────────┼─────────────┼─────────────────┤
│ • Growable  │ • UTF-8     │ • Key-value     │
│ • Indexed   │ • Growable  │ • Fast lookup   │
│ • Contiguous│ • Owned     │ • Unordered     │
└─────────────┴─────────────┴─────────────────┘
```

**All are heap-allocated and can grow/shrink**

---

## Slide 4: Creating Vectors

```rust
fn main() {
    // Empty vector with type annotation
    let v1: Vec<i32> = Vec::new();

    // Using vec! macro
    let v2 = vec![1, 2, 3];

    // With capacity
    let v3: Vec<i32> = Vec::with_capacity(10);
}
```

---

## Slide 5: Modifying Vectors

```rust
fn main() {
    let mut v = Vec::new();

    // Add elements
    v.push(1);
    v.push(2);
    v.push(3);

    // Remove last element
    let last = v.pop();  // Returns Option<T>

    // Remove at index
    let removed = v.remove(0);

    println!("{:?}", v);  // [2]
}
```

---

## Slide 6: Accessing Vector Elements

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Direct indexing (panics if out of bounds)
    let third = v[2];

    // Safe access with get (returns Option)
    match v.get(2) {
        Some(value) => println!("Third: {}", value),
        None => println!("No third element"),
    }

    // Panics!
    // let bad = v[100];

    // Returns None
    let safe = v.get(100);  // None
}
```

---

## Slide 7: Iterating Over Vectors

```rust
fn main() {
    let v = vec![10, 20, 30];

    // Immutable iteration
    for value in &v {
        println!("{}", value);
    }

    // Mutable iteration
    let mut v2 = vec![1, 2, 3];
    for value in &mut v2 {
        *value *= 2;
    }
    println!("{:?}", v2);  // [2, 4, 6]

    // Consuming iteration (takes ownership)
    for value in v {
        println!("{}", value);
    }
    // v is no longer valid
}
```

---

## Slide 8: Vector Methods

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("Length: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    println!("Contains 3: {}", v.contains(&3));

    let first = v.first();  // Option<&T>
    let last = v.last();    // Option<&T>

    // Slice
    let slice = &v[1..3];   // &[2, 3]
}
```

---

## Slide 9: String Basics

**String is a growable, UTF-8 encoded string:**

```rust
fn main() {
    // Create empty String
    let mut s = String::new();

    // From string literal
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    // Append
    s.push_str("hello");
    s.push(' ');           // Single char
    s.push_str("world");

    println!("{}", s);  // "hello world"
}
```

---

## Slide 10: String Concatenation

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // Using + (takes ownership of s1)
    let s3 = s1 + &s2;
    // s1 is invalid now, s2 is still valid

    // Using format! (doesn't take ownership)
    let s4 = String::from("Hello");
    let s5 = String::from("world");
    let s6 = format!("{}, {}!", s4, s5);
    // s4 and s5 are still valid
}
```

---

## Slide 11: String Indexing

**Cannot index String by integer:**

```rust
fn main() {
    let s = String::from("hello");

    // let c = s[0];  // ERROR!

    // Use chars() for characters
    for c in s.chars() {
        println!("{}", c);
    }

    // Use bytes() for raw bytes
    for b in s.bytes() {
        println!("{}", b);
    }

    // Slice (be careful with UTF-8!)
    let slice = &s[0..2];  // "he"
}
```

---

## Slide 12: String Methods

```rust
fn main() {
    let s = String::from("  Hello, World!  ");

    println!("Length: {}", s.len());           // Bytes, not chars!
    println!("Chars: {}", s.chars().count()); // Character count

    let trimmed = s.trim();
    let lower = s.to_lowercase();
    let upper = s.to_uppercase();

    let replaced = s.replace("World", "Rust");

    println!("Contains 'Hello': {}", s.contains("Hello"));
    println!("Starts with '  ': {}", s.starts_with("  "));
}
```

---

## Slide 13: HashMap Basics

```rust
use std::collections::HashMap;

fn main() {
    // Create empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Access values
    let team = String::from("Blue");
    let score = scores.get(&team);  // Returns Option<&V>

    println!("{:?}", score);  // Some(10)
}
```

---

## Slide 14: HashMap Iteration

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Iterate over keys only
    for key in scores.keys() {
        println!("Team: {}", key);
    }

    // Iterate over values only
    for value in scores.values() {
        println!("Score: {}", value);
    }
}
```

---

## Slide 15: HashMap Entry API

**Conditional insertion:**

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Only insert if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);  // Not inserted
    scores.entry(String::from("Yellow")).or_insert(25); // Inserted

    // Modify based on existing value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
```

---

## Slide 16: Introduction to Iterators

**Iterators process sequences lazily:**

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Create an iterator
    let iter = v.iter();

    // Consume with for loop
    for value in iter {
        println!("{}", value);
    }
}
```

---

## Slide 17: Iterator Methods

**Three types of iteration:**

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter() - borrows, yields &T
    for val in v.iter() {
        println!("{}", val);
    }

    // iter_mut() - mutable borrow, yields &mut T
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;
    }

    // into_iter() - takes ownership, yields T
    for val in v.into_iter() {
        println!("{}", val);
    }
    // v is no longer valid
}
```

---

## Slide 18: Iterator Adapters

**Transform iterators (lazy):**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // map - transform each element
    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();

    // filter - keep elements matching predicate
    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
    println!("{:?}", evens);    // [2, 4]
}
```

---

## Slide 19: Closures

**Anonymous functions that capture environment:**

```rust
fn main() {
    // Basic closure
    let add_one = |x| x + 1;
    println!("{}", add_one(5));  // 6

    // With type annotations
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("{}", multiply(3, 4));  // 12

    // Capturing variables
    let factor = 10;
    let scale = |x| x * factor;
    println!("{}", scale(5));  // 50
}
```

---

## Slide 20: Consuming Iterators

**Methods that consume the iterator:**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // sum
    let total: i32 = v.iter().sum();

    // count
    let count = v.iter().count();

    // collect
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

    // any / all
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);

    // find
    let first_even = v.iter().find(|x| *x % 2 == 0);
}
```

---

## Slide 21: Chaining Iterator Methods

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: i32 = numbers
        .iter()
        .filter(|x| *x % 2 == 0)    // Keep evens: [2,4,6,8,10]
        .map(|x| x * x)              // Square: [4,16,36,64,100]
        .take(3)                     // First 3: [4,16,36]
        .sum();                      // Sum: 56

    println!("{}", result);  // 56
}
```

---

## Slide 22: More Iterator Methods

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // enumerate - add index
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }

    // zip - combine two iterators
    let a = vec![1, 2, 3];
    let b = vec!["one", "two", "three"];
    for (num, name) in a.iter().zip(b.iter()) {
        println!("{} = {}", num, name);
    }

    // skip and take
    let middle: Vec<&i32> = v.iter().skip(1).take(3).collect();
}
```

---

## Slide 23: fold / reduce

**Accumulate values:**

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // fold - with initial value
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);  // 15

    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);  // 120

    // Building a string
    let words = vec!["hello", "world"];
    let sentence = words.iter()
        .fold(String::new(), |acc, word| acc + word + " ");
}
```

---

## Slide 24: Performance Note

**Iterators are zero-cost abstractions:**

```rust
// These have the same performance!

// Iterator version
let sum: i32 = (0..1000).filter(|x| x % 2 == 0).sum();

// Loop version
let mut sum = 0;
for i in 0..1000 {
    if i % 2 == 0 {
        sum += i;
    }
}
```

**Rust optimizes iterator chains to be as fast as hand-written loops**

---

## Slide 25: Key Takeaways

1. **Vec<T>** is a growable array, use `vec![]` or `Vec::new()`
2. **String** is UTF-8, use `String::from()` or `.to_string()`
3. **HashMap** stores key-value pairs, use `entry()` for conditional insert
4. **Iterators** are lazy - nothing happens until consumed
5. **Closures** are anonymous functions with `|args| body`
6. **Iterator adapters** (map, filter) transform
7. **Consumers** (collect, sum, fold) produce values

---

## Slide 26: Lab Preview

**Lab 9: Collections and Iterators** (30 minutes)

You will:
- Create and manipulate vectors
- Work with String operations
- Use HashMap for data storage
- Chain iterator methods
- Write closures for transformations
- Process data with map, filter, fold

---

## Questions?

**Next Module:** Modules and Crates
