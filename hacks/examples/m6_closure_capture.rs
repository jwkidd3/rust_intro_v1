// cargo run --example m6_closure_capture
//
// Demonstrates the three ways closures capture variables:
// 1. Borrow immutably (&T) - default for reading
// 2. Borrow mutably (&mut T) - when modifying
// 3. Take ownership (T) - with 'move' keyword

fn main() {
    println!("=== Immutable Borrow ===");
    // Closure borrows 's' immutably - just reads it
    let s = String::from("hello");
    let print_s = || println!("Captured: {}", s);
    print_s();
    print_s();  // Can call multiple times
    println!("s is still valid: {}", s);  // Original still usable

    println!("\n=== Mutable Borrow ===");
    // Closure borrows 'count' mutably - modifies it
    let mut count = 0;
    let mut increment = || {
        count += 1;  // Modifies captured variable
        println!("Count is now: {}", count);
    };
    increment();
    increment();
    // Note: can't use 'count' while closure exists and might be called
    println!("Final count: {}", count);

    println!("\n=== Move (Take Ownership) ===");
    // 'move' keyword forces ownership transfer to closure
    let name = String::from("Alice");
    let consume = move || println!("I own: {}", name);
    consume();
    // println!("{}", name);  // ERROR: name was moved into closure

    println!("\n=== Why use 'move'? ===");
    // Essential when closure outlives the current scope
    // Common with threads or returning closures from functions
    let data = vec![1, 2, 3];
    let closure = move || {
        // Closure owns 'data' - safe to use even if original scope ends
        println!("Data: {:?}", data);
    };
    closure();

    println!("\nCapture rules:");
    println!("  Default: borrows (immut or mut as needed)");
    println!("  'move': takes ownership of all captured variables");
    println!("  Use 'move' for threads or when closure outlives scope");
}
