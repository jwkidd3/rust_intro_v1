// cargo run --example m6_enumerate_zip
//
// Demonstrates enumerate and zip - iterators for indexed and parallel access.
// enumerate: adds indices to elements
// zip: combines two iterators element by element

fn main() {
    println!("=== enumerate() - Add Indices ===");
    // enumerate() wraps each element as (index, element)
    let fruits = vec!["apple", "banana", "cherry"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", index, fruit);
    }

    // Useful for finding positions
    let numbers = vec![10, 20, 30, 40, 50];
    let position = numbers.iter()
        .enumerate()
        .find(|(_, &val)| val == 30);
    println!("\n  30 is at position: {:?}", position);

    println!("\n=== zip() - Combine Two Iterators ===");
    // zip() pairs elements from two iterators
    // Stops when either iterator is exhausted
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("  {} is {} years old", name, age);
    }

    // Practical: parallel computation
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];
    let sums: Vec<i32> = a.iter()
        .zip(b.iter())
        .map(|(x, y)| x + y)
        .collect();
    println!("\n  Pairwise sums: {:?}", sums);

    // Dot product of two vectors
    let dot_product: i32 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum();
    println!("  Dot product: {}", dot_product);

    println!("\n=== Combining enumerate and zip ===");
    // You can chain these together
    let scores = vec![85, 92, 78];
    let students = vec!["Alice", "Bob", "Charlie"];

    for (rank, (student, score)) in students.iter().zip(scores.iter()).enumerate() {
        println!("  #{}: {} scored {}", rank + 1, student, score);
    }
}
