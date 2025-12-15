# Module 4: Ownership Basics

**Duration:** 50 minutes
**Type:** Presentation

---

## Slide 1: Title

**Ownership - Rust's Key Innovation**

- Memory Safety Without Garbage Collection
- Module 4 of 12

---

## Slide 2: Module Objectives

By the end of this module, you will:

- Understand what ownership is and why it matters
- Know the three rules of ownership
- Understand move semantics
- Work with Copy and Clone traits
- Understand stack vs heap allocation

---

## Slide 3: The Memory Problem

**Traditional approaches:**

| Approach | Language | Problem |
|----------|----------|---------|
| Manual | C/C++ | Use-after-free, double-free, leaks |
| Garbage Collection | Java, Go, Python | Runtime overhead, unpredictable pauses |

**Rust's approach:**
- Compile-time memory management
- Zero runtime cost
- Guaranteed safety

---

## Slide 4: Stack vs Heap

```
┌─────────────────────────────────────────────┐
│                  Stack                       │
├─────────────────────────────────────────────┤
│ • Fixed size, known at compile time         │
│ • Fast allocation/deallocation              │
│ • Automatic cleanup when scope ends         │
│ • Examples: i32, f64, bool, [i32; 5]        │
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│                   Heap                       │
├─────────────────────────────────────────────┤
│ • Dynamic size, determined at runtime       │
│ • Slower allocation (find space, bookkeep)  │
│ • Must be explicitly managed                │
│ • Examples: String, Vec<T>, Box<T>          │
└─────────────────────────────────────────────┘
```

---

## Slide 5: String: A Heap-Allocated Type

```rust
fn main() {
    let s = String::from("hello");
}
```

**Memory layout:**
```
Stack                    Heap
┌─────────────┐         ┌───┬───┬───┬───┬───┐
│ ptr     ────┼────────►│ h │ e │ l │ l │ o │
│ len: 5      │         └───┴───┴───┴───┴───┘
│ capacity: 5 │
└─────────────┘
     s
```

---

## Slide 6: The Three Rules of Ownership

```
┌─────────────────────────────────────────────┐
│         Rust's Ownership Rules              │
├─────────────────────────────────────────────┤
│                                             │
│  1. Each value has exactly ONE owner        │
│                                             │
│  2. There can only be ONE owner at a time   │
│                                             │
│  3. When the owner goes out of scope,       │
│     the value is dropped                    │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Slide 7: Rule 1: One Owner

```rust
fn main() {
    let s = String::from("hello");
    //  ^ s is the owner of this String

    println!("{}", s);
}   // s goes out of scope, String is dropped
```

**The variable `s` owns the String data**

---

## Slide 8: Rule 2: Only One Owner at a Time

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership MOVES from s1 to s2

    // println!("{}", s1);  // ERROR: s1 no longer valid
    println!("{}", s2);     // OK: s2 owns the data
}
```

**After the move, s1 is invalid**

---

## Slide 9: Visualizing a Move

```
Before move:
Stack                    Heap
┌─────────────┐         ┌───────────┐
│ s1.ptr  ────┼────────►│  "hello"  │
│ s1.len: 5   │         └───────────┘
│ s1.cap: 5   │
└─────────────┘

After: let s2 = s1;
Stack                    Heap
┌─────────────┐         ┌───────────┐
│ s1 INVALID  │    ┌───►│  "hello"  │
├─────────────┤    │    └───────────┘
│ s2.ptr  ────┼────┘
│ s2.len: 5   │
│ s2.cap: 5   │
└─────────────┘
```

---

## Slide 10: Rule 3: Drop When Out of Scope

```rust
fn main() {
    {
        let s = String::from("hello");
        // s is valid here
    }   // s goes out of scope
        // Rust calls drop(), memory is freed

    // s is not valid here
}
```

**Rust automatically cleans up when owner goes out of scope**

---

## Slide 11: Why Move Instead of Copy?

**Problem with copying heap data:**

```
If Rust copied heap data:
Stack                    Heap
┌─────────────┐    ┌────►┌───────────┐
│ s1.ptr  ────┼────┤     │  "hello"  │
└─────────────┘    │     └───────────┘
┌─────────────┐    │
│ s2.ptr  ────┼────┘     Both point to same data!
└─────────────┘          Double-free when both drop!
```

**Solution:** Move ownership instead of shallow copy

---

## Slide 12: Clone for Deep Copy

**When you actually need a copy:**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy

    println!("s1 = {}", s1);  // Both valid!
    println!("s2 = {}", s2);
}
```

**clone() creates independent copy of heap data**

---

## Slide 13: Visualizing Clone

```
After: let s2 = s1.clone();

Stack                    Heap
┌─────────────┐         ┌───────────┐
│ s1.ptr  ────┼────────►│  "hello"  │
│ s1.len: 5   │         └───────────┘
│ s1.cap: 5   │
├─────────────┤         ┌───────────┐
│ s2.ptr  ────┼────────►│  "hello"  │  (separate copy)
│ s2.len: 5   │         └───────────┘
│ s2.cap: 5   │
└─────────────┘
```

---

## Slide 14: The Copy Trait

**Stack-only types don't move, they copy:**

```rust
fn main() {
    let x = 5;
    let y = x;  // Copy, not move!

    println!("x = {}, y = {}", x, y);  // Both valid!
}
```

**Types that implement Copy:**
- All integer types (i32, u64, etc.)
- Boolean (bool)
- Floating point (f32, f64)
- Character (char)
- Tuples of Copy types: (i32, i32)
- Arrays of Copy types: [i32; 5]

---

## Slide 15: Copy vs Clone

| Copy | Clone |
|------|-------|
| Implicit | Explicit (call .clone()) |
| Bitwise copy | Can be custom logic |
| Stack data only | Can copy heap data |
| Very fast | May be expensive |
| No custom code | Implement Clone trait |

```rust
let x = 5;
let y = x;      // Copy (implicit)

let s1 = String::from("hi");
let s2 = s1.clone();  // Clone (explicit)
```

---

## Slide 16: Ownership and Functions

**Passing a value to a function moves or copies:**

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);     // s moves into function

    // println!("{}", s);   // ERROR: s is invalid

    let x = 5;
    makes_copy(x);          // x is copied
    println!("{}", x);      // OK: x still valid
}

fn takes_ownership(s: String) {
    println!("{}", s);
}   // s is dropped here

fn makes_copy(x: i32) {
    println!("{}", x);
}
```

---

## Slide 17: Returning Ownership

**Functions can return ownership:**

```rust
fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s2 is invalid, s3 is valid
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s  // Return ownership to caller
}
```

---

## Slide 18: The Problem with Moving

**Moving gets tedious:**

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    // Had to return s1 back to use it again!

    println!("'{}' has length {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return both!
}
```

**Solution:** References (next module!)

---

## Slide 19: Memory Safety Guarantees

**Ownership prevents:**

| Bug | How Ownership Prevents It |
|-----|--------------------------|
| Use after free | Compiler tracks ownership, prevents use after move |
| Double free | Only one owner, only one drop |
| Memory leaks | Automatic drop when out of scope |
| Null pointers | No null in safe Rust |
| Dangling pointers | Compiler validates all references |

---

## Slide 20: Common Ownership Patterns

```rust
// Pattern 1: Create and use locally
fn process() {
    let data = create_data();
    use_data(&data);  // Borrow (next module)
}

// Pattern 2: Return ownership
fn create() -> String {
    String::from("new data")
}

// Pattern 3: Clone when needed
fn share() {
    let original = String::from("data");
    let copy = original.clone();
    // Both can be used independently
}
```

---

## Slide 21: Summary: Ownership Flow

```
┌─────────────────────────────────────────────┐
│              Ownership Flow                  │
├─────────────────────────────────────────────┤
│                                             │
│   let s = String::from("hello");            │
│        │                                    │
│        ▼                                    │
│   ┌─────────┐     Move      ┌─────────┐    │
│   │   s     │ ────────────► │   s2    │    │
│   │ (owner) │  let s2 = s;  │ (owner) │    │
│   │ INVALID │               │  VALID  │    │
│   └─────────┘               └────┬────┘    │
│                                  │         │
│                                  ▼         │
│                            } // dropped    │
└─────────────────────────────────────────────┘
```

---

## Slide 22: Key Takeaways

1. **Every value has exactly one owner**
2. **Assignment moves ownership** (for heap data)
3. **When owner goes out of scope, value is dropped**
4. **Stack types implement Copy** - they copy, not move
5. **Use clone()** for explicit deep copies
6. **Functions can take and return ownership**
7. **Ownership enables memory safety without GC**

---

## Slide 23: Lab Preview

**Lab 4: Ownership** (25 minutes)

You will:
- Observe move semantics with String
- Use Clone for deep copies
- Work with Copy types
- Pass ownership to functions
- Return ownership from functions
- Fix ownership-related compiler errors

---

## Questions?

**Next Module:** References and Borrowing
