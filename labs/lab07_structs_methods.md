# Lab 7: Structs and Methods

**Duration:** 35 minutes

## Objectives

- Define structs with named fields
- Create tuple structs
- Implement methods using impl
- Create associated functions (constructors)
- Use derive for Debug, Clone

## Prerequisites

- Completed Lab 6

---

## Setup

Create a new project:
```bash
cargo new lab07_structs
cd lab07_structs
code .
```

---

## Exercise 1: Defining Structs (10 min)

### Step 1: Basic struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-ins: {}", user1.sign_in_count);
}
```

### Step 2: Mutable struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    println!("Original email: {}", user1.email);

    user1.email = String::from("new_email@example.com");
    user1.sign_in_count += 1;

    println!("Updated email: {}", user1.email);
    println!("Sign-ins: {}", user1.sign_in_count);
}
```

### Step 3: Field init shorthand and struct update

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // shorthand
        username,   // shorthand
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("user1@example.com"),
        String::from("user1"),
    );

    // Struct update syntax
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1  // use remaining fields from user1
    };

    println!("User2 email: {}", user2.email);
    println!("User2 username: {}", user2.username);
}
```

---

## Exercise 2: Tuple Structs and Debug (5 min)

### Step 1: Tuple structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // These are different types!
    // let color: Color = origin;  // Error!
}
```

### Step 2: Using Debug derive

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {:?}", rect);
    println!("Rectangle (pretty): {:#?}", rect);
}
```

---

## Exercise 3: Methods (10 min)

### Step 1: Basic methods

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

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}
```

### Step 2: Methods that modify self

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

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Original: {:?}", rect);
    println!("Area: {}", rect.area());

    rect.double();
    println!("After double: {:?}", rect);
    println!("Area: {}", rect.area());

    rect.set_width(10);
    println!("After set_width: {:?}", rect);
}
```

### Step 3: Methods with parameters

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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
}
```

---

## Exercise 4: Associated Functions (5 min)

### Step 1: Constructor pattern

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no self) - constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Call with ::
    let rect = Rectangle::new(30, 50);
    let square = Rectangle::square(10);

    println!("Rectangle: {:?}, area: {}", rect, rect.area());
    println!("Square: {:?}, area: {}", square, square.area());
}
```

---

## Exercise 5: Using Clone and PartialEq (5 min)

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        let sum = (self.x * self.x + self.y * self.y) as f64;
        sum.sqrt()
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    let p2 = p1.clone();  // Clone
    let p3 = Point::new(3, 4);

    println!("p1: {:?}", p1);
    println!("p2 (cloned): {:?}", p2);

    // PartialEq allows ==
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);

    println!("p1 distance from origin: {:.2}", p1.distance_from_origin());
}
```

---

## Challenge Exercise (Bonus)

Create a `BankAccount` struct with:
- Fields: `owner` (String), `balance` (f64)
- Associated function `new(owner: &str, initial_balance: f64)`
- Method `deposit(&mut self, amount: f64)`
- Method `withdraw(&mut self, amount: f64) -> Result<f64, String>`
- Method `balance(&self) -> f64`

```rust
#[derive(Debug)]
struct BankAccount {
    // Your fields here
}

impl BankAccount {
    // Your implementation here
}

fn main() {
    let mut account = BankAccount::new("Alice", 100.0);
    println!("Initial: {:?}", account);

    account.deposit(50.0);
    println!("After deposit: {:?}", account);

    match account.withdraw(30.0) {
        Ok(amount) => println!("Withdrew: {}", amount),
        Err(e) => println!("Error: {}", e),
    }

    match account.withdraw(200.0) {
        Ok(amount) => println!("Withdrew: {}", amount),
        Err(e) => println!("Error: {}", e),
    }

    println!("Final balance: {}", account.balance());
}
```

---

## Verification Checklist

- [ ] Created structs with named fields
- [ ] Used mutable structs and modified fields
- [ ] Used field init shorthand
- [ ] Used struct update syntax (..)
- [ ] Created tuple structs
- [ ] Used #[derive(Debug)] for printing
- [ ] Implemented methods with &self
- [ ] Implemented methods with &mut self
- [ ] Implemented methods with parameters
- [ ] Created associated functions (constructors)
- [ ] Used Clone and PartialEq derives

---

## Summary

You have learned:
- Structs group related data with named fields
- Use `mut` to make entire struct mutable
- `impl` blocks contain methods and associated functions
- `&self` borrows, `&mut self` borrows mutably
- Associated functions (no self) are called with `::`
- `#[derive(...)]` auto-implements common traits

**Next:** Lab 8 - Traits and Generics
