# Module 5: Object Orientation

**Duration:** 60 minutes (split into 2 parts)

---

# Part 1: Structs and Methods (30 min)

---

## Slide 1: Title

**Object Orientation - Part 1**

- Structs and Methods
- Module 5a of 6

---

## Slide 2: Defining Structures

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
}
```

---

## Slide 3: Accessing Fields

```rust
fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    println!("Email: {}", user1.email);
    println!("Username: {}", user1.username);
}
```

---

## Slide 4: Mutable Structs

```rust
fn main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new@example.com");
}
```

**The entire struct must be mutable**

---

## Slide 5: Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,          // shorthand for email: email
        username,       // shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}
```

---

## Slide 6: Struct Update Syntax

```rust
fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // Use remaining fields from user1
    };
}
```

---

## Slide 7: Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("R: {}", black.0);
    println!("X: {}", origin.0);

    // black and origin are different types!
}
```

---

## Slide 8: Unit Structs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

**Useful for:**
- Marker types
- Implementing traits with no data

---

## Slide 9: Printing Structs with Debug

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("{:?}", rect);    // One line
    println!("{:#?}", rect);   // Pretty-printed
}
```

---

## Slide 10: Implementing Functionality

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
}
```

---

## Slide 11: Methods - self Parameter

```rust
impl Rectangle {
    // Borrow immutably - most common
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Borrow mutably - for modification
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // Take ownership - rare
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}
```

---

## Slide 12: Associated Functions

```rust
impl Rectangle {
    // No self parameter - called with ::
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(10);
}
```

---

## Slide 13: Multiple impl Blocks

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

---

# Part 2: Traits and Generics (30 min)

---

## Slide 14: Title

**Object Orientation - Part 2**

- Traits and Generics
- Module 5b of 6

---

## Slide 15: What are Traits?

**Shared behavior definition:**

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

**Similar to:**
- Interfaces in Java
- Protocols in Swift
- Abstract classes

---

## Slide 16: Implementing Traits

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
```

---

## Slide 17: Using Traits

```rust
fn main() {
    let article = Article {
        title: String::from("Breaking News"),
        author: String::from("John"),
    };

    println!("{}", article.summarize());
}
```

---

## Slide 18: Default Implementations

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // Uses default summarize()
}
```

---

## Slide 19: Generic Functions

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
    let numbers = vec![34, 50, 25, 100];
    println!("Largest: {}", largest(&numbers));
}
```

---

## Slide 20: Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}
```

---

## Slide 21: Multiple Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };
}
```

---

## Slide 22: Trait Bounds

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

## Slide 23: Multiple Trait Bounds

```rust
use std::fmt::Display;

fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

// With where clause
fn complex<T, U>(t: &T, u: &U)
where
    T: Summary + Clone,
    U: Clone + Display,
{
    // ...
}
```

---

## Slide 24: Common Derive Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();      // Clone

    println!("{:?}", p1);      // Debug

    if p1 == p2 {              // PartialEq
        println!("Equal!");
    }
}
```

---

## Slide 25: Design Patterns - Builder

```rust
struct RequestBuilder {
    url: String,
    method: String,
}

impl RequestBuilder {
    fn new() -> Self {
        RequestBuilder {
            url: String::new(),
            method: String::from("GET"),
        }
    }

    fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn build(self) -> Request {
        Request { url: self.url, method: self.method }
    }
}
```

---

## Slide 26: Design Patterns - Newtype

```rust
// Wrap a type to give it new meaning
struct Meters(f64);
struct Feet(f64);

impl Meters {
    fn to_feet(&self) -> Feet {
        Feet(self.0 * 3.28084)
    }
}

fn main() {
    let distance = Meters(100.0);
    let in_feet = distance.to_feet();
}
```

---

## Slide 27: Key Takeaways

1. **Structs** group related data
2. **impl blocks** add methods and associated functions
3. **Traits** define shared behavior
4. **Generics** allow type-flexible code
5. **Trait bounds** constrain generic types
6. **#[derive]** auto-implements common traits
7. **Patterns** like Builder and Newtype are common in Rust

---

## Slide 28: Lab Preview

**Lab 5a: Structs and Methods** (35 min)
- Define structs, implement methods

**Lab 5b: Traits and Generics** (35 min)
- Define traits, use generics and bounds

---

## Questions?
