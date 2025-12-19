// cargo run --example m6_fold
//
// Demonstrates fold - the most powerful iterator consumer.
// fold reduces a collection to a single value using an accumulator.
// Many other methods (sum, product, count) are special cases of fold.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("=== fold() Basics ===");
    // fold(initial_value, |accumulator, element| -> new_accumulator)

    // Sum: start with 0, add each element
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("  Sum: {} (0 + 1 + 2 + 3 + 4 + 5)", sum);

    // Product: start with 1, multiply each element
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("  Product: {} (1 * 2 * 3 * 4 * 5)", product);

    println!("\n=== fold() Step by Step ===");
    // Let's trace through sum:
    let sum_traced = numbers.iter().fold(0, |acc, x| {
        let new_acc = acc + x;
        println!("  acc={}, x={} -> new_acc={}", acc, x, new_acc);
        new_acc
    });
    println!("  Final sum: {}", sum_traced);

    println!("\n=== Building Collections with fold() ===");
    // fold can build any type, not just numbers
    let words = vec!["hello", "world", "rust"];
    let sentence = words.iter().fold(String::new(), |acc, word| {
        if acc.is_empty() {
            word.to_string()
        } else {
            acc + " " + word
        }
    });
    println!("  Sentence: {}", sentence);

    println!("\n=== Convenience Methods (Built on fold) ===");
    // These are equivalent to specific fold operations:
    println!("  sum():     {}", numbers.iter().sum::<i32>());
    println!("  product(): {}", numbers.iter().product::<i32>());
    println!("  count():   {}", numbers.iter().count());
    println!("  max():     {:?}", numbers.iter().max());
    println!("  min():     {:?}", numbers.iter().min());

    println!("\n=== Complex fold Example ===");
    // Count evens and odds in one pass
    let (evens, odds) = numbers.iter().fold((0, 0), |(e, o), x| {
        if x % 2 == 0 { (e + 1, o) } else { (e, o + 1) }
    });
    println!("  Evens: {}, Odds: {}", evens, odds);
}
