// cargo run --example m5_derive
//
// Demonstrates #[derive] - automatic implementation of common traits.
// Derive macros generate boilerplate code for standard behaviors.

// #[derive(...)] tells Rust to auto-implement these traits:
// - Debug: enables {:?} formatting for printing
// - Clone: enables .clone() to create a deep copy
// - PartialEq: enables == and != comparison
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// More derivable traits example
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SimplePoint {
    x: i32,
    y: i32,
}
// Copy: enables implicit copying (only for types that are entirely stack-based)
// Eq: full equality (PartialEq that's also reflexive)
// Hash: can be used as HashMap keys

fn main() {
    let p1 = Point { x: 1, y: 2 };

    // Debug trait: print with {:?} or {:#?} (pretty)
    println!("Debug format:  {:?}", p1);
    println!("Pretty format: {:#?}", p1);

    // Clone trait: create a deep copy
    let p2 = p1.clone();
    println!("\nCloned: {:?}", p2);

    // PartialEq trait: compare with == and !=
    if p1 == p2 {
        println!("p1 == p2: true");
    }

    let p3 = Point { x: 3, y: 4 };
    if p1 != p3 {
        println!("p1 != p3: true");
    }

    // Copy trait example (SimplePoint has Copy)
    let sp1 = SimplePoint { x: 10, y: 20 };
    let sp2 = sp1;  // This is a COPY, not a move!
    println!("\nSimplePoint sp1: {:?}", sp1);  // sp1 still valid!
    println!("SimplePoint sp2: {:?}", sp2);

    println!("\nCommon derivable traits:");
    println!("  Debug     - {{:?}} formatting");
    println!("  Clone     - .clone() deep copy");
    println!("  Copy      - implicit copy (stack-only types)");
    println!("  PartialEq - == and != operators");
    println!("  Eq        - full equality");
    println!("  Hash      - can be HashMap key");
    println!("  Default   - Default::default() value");
}
