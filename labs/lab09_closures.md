# Lab 9: Closures

**Duration:** 30 minutes

## Objectives

- Write basic closures
- Use closures with different capture modes
- Pass closures to functions
- Store closures in variables

## Prerequisites

- Completed Labs 7 and 8

---

## Setup

Create a new project:
```bash
cargo new lab06_functional
cd lab06_functional
code .
```

---

## Exercise 1: Basic Closures (10 min)

### Step 1: Simple closure syntax

```rust
fn main() {
    // Regular function
    fn add_one_fn(x: i32) -> i32 {
        x + 1
    }

    // Closure with full type annotations
    let add_one_v1 = |x: i32| -> i32 { x + 1 };

    // Closure with inferred types
    let add_one_v2 = |x| x + 1;

    // Using the closures
    let num = 5;
    println!("Function: {}", add_one_fn(num));
    println!("Closure v1: {}", add_one_v1(num));
    println!("Closure v2: {}", add_one_v2(num));
}
```

### Step 2: Closures with multiple parameters

```rust
fn main() {
    let add = |a, b| a + b;
    let multiply = |a: i32, b: i32| -> i32 { a * b };

    println!("3 + 4 = {}", add(3, 4));
    println!("3 * 4 = {}", multiply(3, 4));

    // Closure with no parameters
    let greet = || println!("Hello!");
    greet();

    // Closure that returns a value
    let get_number = || 42;
    println!("The number is: {}", get_number());
}
```

### Step 3: Multi-line closures

```rust
fn main() {
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        println!("Sum: {}, Product: {}", sum, product);
        sum + product
    };

    let result = calculate(3, 4);
    println!("Final result: {}", result);
}
```

---

## Exercise 2: Capturing Environment (10 min)

### Step 1: Borrowing immutably

```rust
fn main() {
    let message = String::from("Hello");
    let factor = 10;

    // Closure borrows message and factor immutably
    let print_message = || {
        println!("{}, factor is {}", message, factor);
    };

    print_message();
    print_message();

    // Original variables still accessible
    println!("Original message: {}", message);
    println!("Original factor: {}", factor);
}
```

### Step 2: Borrowing mutably

```rust
fn main() {
    let mut count = 0;

    // Closure borrows count mutably
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };

    increment();
    increment();
    increment();

    // Can use count after closure is done
    println!("Final count: {}", count);
}
```

### Step 3: Taking ownership with move

```rust
fn main() {
    let message = String::from("Hello");

    // move forces ownership transfer
    let print_message = move || {
        println!("{}", message);
    };

    print_message();

    // This would error - message was moved
    // println!("{}", message);

    // With Copy types, move copies the value
    let num = 42;
    let print_num = move || println!("{}", num);
    print_num();
    println!("num is still: {}", num);  // Works! i32 implements Copy
}
```

### Step 4: Understanding capture modes

```rust
fn main() {
    let s = String::from("hello");
    let n = 5;

    // Borrows s (String doesn't implement Copy)
    let closure1 = || println!("s = {}", s);
    closure1();
    println!("s after closure1: {}", s);  // Still valid

    // With move, takes ownership of s
    let closure2 = move || println!("s = {}", s);
    closure2();
    // println!("s after closure2: {}", s);  // Error: s was moved

    // n is Copy, so even with move, n is copied
    let closure3 = move || println!("n = {}", n);
    closure3();
    println!("n after closure3: {}", n);  // Still valid
}
```

---

## Exercise 3: Closures as Function Parameters (10 min)

### Step 1: Function accepting closure

```rust
fn apply_to_number<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let double = |x| x * 2;
    let square = |x| x * x;

    println!("double(5) = {}", apply_to_number(double, 5));
    println!("square(5) = {}", apply_to_number(square, 5));

    // Can also pass inline
    println!("triple(5) = {}", apply_to_number(|x| x * 3, 5));
}
```

### Step 2: Using Fn, FnMut, and FnOnce

```rust
// Fn - borrows captured values immutably
fn call_twice<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
}

// FnMut - can borrow captured values mutably
fn call_with_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

// FnOnce - takes ownership, can only call once
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
    // f();  // Error: can't call twice
}

fn main() {
    // Fn closure
    let message = "Hello";
    call_twice(|| println!("{}", message));

    // FnMut closure
    let mut count = 0;
    call_with_mut(|| {
        count += 1;
        println!("Count: {}", count);
    });
    println!("Final count: {}", count);

    // FnOnce closure
    let s = String::from("goodbye");
    call_once(move || println!("{}", s));
}
```

### Step 3: Returning closures

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x * y)
}

fn main() {
    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("add_5(3) = {}", add_5(3));   // 8
    println!("add_10(3) = {}", add_10(3)); // 13

    let mult_3 = make_multiplier(3);
    println!("mult_3(4) = {}", mult_3(4)); // 12
}
```

---

## Challenge Exercise (Bonus)

Create a simple calculator that uses closures:

```rust
struct Calculator {
    operations: Vec<Box<dyn Fn(i32, i32) -> i32>>,
    names: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            operations: Vec::new(),
            names: Vec::new(),
        }
    }

    fn add_operation<F>(&mut self, name: &str, op: F)
    where
        F: Fn(i32, i32) -> i32 + 'static,
    {
        self.names.push(name.to_string());
        self.operations.push(Box::new(op));
    }

    fn calculate(&self, index: usize, a: i32, b: i32) -> Option<i32> {
        self.operations.get(index).map(|op| op(a, b))
    }

    fn list_operations(&self) {
        for (i, name) in self.names.iter().enumerate() {
            println!("{}: {}", i, name);
        }
    }
}

fn main() {
    let mut calc = Calculator::new();

    calc.add_operation("add", |a, b| a + b);
    calc.add_operation("subtract", |a, b| a - b);
    calc.add_operation("multiply", |a, b| a * b);
    calc.add_operation("max", |a, b| if a > b { a } else { b });

    calc.list_operations();

    println!("\nCalculations with 10 and 3:");
    for i in 0..4 {
        if let Some(result) = calc.calculate(i, 10, 3) {
            println!("{}: {}", calc.names[i], result);
        }
    }
}
```

---

## Verification Checklist

- [ ] Created closures with various syntax forms
- [ ] Used closures with multiple parameters
- [ ] Used closures with no parameters
- [ ] Captured environment variables immutably
- [ ] Captured environment variables mutably
- [ ] Used move to transfer ownership
- [ ] Passed closures to functions
- [ ] Understood Fn, FnMut, FnOnce traits
- [ ] Created functions that return closures

---

## Summary

You have learned:
- Closures are anonymous functions with `|params| body` syntax
- Closures capture environment variables automatically
- Immutable borrow is the default capture mode
- `move` forces ownership transfer
- `Fn` - borrows immutably, `FnMut` - borrows mutably, `FnOnce` - takes ownership
- Closures can be passed to and returned from functions

**Next:** Lab 10 - Iterators
