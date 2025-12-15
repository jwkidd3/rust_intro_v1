# Lab 4: Ownership Basics

## Duration: 25 minutes

## Objectives
- Understand Rust's ownership rules
- Learn about move semantics
- Work with Copy and Clone traits
- Understand stack vs heap allocation

## Prerequisites
- Completed Labs 1-3
- Understanding of functions and variables

## Part 1: Understanding Ownership (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab04_ownership
cd lab04_ownership
```

### Exercise 2: Stack vs Heap

Edit `src/main.rs`:

```rust
fn main() {
    // Stack allocation - fixed size, fast
    let x = 5;           // i32 on stack
    let y = x;           // Copy of value on stack
    println!("x = {}, y = {}", x, y);  // Both valid!

    // Heap allocation - dynamic size
    let s1 = String::from("hello");  // String on heap
    let s2 = s1;                      // Ownership MOVES to s2

    // println!("s1 = {}", s1);  // ERROR! s1 no longer valid
    println!("s2 = {}", s2);      // s2 owns the data now
}
```

### Exercise 3: The Three Ownership Rules

```rust
fn main() {
    // Rule 1: Each value has exactly one owner
    let s = String::from("hello");  // s is the owner

    // Rule 2: Only one owner at a time
    let s2 = s;  // Ownership moved from s to s2
    // s is no longer valid

    // Rule 3: Value is dropped when owner goes out of scope
    {
        let s3 = String::from("world");
        println!("s3 = {}", s3);
    }  // s3 goes out of scope and is dropped here

    println!("s2 = {}", s2);
}
```

### Exercise 4: Move Semantics

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2

    // Uncommenting the next line causes a compile error
    // println!("s1 = {}", s1);

    println!("s2 = {}", s2);

    // Visualizing the move:
    // Before: s1 -> [ptr | len: 5 | cap: 5] -> "hello"
    // After:  s2 -> [ptr | len: 5 | cap: 5] -> "hello"
    //         s1 is invalidated
}
```

### Exercise 5: Clone for Deep Copy

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy of heap data

    println!("s1 = {}", s1);  // Both are valid!
    println!("s2 = {}", s2);

    // Both now own their own data:
    // s1 -> "hello"
    // s2 -> "hello" (separate copy)
}
```

## Part 2: Copy Types (5 minutes)

### Exercise 6: Copy Trait

```rust
fn main() {
    // Types that implement Copy (stack-only data)
    let x = 5;
    let y = x;  // Copy, not move
    println!("x = {}, y = {}", x, y);  // Both valid!

    // Copy types include:
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'z';

    // Tuples of Copy types are also Copy
    let point = (3, 4);
    let point2 = point;
    println!("point = {:?}, point2 = {:?}", point, point2);

    // Arrays of Copy types are Copy
    let arr = [1, 2, 3];
    let arr2 = arr;
    println!("arr = {:?}, arr2 = {:?}", arr, arr2);
}
```

## Part 3: Ownership and Functions (10 minutes)

### Exercise 7: Passing Ownership to Functions

```rust
fn main() {
    let s = String::from("hello");

    takes_ownership(s);  // s's value moves into function

    // println!("{}", s);  // ERROR: s is no longer valid

    let x = 5;
    makes_copy(x);  // i32 is Copy, so x is still valid

    println!("x is still valid: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("In function: {}", some_string);
}  // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("In function: {}", some_integer);
}  // some_integer goes out of scope, nothing special happens
```

### Exercise 8: Returning Ownership

```rust
fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2);  // ERROR: s2 moved
    println!("s3 = {}", s3);  // s3 is valid
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // Returned and ownership moves out
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Returned and ownership moves out
}
```

### Exercise 9: Returning Multiple Values

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return tuple with ownership and result
}
```

### Exercise 10: Ownership Challenge

Fix the following code so it compiles:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    // We want to print both strings and still use them after
    print_string(s1.clone());  // Clone to keep ownership
    print_string(s2.clone());

    // Both strings still valid here
    println!("s1: {}, s2: {}", s1, s2);

    // Now transfer ownership
    let s3 = combine_strings(s1, s2);
    // s1 and s2 are no longer valid
    println!("Combined: {}", s3);
}

fn print_string(s: String) {
    println!("String: {}", s);
}

fn combine_strings(a: String, b: String) -> String {
    let mut result = a;
    result.push_str(" ");
    result.push_str(&b);
    result
}
```

## Verification Steps

### Checklist
- [ ] Understand difference between stack and heap allocation
- [ ] Can explain the three ownership rules
- [ ] Understand move semantics with String
- [ ] Know when Copy vs Clone is used
- [ ] Can pass ownership to functions
- [ ] Can return ownership from functions
- [ ] Understand when values are dropped

## Lab Questions

1. What are the three rules of ownership?
2. Why doesn't `i32` move but `String` does?
3. What happens when a variable goes out of scope?
4. How do you create a deep copy of a String?

## Answers

1. **Three rules**: (1) Each value has exactly one owner. (2) There can only be one owner at a time. (3) When the owner goes out of scope, the value is dropped.

2. `i32` implements the **Copy trait** (stored entirely on stack, cheap to copy). `String` stores data on the heap, so copying would be expensive; Rust moves it instead.

3. Rust automatically calls the **drop** function, which cleans up heap memory if applicable. Stack memory is reclaimed automatically.

4. Use the **clone()** method: `let s2 = s1.clone();`

## Common Issues

**Issue: "borrow of moved value"**
```
Solution: The value was moved. Either:
- Clone the value before moving
- Use references (covered in Lab 5)
- Restructure code to avoid the move
```

**Issue: "cannot move out of borrowed content"**
```
Solution: You're trying to move a value that's borrowed.
Use clone() or restructure your code.
```

## Next Steps

In Lab 5, you will:
- Learn about references and borrowing
- Understand mutable vs immutable references
- Work with the slice type

## Completion

You have completed Lab 4 when you can:
- Explain what ownership is and why Rust uses it
- Predict whether a value will be moved or copied
- Track ownership through function calls
- Use clone() when needed to retain ownership
