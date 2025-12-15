# Lab 11: Generics and Traits

## Duration: 25 minutes

## Objectives
- Write generic functions and structs
- Define and implement traits
- Use trait bounds to constrain generics
- Implement standard library traits

## Prerequisites
- Completed Lab 10
- Understanding of structs and modules

## Part 1: Generics (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab11_generics
cd lab11_generics
```

### Exercise 2: Generic Functions

Edit `src/main.rs`:

```rust
// Without generics - duplicated code
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// With generics - single function
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

### Exercise 3: Generic Structs

```rust
// Generic struct with one type parameter
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct with multiple type parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Implement only for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed = MixedPoint { x: 5, y: 4.0 };

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point distance = {}", float_point.distance_from_origin());
    println!("mixed: ({}, {})", mixed.x, mixed.y);

    // This won't compile - distance_from_origin only for f64:
    // integer_point.distance_from_origin();
}
```

### Exercise 4: Generic Enums

```rust
// Standard library examples:
// enum Option<T> { Some(T), None }
// enum Result<T, E> { Ok(T), Err(E) }

// Our custom generic enum
enum Container<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

impl<T: std::fmt::Debug> Container<T> {
    fn describe(&self) {
        match self {
            Container::Empty => println!("Container is empty"),
            Container::Single(item) => println!("Contains one: {:?}", item),
            Container::Pair(a, b) => println!("Contains pair: {:?}, {:?}", a, b),
        }
    }
}

fn main() {
    let empty: Container<i32> = Container::Empty;
    let single = Container::Single(42);
    let pair = Container::Pair("hello", "world");

    empty.describe();
    single.describe();
    pair.describe();
}
```

## Part 2: Traits (10 minutes)

### Exercise 5: Defining Traits

```rust
// Define a trait
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn read_more(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    retweet_count: u32,
}

// Implement trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Implement trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

    // Override default implementation
    fn read_more(&self) -> String {
        format!("(Retweeted {} times)", self.retweet_count)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2024 Released"),
        location: String::from("San Francisco"),
        author: String::from("Jane Doe"),
        content: String::from("The Rust team announced..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Excited to announce Rust 2024!"),
        retweet_count: 1000,
    };

    println!("Article: {}", article.summarize());
    println!("  {}", article.read_more());

    println!("Tweet: {}", tweet.summarize());
    println!("  {}", tweet.read_more());
}
```

### Exercise 6: Traits as Parameters

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Trait bound syntax
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait syntax (shorthand)
fn notify_short(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify_debug<T: Summary + std::fmt::Debug>(item: &T) {
    println!("Notify: {:?}", item);
    println!("Summary: {}", item.summarize());
}

// where clause for cleaner syntax
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + std::fmt::Debug,
{
    format!("{} - {:?}", t.summarize(), u)
}

fn main() {
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Hello, world!"),
    };

    notify(&tweet);
    notify_short(&tweet);
}
```

### Exercise 7: Standard Library Traits

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implement Display trait manually
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implement Default trait
impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = p1.clone();
    let p3 = Point::default();

    // Debug (from derive)
    println!("Debug: {:?}", p1);

    // Display (manual implementation)
    println!("Display: {}", p1);

    // PartialEq (from derive)
    println!("p1 == p2: {}", p1 == p2);

    // Default
    println!("Default point: {}", p3);
}
```

## Part 3: Trait Bounds in Practice (5 minutes)

### Exercise 8: Returning Traits

```rust
trait Animal {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says Woof!", self.name)
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says Meow!", self.name)
    }
}

// Return impl Trait (single concrete type)
fn get_dog() -> impl Animal {
    Dog { name: String::from("Buddy") }
}

// For multiple types, use trait objects (Box<dyn Trait>)
fn get_animal(animal_type: &str) -> Box<dyn Animal> {
    match animal_type {
        "dog" => Box::new(Dog { name: String::from("Buddy") }),
        "cat" => Box::new(Cat { name: String::from("Whiskers") }),
        _ => Box::new(Dog { name: String::from("Unknown") }),
    }
}

fn main() {
    let dog = get_dog();
    println!("{}", dog.speak());

    let animal = get_animal("cat");
    println!("{}", animal.speak());

    // Collection of different animals
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Rex") }),
        Box::new(Cat { name: String::from("Felix") }),
    ];

    for animal in animals {
        println!("{}", animal.speak());
    }
}
```

### Exercise 9: Conditional Implementations

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only implement cmp_display for types that are Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let string_pair = Pair::new("hello", "world");
    string_pair.cmp_display();
}
```

## Verification Steps

### Checklist
- [ ] Wrote generic functions with type parameters
- [ ] Created generic structs and enums
- [ ] Defined custom traits
- [ ] Implemented traits for structs
- [ ] Used trait bounds to constrain generics
- [ ] Implemented standard library traits (Display, Debug, Clone)
- [ ] Used impl Trait and dyn Trait syntax
- [ ] Created conditional implementations with trait bounds

## Lab Questions

1. What is the difference between `impl Trait` and `dyn Trait`?
2. What does the derive macro do?
3. Why do we need trait bounds on generic functions?
4. Can you implement external traits on external types?

## Answers

1. `impl Trait` is static dispatch (compiler knows exact type, zero cost). `dyn Trait` is dynamic dispatch (type determined at runtime, slight overhead).

2. `derive` automatically implements common traits based on the struct's fields. It generates the implementation code at compile time.

3. **Trait bounds** tell the compiler what capabilities a generic type must have. Without them, you can't call methods on the generic type.

4. No, this is the **orphan rule**. You can only implement a trait if either the trait or the type is local to your crate. This prevents conflicts.

## Common Issues

**Issue: "the trait bound is not satisfied"**
```
Solution: Add the required trait bound to your generic: fn foo<T: Clone>(x: T)
```

**Issue: "cannot return value referencing local variable"**
```
Solution: Return an owned value or use Box for trait objects.
```

## Next Steps

In Lab 12, you will:
- Understand lifetimes
- Write tests
- Use test organization

## Completion

You have completed Lab 11 when you can:
- Write generic functions, structs, and enums
- Define and implement custom traits
- Use trait bounds effectively
- Implement and derive standard library traits
