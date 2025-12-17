# Lab 8: Traits and Generics

**Duration:** 35 minutes

## Objectives

- Define custom traits
- Implement traits for structs
- Write generic functions
- Use trait bounds
- Implement common patterns

## Prerequisites

- Completed Lab 7

---

## Setup

Create a new project:
```bash
cargo new lab08_traits
cd lab08_traits
code .
```

---

## Exercise 1: Defining Traits (10 min)

### Step 1: Basic trait definition

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust is Great"),
        author: String::from("Jane Doe"),
        content: String::from("Rust provides memory safety..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello, Rustaceans!"),
    };

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());
}
```

### Step 2: Default implementations

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // Override default
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize()
}

fn main() {
    let article = Article {
        title: String::from("Rust Tips"),
        author: String::from("Jane"),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello!"),
    };

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());
}
```

---

## Exercise 2: Traits as Parameters (5 min)

### Step 1: Using traits in function parameters

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

// Using impl Trait syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Equivalent using trait bound syntax
fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Big News"),
        author: String::from("Reporter"),
    };

    notify(&article);
    notify_verbose(&article);
}
```

---

## Exercise 3: Generics (10 min)

### Step 1: Generic functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Largest number: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Largest char: {}", result);
}
```

### Step 2: Generic structs

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.5);

    println!("Integer point: {:?}", int_point);
    println!("Float point: {:?}", float_point);
    println!("int_point.x = {}", int_point.x());
}
```

### Step 3: Multiple type parameters

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point::new(5, 10.4);
    let p2 = Point::new("Hello", 'c');

    let p3 = p1.mixup(p2);

    println!("p3: {:?}", p3);  // Point { x: 5, y: 'c' }
}
```

---

## Exercise 4: Trait Bounds (5 min)

### Step 1: Multiple trait bounds

```rust
use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

fn notify<T: Summary + Display>(item: &T) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

// Using where clause for clarity
fn notify_where<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

#[derive(Debug)]
struct Article {
    title: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.title.clone()
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Article: {}", self.title)
    }
}

fn main() {
    let article = Article {
        title: String::from("Important News"),
    };

    notify(&article);
}
```

---

## Exercise 5: Common Patterns (5 min)

### Step 1: Builder pattern

```rust
#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    max_connections: u32,
}

struct ServerBuilder {
    host: String,
    port: u16,
    max_connections: u32,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
        }
    }
}

fn main() {
    let server = ServerBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(1000)
        .build();

    println!("Server config: {:?}", server);

    // With defaults
    let default_server = ServerBuilder::new().build();
    println!("Default server: {:?}", default_server);
}
```

### Step 2: Newtype pattern

```rust
struct Meters(f64);
struct Kilometers(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

impl Kilometers {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1000.0)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

fn main() {
    let distance = Meters::new(5000.0);
    let km = distance.to_kilometers();

    println!("{} meters = {} kilometers", distance.value(), km.value());
}
```

---

## Challenge Exercise (Bonus)

Create a trait `Drawable` with:
- Method `draw(&self)` that prints a shape description
- Method `area(&self) -> f64`

Implement it for `Circle` and `Rectangle` structs.

Create a function `print_shapes(shapes: &[&dyn Drawable])` that prints all shapes.

```rust
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// Your implementations here

fn print_shapes(shapes: &[&dyn Drawable]) {
    for shape in shapes {
        shape.draw();
        println!("Area: {:.2}", shape.area());
        println!();
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 4.0, height: 6.0 };

    let shapes: Vec<&dyn Drawable> = vec![&circle, &rect];
    print_shapes(&shapes);
}
```

---

## Verification Checklist

- [ ] Defined a custom trait
- [ ] Implemented trait for multiple types
- [ ] Used default trait implementations
- [ ] Used traits as function parameters
- [ ] Created generic functions
- [ ] Created generic structs
- [ ] Used multiple type parameters
- [ ] Applied trait bounds
- [ ] Used where clause for bounds
- [ ] Implemented Builder pattern
- [ ] Implemented Newtype pattern

---

## Summary

You have learned:
- Traits define shared behavior
- `impl TraitName for Type` implements traits
- Default implementations can be overridden
- Generic functions work with multiple types
- Trait bounds constrain what types can be used
- `where` clause improves readability for complex bounds
- Builder pattern creates flexible constructors
- Newtype pattern provides type safety

**Next:** Lab 9 - Closures
