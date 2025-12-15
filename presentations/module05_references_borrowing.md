# Module 5: References and Borrowing

**Duration:** 45 minutes
**Type:** Presentation

---

## Slide 1: Title

**References and Borrowing**

- Using Data Without Taking Ownership
- Module 5 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand references as non-owning pointers
- Create immutable and mutable references
- Apply the borrowing rules
- Work with the slice type
- Understand the difference between &str and String

---

## Slide 3: The Problem

**Moving ownership is inconvenient:**

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    // Can't use s1 anymore!
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

**We want to use data without taking ownership**

---

## Slide 4: References to the Rescue

**A reference borrows without taking ownership:**

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Pass reference

    println!("'{}' has length {}", s1, len);  // s1 still valid!
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

## Slide 5: Visualizing References

```
Stack                    Heap
┌─────────────┐         ┌───────────┐
│ s1.ptr  ────┼────────►│  "hello"  │
│ s1.len: 5   │         └───────────┘
│ s1.cap: 5   │
└─────────────┘
       ▲
       │
┌──────┴──────┐
│ s (ref to   │
│    s1)      │
└─────────────┘

s points to s1, not the heap data
```

---

## Slide 6: Creating References

**Use `&` to create a reference:**

```rust
fn main() {
    let x = 5;
    let r = &x;      // r is a reference to x

    println!("x = {}", x);
    println!("r = {}", r);   // Automatically dereferenced
    println!("*r = {}", *r); // Explicit dereference
}
```

**References are like pointers, but guaranteed valid**

---

## Slide 7: Borrowing

**Creating a reference = borrowing**

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;  // r1 borrows s
    let r2 = &s;  // r2 also borrows s

    println!("{}, {}", r1, r2);
}   // r1, r2 go out of scope, s is still owner
```

**Multiple immutable borrows are allowed**

---

## Slide 8: References Are Immutable by Default

**Cannot modify through an immutable reference:**

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;

    // r.push_str(" world");  // ERROR: cannot borrow as mutable
}

fn change(s: &String) {
    // s.push_str(" world");  // ERROR!
}
```

---

## Slide 9: Mutable References

**Use `&mut` for mutable references:**

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);  // Prints: hello world
}

fn change(s: &mut String) {
    s.push_str(" world");
}
```

**Note:** The variable must also be `mut`

---

## Slide 10: The Borrowing Rules

```
┌─────────────────────────────────────────────┐
│           Rust's Borrowing Rules            │
├─────────────────────────────────────────────┤
│                                             │
│  At any given time, you can have EITHER:    │
│                                             │
│    • ONE mutable reference                  │
│                  OR                         │
│    • ANY NUMBER of immutable references     │
│                                             │
│  References must always be valid            │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Slide 11: Why These Rules?

**Prevents data races at compile time:**

A data race occurs when:
1. Two or more pointers access the same data simultaneously
2. At least one is writing
3. No synchronization

```rust
// NOT ALLOWED - potential data race
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ERROR: second mutable borrow
```

---

## Slide 12: Mutable Reference Restriction

**Only one mutable reference at a time:**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // ERROR!

    println!("{}", r1);
}
```

---

## Slide 13: Cannot Mix Mutable and Immutable

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // OK
    let r2 = &s;      // OK
    // let r3 = &mut s;  // ERROR: cannot borrow as mutable

    println!("{}, {}", r1, r2);
}
```

**Readers don't expect values to change!**

---

## Slide 14: Non-Lexical Lifetimes (NLL)

**References' scope ends at last use, not block end:**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 and r2 no longer used after this

    let r3 = &mut s;  // OK! r1, r2 scope ended
    println!("{}", r3);
}
```

---

## Slide 15: Dangling References

**Rust prevents dangling references:**

```rust
fn main() {
    let r = dangle();
}

fn dangle() -> &String {       // ERROR!
    let s = String::from("hello");
    &s  // Return reference to local variable
}   // s is dropped here, reference would be invalid!
```

**Solution: Return the owned value**

```rust
fn no_dangle() -> String {
    String::from("hello")  // Return ownership
}
```

---

## Slide 16: The Slice Type

**A reference to a contiguous sequence:**

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"

    println!("{} {}", hello, world);
}
```

**Slices are references - they don't own data**

---

## Slide 17: Slice Syntax

```rust
let s = String::from("hello");

let slice = &s[0..2];   // "he"
let slice = &s[..2];    // Same: "he"
let slice = &s[3..];    // "lo"
let slice = &s[..];     // "hello" (entire string)

// Range syntax
// [start..end)  - end is exclusive
// [start..=end] - end is inclusive
```

---

## Slide 18: String Slices (&str)

**`&str` is a string slice type:**

```rust
fn main() {
    // String slice from String
    let s = String::from("hello world");
    let word = &s[0..5];  // type: &str

    // String literal is also &str
    let literal: &str = "hello world";

    // Function accepting &str works with both
    print_str(word);
    print_str(literal);
    print_str(&s);  // &String coerces to &str
}

fn print_str(s: &str) {
    println!("{}", s);
}
```

---

## Slide 19: String vs &str

| String | &str |
|--------|------|
| Owned | Borrowed |
| Heap-allocated | Points to data |
| Growable | Fixed size |
| Can modify | Read-only |
| Use for owned text | Use for borrowed text |

```rust
fn takes_ownership(s: String) { }
fn borrows(s: &str) { }  // More flexible!
```

---

## Slide 20: Array Slices

**Slices work on arrays too:**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];  // [2, 3]

    println!("{:?}", slice);
}

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    let mut i = 0;
    while i < numbers.len() {
        total += numbers[i];
        i += 1;
    }
    total
}
```

---

## Slide 21: Slice Type Syntax

```rust
// String slice
let s: &str = "hello";

// Array slice
let a: &[i32] = &[1, 2, 3];

// Mutable slice
let mut arr = [1, 2, 3];
let slice: &mut [i32] = &mut arr[..];
slice[0] = 10;
```

---

## Slide 22: Summary: Reference Types

| Syntax | Type | Description |
|--------|------|-------------|
| `&T` | Reference | Immutable borrow of T |
| `&mut T` | Mutable ref | Mutable borrow of T |
| `&str` | String slice | Reference to string data |
| `&[T]` | Slice | Reference to array/vec data |

---

## Slide 23: Key Takeaways

1. **References borrow without taking ownership**
2. **`&T`** is immutable, **`&mut T`** is mutable
3. **One mutable OR many immutable** references
4. **References must always be valid**
5. **Slices** are references to portions of data
6. **`&str`** is more flexible than `&String` in function params
7. **NLL** allows references to end at last use

---

## Slide 24: Lab Preview

**Lab 5: References and Borrowing** (25 minutes)

You will:
- Create immutable and mutable references
- Experience borrowing rule violations
- Pass references to functions
- Work with string slices
- Work with array slices
- Understand &str vs String

---

## Questions?

**Next Module:** Structs and Methods
