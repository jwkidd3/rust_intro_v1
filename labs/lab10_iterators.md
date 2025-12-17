# Lab 10: Iterators

**Duration:** 35 minutes

## Objectives

- Create and consume iterators
- Use map and filter
- Chain iterator methods
- Use fold for accumulation
- Process data with functional style

## Prerequisites

- Completed Lab 9

---

## Setup

Create a new project:
```bash
cargo new lab10_iterators
cd lab10_iterators
code .
```

---

## Exercise 1: Basic Iterators (10 min)

### Step 1: Creating iterators

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Create iterator explicitly
    let iter = v.iter();

    // Consume with for loop
    println!("Using for loop:");
    for value in iter {
        println!("  {}", value);
    }

    // iter() again since previous was consumed
    println!("\nUsing for loop directly on &v:");
    for value in &v {
        println!("  {}", value);
    }
}
```

### Step 2: iter, iter_mut, into_iter

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter() - borrows, yields &T
    println!("iter() - borrowing:");
    for val in v.iter() {
        println!("  {} (type: &i32)", val);
    }
    println!("  v is still valid: {:?}", v);

    // iter_mut() - mutable borrow, yields &mut T
    let mut v2 = vec![1, 2, 3];
    println!("\niter_mut() - mutable borrow:");
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("  v2 after doubling: {:?}", v2);

    // into_iter() - takes ownership, yields T
    let v3 = vec![1, 2, 3];
    println!("\ninto_iter() - taking ownership:");
    for val in v3.into_iter() {
        println!("  {} (type: i32)", val);
    }
    // println!("  v3: {:?}", v3);  // Error: v3 was moved
}
```

### Step 3: Range iterators

```rust
fn main() {
    // Exclusive range
    println!("1..5 (exclusive):");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    // Inclusive range
    println!("1..=5 (inclusive):");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // With step_by
    println!("(0..10).step_by(2):");
    for i in (0..10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    // Reverse
    println!("(1..5).rev():");
    for i in (1..5).rev() {
        print!("{} ", i);
    }
    println!();
}
```

---

## Exercise 2: Map and Filter (10 min)

### Step 1: Using map

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Double each number
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);

    // Square each number
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    // Convert to strings
    let strings: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    println!("Strings: {:?}", strings);
}
```

### Step 2: Using filter

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Keep even numbers
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // Keep numbers greater than 5
    let greater_than_5: Vec<&i32> = numbers.iter().filter(|x| **x > 5).collect();
    println!("Greater than 5: {:?}", greater_than_5);

    // Filter with ownership
    let owned_evens: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Owned evens: {:?}", owned_evens);
}
```

### Step 3: Combining map and filter

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Square the even numbers
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Squared evens: {:?}", result);

    // filter_map - filter and map in one
    let strings = vec!["1", "two", "3", "four", "5"];
    let parsed: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Parsed numbers: {:?}", parsed);
}
```

---

## Exercise 3: More Iterator Methods (10 min)

### Step 1: take and skip

```rust
fn main() {
    let numbers: Vec<i32> = (1..=10).collect();

    // Take first 3
    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    println!("First three: {:?}", first_three);

    // Skip first 5
    let after_five: Vec<i32> = numbers.iter().skip(5).copied().collect();
    println!("After skipping 5: {:?}", after_five);

    // Combine: skip 2, take 3
    let middle: Vec<i32> = numbers.iter().skip(2).take(3).copied().collect();
    println!("Skip 2, take 3: {:?}", middle);
}
```

### Step 2: enumerate and zip

```rust
fn main() {
    let fruits = vec!["apple", "banana", "cherry"];

    // enumerate - add index
    println!("Enumerated:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", index, fruit);
    }

    // zip - combine iterators
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    println!("\nZipped:");
    for (num, letter) in numbers.iter().zip(letters.iter()) {
        println!("  {} - {}", num, letter);
    }

    // Create tuples
    let pairs: Vec<(i32, char)> = numbers
        .into_iter()
        .zip(letters.into_iter())
        .collect();
    println!("\nPairs: {:?}", pairs);
}
```

### Step 3: find, any, all

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // find - first matching element
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    let first_gt_100 = numbers.iter().find(|x| **x > 100);
    println!("First > 100: {:?}", first_gt_100);

    // any - is there at least one?
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    let has_negative = numbers.iter().any(|x| *x < 0);
    println!("Has even: {}", has_even);
    println!("Has negative: {}", has_negative);

    // all - do all match?
    let all_positive = numbers.iter().all(|x| *x > 0);
    let all_even = numbers.iter().all(|x| x % 2 == 0);
    println!("All positive: {}", all_positive);
    println!("All even: {}", all_even);
}
```

---

## Exercise 4: Fold and Reduce (5 min)

### Step 1: Using fold

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Sum using fold
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    // Product using fold
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // Find max using fold
    let max = numbers.iter().fold(i32::MIN, |acc, x| {
        if *x > acc { *x } else { acc }
    });
    println!("Max: {}", max);

    // Build string
    let words = vec!["Hello", "World", "Rust"];
    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });
    println!("Sentence: {}", sentence);
}
```

### Step 2: Using sum and product directly

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();

    println!("Sum: {}", sum);
    println!("Product: {}", product);

    // Count
    let count = numbers.iter().count();
    println!("Count: {}", count);

    // Max and min
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("Max: {:?}, Min: {:?}", max, min);
}
```

---

## Challenge Exercise (Bonus)

Process a list of students and their scores:

```rust
#[derive(Debug)]
struct Student {
    name: String,
    score: u32,
}

fn main() {
    let students = vec![
        Student { name: String::from("Alice"), score: 85 },
        Student { name: String::from("Bob"), score: 72 },
        Student { name: String::from("Charlie"), score: 90 },
        Student { name: String::from("Diana"), score: 68 },
        Student { name: String::from("Eve"), score: 95 },
    ];

    // 1. Find students with score >= 80
    let high_scorers: Vec<&Student> = students
        .iter()
        .filter(|s| s.score >= 80)
        .collect();
    println!("High scorers: {:?}", high_scorers);

    // 2. Get just the names of high scorers
    let high_scorer_names: Vec<&str> = students
        .iter()
        .filter(|s| s.score >= 80)
        .map(|s| s.name.as_str())
        .collect();
    println!("High scorer names: {:?}", high_scorer_names);

    // 3. Calculate average score
    let total: u32 = students.iter().map(|s| s.score).sum();
    let average = total as f64 / students.len() as f64;
    println!("Average score: {:.2}", average);

    // 4. Find the top scorer
    let top = students.iter().max_by_key(|s| s.score);
    println!("Top scorer: {:?}", top);

    // 5. Create a grade report
    let report: Vec<String> = students
        .iter()
        .map(|s| {
            let grade = match s.score {
                90..=100 => "A",
                80..=89 => "B",
                70..=79 => "C",
                60..=69 => "D",
                _ => "F",
            };
            format!("{}: {} ({})", s.name, s.score, grade)
        })
        .collect();

    println!("\nGrade Report:");
    for line in report {
        println!("  {}", line);
    }
}
```

---

## Verification Checklist

- [ ] Created iterators with iter(), iter_mut(), into_iter()
- [ ] Used range iterators with step_by and rev
- [ ] Used map to transform elements
- [ ] Used filter to select elements
- [ ] Combined map and filter
- [ ] Used take and skip
- [ ] Used enumerate and zip
- [ ] Used find, any, all
- [ ] Used fold for accumulation
- [ ] Used sum, product, count, max, min

---

## Summary

You have learned:
- `iter()` borrows, `iter_mut()` borrows mutably, `into_iter()` takes ownership
- `map` transforms each element
- `filter` keeps elements matching a predicate
- `take`, `skip` control how many elements to process
- `enumerate` adds indices, `zip` combines iterators
- `find` returns first match, `any`/`all` test conditions
- `fold` is the most powerful - can implement any reduction
- Iterators are lazy - nothing happens until consumed

**Course Complete!** You now have a solid foundation in Rust programming.
