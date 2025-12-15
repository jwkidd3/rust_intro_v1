# Lab 6: Structs and Methods

## Duration: 25 minutes

## Objectives
- Define and instantiate structs
- Use field init shorthand and struct update syntax
- Implement methods using impl blocks
- Understand associated functions

## Prerequisites
- Completed Labs 1-5
- Understanding of ownership and references

## Part 1: Defining Structs (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab06_structs
cd lab06_structs
```

### Exercise 2: Basic Struct Definition

Edit `src/main.rs`:

```rust
// Define a struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    // Create an instance
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-in count: {}", user1.sign_in_count);
}
```

### Exercise 3: Mutable Struct Instances

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    // Entire instance must be mutable
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    // Modify a field
    user1.email = String::from("newemail@example.com");
    user1.sign_in_count += 1;

    println!("Updated email: {}", user1.email);
    println!("Sign-in count: {}", user1.sign_in_count);
}
```

### Exercise 4: Field Init Shorthand

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,     // Shorthand: same as email: email
        username,  // Shorthand: same as username: username
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user(
        String::from("test@test.com"),
        String::from("testuser"),
    );
    println!("Created user: {}", user.username);
}
```

### Exercise 5: Struct Update Syntax

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    // Create user2 from user1, changing only some fields
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1  // Use remaining fields from user1
    };

    // Note: user1.username was moved to user2
    println!("User2 email: {}", user2.email);
    println!("User2 username: {}", user2.username);
    // println!("{}", user1.username);  // ERROR: value moved
    println!("User1 active: {}", user1.active);  // OK: bool is Copy
}
```

## Part 2: Tuple Structs and Unit Structs (5 minutes)

### Exercise 6: Tuple Structs

```rust
// Tuple structs - named tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access by index
    println!("Black R: {}", black.0);
    println!("Origin x: {}", origin.0);

    // Destructuring
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);

    // Note: Color and Point are different types!
    // Even though they have the same field types
}
```

### Exercise 7: Unit-Like Structs

```rust
// Unit-like struct (no fields)
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    // Useful for traits without data (covered later)
}
```

## Part 3: Methods (10 minutes)

### Exercise 8: Implementing Methods

```rust
#[derive(Debug)]  // Enable {:?} printing
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method: takes &self as first parameter
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
    println!("Area: {} sq pixels", rect.area());
    println!("Perimeter: {} pixels", rect.perimeter());
}
```

### Exercise 9: Methods with Parameters

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

    // Method with additional parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### Exercise 10: Associated Functions (Constructors)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no self) - often used as constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Another associated function
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
    // Call associated functions with ::
    let rect = Rectangle::new(30, 50);
    let square = Rectangle::square(25);

    println!("Rectangle: {:?}, area: {}", rect, rect.area());
    println!("Square: {:?}, area: {}", square, square.area());
}
```

### Exercise 11: Methods that Modify Self

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Takes &mut self to modify
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // Takes self (ownership) - consumes the rectangle
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle::square(size)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect = Rectangle::new(10, 20);
    println!("Original: {:?}", rect);

    rect.double_size();
    println!("Doubled: {:?}", rect);

    let square = rect.into_square();
    // println!("{:?}", rect);  // ERROR: rect was moved
    println!("Square: {:?}", square);
}
```

## Verification Steps

### Checklist
- [ ] Defined structs with named fields
- [ ] Created struct instances
- [ ] Used field init shorthand
- [ ] Used struct update syntax with ..
- [ ] Created tuple structs
- [ ] Implemented methods with impl blocks
- [ ] Created associated functions (constructors)
- [ ] Used &self, &mut self, and self in methods

## Lab Questions

1. What is the difference between a method and an associated function?
2. What does `&self` mean in a method signature?
3. When would you use `self` instead of `&self`?
4. How do you call an associated function?

## Answers

1. A **method** takes `self` (or `&self`, `&mut self`) as its first parameter and is called on an instance. An **associated function** doesn't take `self` and is called on the type itself.

2. `&self` is shorthand for `self: &Self`. It borrows the instance immutably, allowing the method to read but not modify the struct.

3. Use `self` (taking ownership) when the method needs to transform the struct into something else, or when you want to prevent the caller from using the original instance.

4. Associated functions are called using `::` syntax: `Rectangle::new(30, 50)` or `Rectangle::square(25)`.

## Common Issues

**Issue: "cannot borrow as mutable"**
```
Solution: Make the instance mutable with `let mut`
```

**Issue: "use of moved value"**
```
Solution: The method took ownership with `self`. Use &self or clone if you need the original.
```

## Next Steps

In Lab 7, you will:
- Define enums with variants
- Use pattern matching with match
- Work with Option and if let

## Completion

You have completed Lab 6 when you can:
- Define and instantiate structs
- Implement methods and associated functions
- Choose the right self type (&self, &mut self, self)
- Use derive macros like #[derive(Debug)]
