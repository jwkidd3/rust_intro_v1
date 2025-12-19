// cargo run --example m4_panic
//
// Demonstrates panic! - Rust's mechanism for unrecoverable errors.
// Use panic! when your program reaches an invalid state that cannot be handled.

fn main() {
    let v = vec![1, 2, 3];

    // SAFE: .get() returns Option<&T> - None if index is out of bounds
    // This allows graceful handling instead of crashing
    match v.get(99) {
        Some(val) => println!("Value: {}", val),
        None => println!("No value at index 99 - handled safely!"),
    }

    // UNSAFE: Direct indexing panics if out of bounds
    // Uncomment either line below to see a panic:

    // panic!("crash and burn!");  // Explicit panic with message
    // let _x = v[99];             // Implicit panic: index out of bounds

    // Tip: Set RUST_BACKTRACE=1 to see full stack trace on panic
    // Example: RUST_BACKTRACE=1 cargo run --example m4_panic
}
