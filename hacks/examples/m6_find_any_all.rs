// cargo run --example m6_find_any_all
//
// Demonstrates find, any, and all - methods for searching and testing iterators.
// These are short-circuiting: they stop as soon as the answer is known.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("=== find() - Get First Match ===");
    // find returns Option<&T> - Some if found, None if not
    // Stops searching as soon as it finds a match

    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("  First even: {:?}", first_even);  // Some(2)

    let first_gt_10 = numbers.iter().find(|x| **x > 10);
    println!("  First > 10: {:?}", first_gt_10);  // None

    // find_map: find and transform in one step
    let strings = vec!["10", "abc", "30"];
    let first_number = strings.iter()
        .find_map(|s| s.parse::<i32>().ok());
    println!("  First valid number: {:?}", first_number);  // Some(10)

    println!("\n=== any() - Does Any Match? ===");
    // Returns true if ANY element matches the predicate
    // Short-circuits: stops on first true

    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("  Has even number: {}", has_even);  // true

    let has_negative = numbers.iter().any(|x| *x < 0);
    println!("  Has negative: {}", has_negative);  // false

    println!("\n=== all() - Do All Match? ===");
    // Returns true if ALL elements match the predicate
    // Short-circuits: stops on first false

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("  All positive: {}", all_positive);  // true

    let all_even = numbers.iter().all(|x| x % 2 == 0);
    println!("  All even: {}", all_even);  // false

    println!("\n=== Practical Example: Validation ===");
    let passwords = vec!["abc", "password123", "SecureP@ss1"];

    // Check password requirements
    let all_long_enough = passwords.iter().all(|p| p.len() >= 6);
    let any_has_number = passwords.iter().any(|p| p.chars().any(|c| c.is_numeric()));
    let any_has_special = passwords.iter().any(|p| p.chars().any(|c| !c.is_alphanumeric()));

    println!("  All >= 6 chars: {}", all_long_enough);
    println!("  Any has number: {}", any_has_number);
    println!("  Any has special char: {}", any_has_special);

    println!("\n=== position() - Find Index ===");
    let position = numbers.iter().position(|x| *x == 3);
    println!("  Position of 3: {:?}", position);  // Some(2)
}
