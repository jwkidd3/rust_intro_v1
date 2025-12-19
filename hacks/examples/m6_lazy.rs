// cargo run --example m6_lazy
//
// Demonstrates that iterators in Rust are LAZY - they don't do any work
// until you consume them. This enables efficient chaining and avoids
// unnecessary computation.

fn main() {
    println!("=== Lazy Iterators ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // LAZY: This creates an iterator but does NO work yet!
    println!("Creating iterator chain (nothing executes yet)...");
    let iter = numbers.iter().map(|x| {
        println!("  [map] Processing {}", x);
        x * 2
    }).filter(|x| {
        println!("  [filter] Checking {}", x);
        x > &4
    });

    println!("Iterator created. No output above means no work done!\n");

    // NOW it executes when we consume the iterator
    println!("Consuming with collect() - NOW it runs:");
    let result: Vec<_> = iter.collect();
    println!("\nResult: {:?}", result);

    // EFFICIENCY: Lazy evaluation enables working with infinite sequences
    println!("\n=== Infinite Iterators ===");
    println!("Creating infinite range (1..) - only possible because lazy!");

    // This would be impossible if iterators were eager (would loop forever)
    let first_5_squares: Vec<i32> = (1..)       // Infinite range: 1, 2, 3, 4, ...
        .map(|x| x * x)                          // Square each (lazily)
        .take(5)                                 // Only take first 5
        .collect();                              // Now compute!

    println!("  First 5 squares from infinite range: {:?}", first_5_squares);

    // EFFICIENCY: Short-circuiting
    println!("\n=== Short-Circuiting ===");
    println!("Finding first even square > 10:");

    let result = (1..)
        .inspect(|x| print!("  generate {} -> ", x))
        .map(|x| x * x)
        .inspect(|x| print!("square {} -> ", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| print!("even {} -> ", x))
        .find(|x| *x > 10);

    println!("\n  Result: {:?}", result);
    println!("  Notice: stopped as soon as it found the answer!\n");

    // ZERO-COST ABSTRACTION
    println!("=== Zero-Cost Abstraction ===");
    println!("Iterator chains compile to the same code as hand-written loops.");
    println!("No runtime overhead for the functional style!");

    // This functional code:
    let sum_functional: i32 = (1..=100)
        .filter(|x| x % 2 == 0)
        .sum();

    // Is as fast as this imperative code:
    let mut sum_imperative = 0;
    for i in 1..=100 {
        if i % 2 == 0 {
            sum_imperative += i;
        }
    }

    println!("  Functional sum of evens 1-100: {}", sum_functional);
    println!("  Imperative sum of evens 1-100: {}", sum_imperative);
}
