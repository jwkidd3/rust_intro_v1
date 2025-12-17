# Module 6: Functional Programming

**Duration:** 55 minutes (split into 2 parts)

---

# Part 1: Closures (25 min)

---

## Slide 1: Title

**Functional Programming - Part 1**

- Closures
- Module 6a of 6

---

## Slide 2: What is Functional Programming?

**Core concepts:**
- Functions as first-class citizens
- Immutability preferred
- Avoid side effects
- Declarative over imperative

**In Rust:**
- Closures (anonymous functions)
- Iterators
- Higher-order functions (map, filter, fold)

---

## Slide 3: Anonymous Functions

```rust
fn main() {
    // Named function
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // Anonymous function (closure)
    let add_one = |x: i32| -> i32 { x + 1 };

    // Shorter syntax
    let add_one = |x| x + 1;

    println!("{}", add_one(5));  // 6
}
```

---

## Slide 4: Closure Syntax

```rust
fn main() {
    // Full syntax
    let add = |x: i32, y: i32| -> i32 { x + y };

    // Type inference
    let add = |x, y| x + y;

    // Single expression (no braces needed)
    let double = |x| x * 2;

    // No parameters
    let greet = || println!("Hello!");

    greet();
    println!("{}", add(2, 3));
    println!("{}", double(5));
}
```

---

## Slide 5: Closures Capture Environment

```rust
fn main() {
    let factor = 10;

    // Closure captures `factor` from environment
    let scale = |x| x * factor;

    println!("{}", scale(5));  // 50
}
```

**Unlike regular functions, closures can use variables from their scope**

---

## Slide 6: Capture Modes

```rust
fn main() {
    let s = String::from("hello");

    // Borrow immutably
    let print_s = || println!("{}", s);
    print_s();
    println!("{}", s);  // s still valid

    // Borrow mutably
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    println!("{}", count);  // 1
}
```

---

## Slide 7: Move Closures

```rust
fn main() {
    let s = String::from("hello");

    // Force ownership transfer with `move`
    let consume = move || println!("{}", s);

    consume();
    // println!("{}", s);  // ERROR: s was moved
}
```

**Use `move` when:**
- Passing closures to threads
- Closure needs to outlive current scope

---

## Slide 8: Closures as Parameters

```rust
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let double = |x| x * 2;
    let result = apply(double, 5);
    println!("{}", result);  // 10
}
```

---

## Slide 9: Closure Traits

| Trait | Description |
|-------|-------------|
| `Fn` | Borrows captured values immutably |
| `FnMut` | Borrows captured values mutably |
| `FnOnce` | Takes ownership of captured values |

```rust
fn call_once<F: FnOnce()>(f: F) {
    f();
}

fn call_many<F: Fn()>(f: F) {
    f();
    f();
}
```

---

## Slide 10: Storing Closures

```rust
struct Calculator {
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

fn main() {
    let calc = Calculator {
        operation: Box::new(|a, b| a + b),
    };

    let result = (calc.operation)(5, 3);
    println!("{}", result);  // 8
}
```

---

# Part 2: Iterators (30 min)

---

## Slide 11: Title

**Functional Programming - Part 2**

- Iterators
- Module 6b of 6

---

## Slide 12: What are Iterators?

**A way to process sequences of elements:**

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

## Slide 13: Iterator Methods

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
}
```

---

## Slide 14: Iterator Adapters

**Transform iterators (lazy):**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // map - transform each element
    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
}
```

---

## Slide 15: filter

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    println!("{:?}", evens);  // [2, 4, 6]
}
```

---

## Slide 16: Chaining Operations

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: i32 = numbers
        .iter()
        .filter(|x| *x % 2 == 0)    // Keep evens
        .map(|x| x * x)              // Square
        .take(3)                     // First 3
        .sum();                      // Sum

    println!("{}", result);  // 4 + 16 + 36 = 56
}
```

---

## Slide 17: Common Iterator Methods

| Method | Description |
|--------|-------------|
| `map` | Transform each element |
| `filter` | Keep elements matching predicate |
| `take(n)` | Take first n elements |
| `skip(n)` | Skip first n elements |
| `enumerate` | Add index to elements |
| `zip` | Combine two iterators |

---

## Slide 18: Consuming Iterators

| Method | Description |
|--------|-------------|
| `collect` | Gather into collection |
| `sum` | Sum all elements |
| `count` | Count elements |
| `any` | True if any match predicate |
| `all` | True if all match predicate |
| `find` | Find first matching element |

---

## Slide 19: fold - The Powerful Accumulator

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Sum
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);  // 15

    // Product
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);  // 120

    // Build string
    let words = vec!["hello", "world"];
    let sentence = words.iter()
        .fold(String::new(), |acc, w| acc + w + " ");
    println!("{}", sentence);  // "hello world "
}
```

---

## Slide 20: enumerate

```rust
fn main() {
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
```

**Output:**
```
0: a
1: b
2: c
```

---

## Slide 21: zip

```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
}
```

---

## Slide 22: find and any/all

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Find first even
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("{:?}", first_even);  // Some(2)

    // Check conditions
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    let all_positive = numbers.iter().all(|x| *x > 0);

    println!("Has even: {}", has_even);      // true
    println!("All positive: {}", all_positive);  // true
}
```

---

## Slide 23: Iterators are Lazy

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Nothing happens yet!
    let iter = v.iter().map(|x| {
        println!("Processing {}", x);
        x * 2
    });

    // Now it runs
    let result: Vec<_> = iter.collect();
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

---

## Slide 25: Patterns and Techniques

```rust
fn main() {
    // Process and collect
    let doubled: Vec<_> = (1..=5).map(|x| x * 2).collect();

    // Find max/min
    let max = vec![3, 1, 4, 1, 5].iter().max();

    // Flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flat: Vec<_> = nested.iter().flatten().collect();

    // Partition by condition
    let (evens, odds): (Vec<_>, Vec<_>) = (1..=10)
        .partition(|x| x % 2 == 0);
}
```

---

## Slide 26: Key Takeaways

1. **Closures** are anonymous functions that capture environment
2. **move** transfers ownership to closure
3. **Iterators** process sequences lazily
4. **Adapters** (map, filter) transform iterators
5. **Consumers** (collect, sum, fold) produce values
6. **Chain operations** for clean, declarative code
7. **Zero-cost** - as fast as hand-written loops

---

## Slide 27: Lab Preview

**Lab 9: Closures** (30 min)
- Write closures with different capture modes
- Pass closures to functions

**Lab 10: Iterators** (35 min)
- Use map, filter, fold
- Chain iterator operations

---

## Slide 28: Course Summary

**What you've learned:**
- Getting started with Rust and Cargo
- Types, variables, functions, control flow
- Collections (Vec, String, HashMap)
- Organizing code with modules and crates
- Error handling with Result and panic!
- Structs, methods, traits, generics
- Closures and iterators

**You're now ready to write Rust programs!**

---

## Questions?

**Resources:**
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings: https://github.com/rust-lang/rustlings
