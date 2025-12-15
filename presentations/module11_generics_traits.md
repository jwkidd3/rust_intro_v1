# Module 11: Generics and Traits

**Duration:** 55 minutes
**Type:** Presentation

---

## Slide 1: Title

**Generics and Traits**

- Abstraction and Polymorphism in Rust
- Module 11 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Define and use generic types
- Understand trait definitions
- Implement traits for types
- Use trait bounds for constraints
- Apply common standard library traits
- Understand blanket implementations

---

## Slide 3: The Problem Without Generics

**Duplicated code:**

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest { largest = item; }
    }
    largest
}

fn largest_f64(list: &[f64]) -> f64 {
    let mut largest = list[0];
    for &item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

**Same logic, different types!**

---

## Slide 4: Generic Functions

**One function works for many types:**

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    println!("Largest number: {}", largest(&numbers));
    println!("Largest char: {}", largest(&chars));
}
```

---

## Slide 5: Generic Syntax

```rust
// Generic function
fn function_name<T>(param: T) -> T { }

// Multiple type parameters
fn function<T, U>(t: T, u: U) { }

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic enum (you've seen this!)
enum Option<T> {
    Some(T),
    None,
}
```

---

## Slide 6: Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // Error: mismatched types
    // let mixed = Point { x: 5, y: 4.0 };
}
```

---

## Slide 7: Multiple Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };  // Now works!
}
```

---

## Slide 8: Methods on Generic Types

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("x = {}", p.x());
}
```

---

## Slide 9: Methods for Specific Types

```rust
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p.distance_from_origin());  // 5.0

    // let p2 = Point { x: 3, y: 4 };
    // p2.distance_from_origin();  // ERROR: not available for i32
}
```

---

## Slide 10: What are Traits?

**Shared behavior definition:**

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

**Similar to:**
- Interfaces in Java/C#
- Protocols in Swift
- Abstract base classes in Python

**But more powerful!**

---

## Slide 11: Implementing Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        author: String::from("John"),
    };
    println!("{}", article.summarize());
}
```

---

## Slide 12: Multiple Types, Same Trait

```rust
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello, Rustaceans!"),
    };
    println!("{}", tweet.summarize());
}
```

---

## Slide 13: Default Implementations

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize()
}
```

---

## Slide 14: Trait Bounds

**Constrain generic types:**

```rust
// Only types with Summary can be passed
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Shorthand syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

---

## Slide 15: Multiple Trait Bounds

```rust
use std::fmt::Display;

// With + syntax
fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item);
    println!("{}", item.summarize());
}

// With where clause (cleaner for multiple params)
fn complex<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + Display,
{
    // ...
}
```

---

## Slide 16: Returning Trait Types

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Hello!"),
    }
}
```

**Limitation:** Can only return one concrete type

```rust
// ERROR: Cannot return different types
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { /* ... */ }
    } else {
        Tweet { /* ... */ }  // Different type!
    }
}
```

---

## Slide 17: Common Standard Library Traits

```
┌─────────────────────────────────────────────┐
│        Common Standard Traits               │
├───────────┬─────────────────────────────────┤
│ Debug     │ {:?} formatting                 │
│ Clone     │ Deep copy                       │
│ Copy      │ Bitwise copy (stack types)      │
│ PartialEq │ == and != comparison            │
│ Eq        │ Reflexive equality              │
│ PartialOrd│ <, >, <=, >= comparison        │
│ Ord       │ Total ordering                  │
│ Default   │ Default value                   │
│ Display   │ {} formatting                   │
└───────────┴─────────────────────────────────┘
```

---

## Slide 18: Deriving Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    println!("{:?}", p1);      // Debug
    println!("{}", p1 == p2);  // PartialEq
}
```

**Derive generates standard implementations**

---

## Slide 19: The Display Trait

**Must implement manually:**

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{}", p);  // (3, 4)
}
```

---

## Slide 20: The Default Trait

```rust
#[derive(Default, Debug)]
struct Config {
    debug: bool,
    timeout: u32,
    name: String,
}

fn main() {
    let config = Config::default();
    println!("{:?}", config);
    // Config { debug: false, timeout: 0, name: "" }

    let custom = Config {
        debug: true,
        ..Default::default()
    };
}
```

---

## Slide 21: Trait Objects

**Dynamic dispatch with dyn:**

```rust
fn print_summaries(items: &[&dyn Summary]) {
    for item in items {
        println!("{}", item.summarize());
    }
}

fn main() {
    let article = NewsArticle { /* ... */ };
    let tweet = Tweet { /* ... */ };

    let items: Vec<&dyn Summary> = vec![&article, &tweet];
    print_summaries(&items);
}
```

---

## Slide 22: Blanket Implementations

**Implement traits for all types matching bounds:**

```rust
// Standard library does this:
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}

// Now any type with Display has to_string()
let s = 3.to_string();  // "3"
```

---

## Slide 23: Generic Performance

**Zero-cost abstractions:**

```rust
// At compile time, Rust generates:
fn largest_i32(list: &[i32]) -> i32 { ... }
fn largest_f64(list: &[f64]) -> f64 { ... }

// Called "monomorphization"
// No runtime overhead!
```

**Each concrete type gets its own optimized code**

---

## Slide 24: Operator Overloading with Traits

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Uses Add trait
    println!("{:?}", p3);  // Point { x: 4, y: 6 }
}
```

---

## Slide 25: Key Takeaways

1. **Generics** reduce code duplication
2. **Traits** define shared behavior
3. **Trait bounds** constrain generic types
4. **#[derive]** auto-implements common traits
5. **impl Trait** for return types, **dyn Trait** for dynamic dispatch
6. **Blanket implementations** provide traits to many types
7. **Monomorphization** means zero runtime cost

---

## Slide 26: Lab Preview

**Lab 11: Generics and Traits** (30 minutes)

You will:
- Write generic functions
- Create generic structs
- Define and implement custom traits
- Use trait bounds
- Derive common traits
- Implement Display for custom types

---

## Questions?

**Next Module:** Lifetimes and Testing
