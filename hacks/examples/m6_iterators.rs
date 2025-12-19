// cargo run --example m6_iterators
//
// Demonstrates iterators - Rust's way to process sequences of elements.
// Iterators are lazy (don't do work until consumed) and zero-cost.

fn main() {
    println!("=== iter() - Borrow Elements ===");
    // iter() borrows the collection, yields &T (references)
    // Collection remains usable after iteration
    let v = vec![1, 2, 3];
    for val in v.iter() {
        println!("  Got: {} (type: &i32)", val);
    }
    println!("  Vector still valid: {:?}", v);

    println!("\n=== iter_mut() - Mutably Borrow Elements ===");
    // iter_mut() gives mutable references, yields &mut T
    // Allows modifying elements in place
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;  // Dereference and modify
    }
    println!("  After doubling: {:?}", v2);

    println!("\n=== into_iter() - Take Ownership ===");
    // into_iter() consumes the collection, yields T (owned values)
    // Collection is moved and no longer usable
    let v3 = vec![String::from("a"), String::from("b")];
    for val in v3.into_iter() {
        println!("  Owned: {}", val);
    }
    // println!("{:?}", v3);  // ERROR: v3 was moved

    println!("\n=== Iterator with for Loop ===");
    // 'for x in collection' calls into_iter() by default
    let nums = vec![10, 20, 30];
    for n in &nums {      // Same as nums.iter()
        println!("  {}", n);
    }
    for n in &mut nums.clone() {  // Same as nums.iter_mut()
        println!("  {}", n);
    }

    println!("\nSummary:");
    println!("  iter()      -> &T    (borrow)");
    println!("  iter_mut()  -> &mut T (mutable borrow)");
    println!("  into_iter() -> T     (take ownership)");
}
