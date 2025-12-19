// cargo run --example m6_closures
//
// Demonstrates closures - anonymous functions that can capture their environment.
// Closures are used extensively with iterators and for callbacks.

fn main() {
    // Basic closure syntax: |parameters| body
    // Type annotations are optional - Rust infers them

    // Closure with inferred types
    let add_one = |x| x + 1;

    // Closure with explicit types (rarely needed)
    let add_one_explicit = |x: i32| -> i32 { x + 1 };

    // Multiple parameters
    let add = |x, y| x + y;

    // No parameters
    let greet = || println!("Hello!");

    // Multi-line closure with braces
    let complex = |x| {
        let doubled = x * 2;
        let tripled = x * 3;
        doubled + tripled  // Returns last expression
    };

    println!("add_one(5) = {}", add_one(5));
    println!("add_one_explicit(5) = {}", add_one_explicit(5));
    println!("add(2, 3) = {}", add(2, 3));
    greet();
    println!("complex(10) = {}", complex(10));  // 20 + 30 = 50

    // KEY FEATURE: Closures capture variables from their environment
    // Regular functions cannot do this!
    let factor = 10;
    let scale = |x| x * factor;  // Captures 'factor' from outer scope
    println!("\nscale(5) = {} (factor={})", scale(5), factor);

    let name = String::from("Rust");
    let greet_name = || println!("Hello, {}!", name);  // Captures 'name'
    greet_name();
    println!("name is still valid: {}", name);  // name was borrowed, not moved
}
