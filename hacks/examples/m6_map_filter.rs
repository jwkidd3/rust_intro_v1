// cargo run --example m6_map_filter
//
// Demonstrates map and filter - the two most common iterator adapters.
// Adapters transform iterators lazily (no work until consumed).

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("=== map() - Transform Each Element ===");
    // map applies a function to each element
    // Original: [1, 2, 3, 4, 5] -> Doubled: [2, 4, 6, 8, 10]
    let doubled: Vec<i32> = numbers
        .iter()           // Create iterator over &i32
        .map(|x| x * 2)   // Transform: multiply each by 2
        .collect();       // Consume iterator into Vec
    println!("  Original: {:?}", numbers);
    println!("  Doubled:  {:?}", doubled);

    println!("\n=== filter() - Keep Matching Elements ===");
    // filter keeps elements where the predicate returns true
    // Note: filter receives &&i32 (reference to the reference from iter())
    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)  // Keep only even numbers
        .collect();
    println!("  Evens: {:?}", evens);

    println!("\n=== Chaining map and filter ===");
    // You can chain multiple operations
    // They execute lazily - nothing happens until collect()

    // Get squares of even numbers
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)  // Keep evens: [2, 4]
        .map(|x| x * x)            // Square them: [4, 16]
        .collect();
    println!("  Even squares: {:?}", even_squares);

    // Sum of squares of even numbers
    let sum: i32 = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum();  // Consume by summing
    println!("  Sum of even squares: {}", sum);  // 4 + 16 = 20

    println!("\n=== Order Matters ===");
    // filter then map: fewer elements to transform
    // map then filter: transform all, then discard
    let result: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0)   // [2,4,6,8,10] - 5 elements
        .map(|x| x * 10)          // [20,40,60,80,100]
        .collect();
    println!("  Filter then map: {:?}", result);
}
