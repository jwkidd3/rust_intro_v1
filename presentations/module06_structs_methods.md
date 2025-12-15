# Module 6: Structs and Methods

**Duration:** 50 minutes
**Type:** Presentation

---

## Slide 1: Title

**Structs and Methods**

- Custom Data Types in Rust
- Module 6 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Define and instantiate structs
- Use tuple structs and unit structs
- Add methods with impl blocks
- Understand self, &self, and &mut self
- Use associated functions
- Work with the derive attribute

---

## Slide 3: What is a Struct?

**A custom data type that groups related values:**

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

**Like a tuple, but with named fields**

---

## Slide 4: Creating Struct Instances

```rust
fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);
}
```

**Order doesn't matter when using field names**

---

## Slide 5: Mutable Structs

**The entire struct must be mutable:**

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

**Can't make individual fields mutable**

---

## Slide 6: Field Init Shorthand

**When variable names match field names:**

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

## Slide 7: Struct Update Syntax

**Create a new struct from an existing one:**

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

**Note:** user1.username is moved to user2

---

## Slide 8: Tuple Structs

**Structs without named fields:**

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access by index
    println!("R: {}", black.0);
    println!("X: {}", origin.0);

    // black and origin are different types!
}
```

---

## Slide 9: Unit-Like Structs

**Structs with no fields:**

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

**Useful for:**
- Implementing traits on a type with no data
- Marker types
- Zero-size types

---

## Slide 10: Printing Structs

**Structs don't implement Display by default:**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    // println!("{}", rect);  // ERROR: doesn't implement Display

    // Use debug format
    println!("{:?}", rect);   // ERROR: doesn't implement Debug
}
```

---

## Slide 11: The Debug Trait

**Derive Debug for easy printing:**

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

**Output:**
```
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}
```

---

## Slide 12: Methods

**Functions defined within a struct's context:**

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

## Slide 13: The impl Block

```
┌─────────────────────────────────────────────┐
│              impl Block                      │
├─────────────────────────────────────────────┤
│                                             │
│   impl StructName {                         │
│       fn method(&self) { }                  │
│       fn mutating(&mut self) { }            │
│       fn consuming(self) { }                │
│       fn associated() -> Self { }           │
│   }                                         │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Slide 14: Self Parameter

**Three ways to take self:**

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

    // Take ownership - rare, transforms
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}
```

---

## Slide 15: Method Call Syntax

**Rust has automatic referencing:**

```rust
fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    // These are equivalent:
    rect.area();
    (&rect).area();  // What Rust does behind the scenes
}
```

**Rust automatically adds `&`, `&mut`, or `*` to match the method signature**

---

## Slide 16: Methods with Parameters

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
}
```

---

## Slide 17: Associated Functions

**Functions in impl that don't take self:**

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);  // Called with ::
}
```

**Often used as constructors**

---

## Slide 18: The new Convention

**Idiomatic constructor pattern:**

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle::new(size, size)
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(10);
}
```

---

## Slide 19: Multiple impl Blocks

**You can have multiple impl blocks:**

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

**Useful for:**
- Organizing code
- Conditional compilation
- Implementing different traits

---

## Slide 20: Returning Self

**Use `Self` as return type:**

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn double(&self) -> Self {
        Self {
            width: self.width * 2,
            height: self.height * 2,
        }
    }
}
```

**`Self` is an alias for the type in the impl block**

---

## Slide 21: Common Derive Traits

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
        println!("Points are equal");
    }
}
```

---

## Slide 22: Struct Ownership

**Structs can own their data or borrow it:**

```rust
// Owned data - struct owns the String
struct User {
    username: String,    // Owned
    email: String,       // Owned
}

// Borrowed data - requires lifetimes (Module 12)
// struct UserRef<'a> {
//     username: &'a str,   // Borrowed
//     email: &'a str,      // Borrowed
// }
```

**For now, prefer owned types in structs**

---

## Slide 23: Key Takeaways

1. **Structs** group related data with named fields
2. **Tuple structs** have unnamed fields
3. **Methods** are defined in `impl` blocks
4. **`&self`** borrows, **`&mut self`** mutably borrows, **`self`** takes ownership
5. **Associated functions** don't take self, called with `::`
6. **`#[derive(Debug)]`** enables debug printing
7. **`Self`** is an alias for the struct type

---

## Slide 24: Lab Preview

**Lab 6: Structs and Methods** (30 minutes)

You will:
- Define custom structs
- Create instances and access fields
- Use tuple structs
- Implement methods with impl
- Create associated functions (constructors)
- Use derive for Debug and Clone

---

## Questions?

**Next Module:** Enums and Pattern Matching
