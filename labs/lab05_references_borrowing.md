# Lab 5: References and Borrowing

## Duration: 25 minutes

## Objectives
- Understand references as non-owning pointers
- Work with immutable and mutable references
- Learn the borrowing rules
- Use slices for partial data access

## Prerequisites
- Completed Lab 4
- Understanding of ownership

## Part 1: References (10 minutes)

### Exercise 1: Create a New Project

```bash
cargo new lab05_references
cd lab05_references
```

### Exercise 2: Immutable References

Edit `src/main.rs`:

```rust
fn main() {
    let s1 = String::from("hello");

    // Create a reference - borrows s1 without taking ownership
    let len = calculate_length(&s1);

    // s1 is still valid because we only borrowed it
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop what it refers to
```

### Exercise 3: Multiple Immutable References

```rust
fn main() {
    let s = String::from("hello");

    // Multiple immutable references are allowed
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    println!("Original: {}", s);  // s is still accessible
}
```

### Exercise 4: References Cannot Modify

```rust
fn main() {
    let s = String::from("hello");

    // Uncommenting the next line causes an error
    // change(&s);  // Cannot modify through immutable reference
}

// This won't compile:
// fn change(some_string: &String) {
//     some_string.push_str(", world");  // ERROR!
// }
```

## Part 2: Mutable References (10 minutes)

### Exercise 5: Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);  // Pass mutable reference

    println!("{}", s);  // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Exercise 6: Mutable Reference Restriction

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // ERROR: only one mutable reference allowed

    println!("{}", r1);

    // After r1 is no longer used, we can create another
    let r2 = &mut s;
    println!("{}", r2);
}
```

### Exercise 7: Cannot Mix Mutable and Immutable

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // immutable borrow
    let r2 = &s;     // another immutable borrow - OK
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // mutable borrow - OK because r1, r2 are done
    println!("{}", r3);
}
```

### Exercise 8: Non-Lexical Lifetimes (NLL)

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2's scope ends here (NLL)

    // This is allowed because r1 and r2 are no longer used
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
```

## Part 3: Slices (5 minutes)

### Exercise 9: String Slices

```rust
fn main() {
    let s = String::from("hello world");

    // String slices reference part of a String
    let hello = &s[0..5];   // or &s[..5]
    let world = &s[6..11];  // or &s[6..]
    let whole = &s[..];     // entire string

    println!("'{}' and '{}'", hello, world);
    println!("Whole: '{}'", whole);

    // Find first word
    let word = first_word(&s);
    println!("First word: '{}'", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### Exercise 10: Array Slices

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // Array slice
    let slice = &a[1..3];
    println!("Slice: {:?}", slice);  // [2, 3]

    // Pass slice to function
    let sum = sum_slice(&a[..]);
    println!("Sum of all: {}", sum);

    let partial_sum = sum_slice(&a[0..3]);
    println!("Sum of first 3: {}", partial_sum);
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &item in slice {
        total += item;
    }
    total
}
```

### Exercise 11: String Literals are Slices

```rust
fn main() {
    // String literals are slices (&str)
    let s: &str = "Hello, world!";

    // Works with both String and &str
    print_str(&String::from("From String"));
    print_str("From literal");
}

fn print_str(s: &str) {
    println!("{}", s);
}
```

## Verification Steps

### Checklist
- [ ] Created immutable references with &
- [ ] Created mutable references with &mut
- [ ] Understand you can have multiple immutable OR one mutable reference
- [ ] Can pass references to functions
- [ ] Can modify data through mutable references
- [ ] Created string slices with &s[start..end]
- [ ] Created array slices
- [ ] Understand &str vs String

## Lab Questions

1. What is the difference between `&s` and `&mut s`?
2. Why can't you have a mutable reference while immutable references exist?
3. What is the type of a string slice?
4. What is the difference between `String` and `&str`?

## Answers

1. `&s` is an **immutable reference** (can read but not modify). `&mut s` is a **mutable reference** (can read and modify).

2. To prevent **data races**. If someone is reading data, another can't modify it simultaneously. This is checked at compile time.

3. `&str` (pronounced "string slice") - it's a reference to a sequence of UTF-8 bytes.

4. **String** is an owned, growable string stored on the heap. **&str** is a borrowed view into a string (could be part of a String or a string literal).

## Common Issues

**Issue: "cannot borrow as mutable because it is also borrowed as immutable"**
```
Solution: Ensure all immutable borrows are done being used before creating a mutable borrow.
```

**Issue: "cannot borrow as mutable, as it is not declared as mutable"**
```
Solution: Add 'mut' to the variable declaration: let mut s = ...
```

**Issue: "byte index is not a char boundary"**
```
Solution: String slices must be on character boundaries. For non-ASCII strings, be careful with indices.
```

## Borrowing Rules Summary

```
1. At any given time, you can have EITHER:
   - One mutable reference, OR
   - Any number of immutable references

2. References must always be valid
   (no dangling references)
```

## Next Steps

In Lab 6, you will:
- Define and use structs
- Implement methods with impl blocks
- Understand self and &self

## Completion

You have completed Lab 5 when you can:
- Create and use immutable and mutable references
- Explain the borrowing rules
- Use slices for strings and arrays
- Choose between &str and String appropriately
