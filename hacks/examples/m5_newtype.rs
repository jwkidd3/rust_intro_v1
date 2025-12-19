// cargo run --example m5_newtype
//
// Demonstrates the Newtype Pattern - wrapping a type to give it new meaning.
// This provides type safety by making the compiler distinguish between
// values that would otherwise have the same underlying type.

// Newtype: wrap f64 to represent specific units
// These are distinct types even though they both wrap f64
struct Meters(f64);
struct Feet(f64);
struct Seconds(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn to_feet(&self) -> Feet {
        Feet(self.0 * 3.28084)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

impl Feet {
    fn new(value: f64) -> Self {
        Feet(value)
    }

    fn to_meters(&self) -> Meters {
        Meters(self.0 / 3.28084)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

impl Seconds {
    fn new(value: f64) -> Self {
        Seconds(value)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

// Function that only accepts Meters - type safe!
fn calculate_area_meters(length: &Meters, width: &Meters) -> f64 {
    length.value() * width.value()
}

// This prevents accidents like passing Feet where Meters expected
// fn bad_example() {
//     let length = Meters::new(10.0);
//     let width = Feet::new(5.0);  // Wrong unit!
//     calculate_area_meters(&length, &width);  // ERROR: expected Meters, found Feet
// }

fn main() {
    println!("=== Newtype Pattern ===\n");

    // Create values with specific types
    let distance_m = Meters::new(100.0);
    let distance_ft = distance_m.to_feet();

    println!("Distance conversions:");
    println!("  {:.1} meters = {:.2} feet", distance_m.value(), distance_ft.value());

    let height_ft = Feet::new(6.0);
    let height_m = height_ft.to_meters();
    println!("  {:.1} feet = {:.2} meters", height_ft.value(), height_m.value());

    // Type safety in action
    println!("\nType-safe calculations:");
    let length = Meters::new(10.0);
    let width = Meters::new(5.0);
    let area = calculate_area_meters(&length, &width);
    println!("  {}m x {}m = {} square meters", length.value(), width.value(), area);

    // Can't mix types accidentally
    let time = Seconds::new(60.0);
    println!("\n  Time: {} seconds", time.value());
    // calculate_area_meters(&length, &time);  // Won't compile! Type mismatch.

    println!("\nBenefits of Newtype:");
    println!("  - Compiler catches unit mismatches");
    println!("  - Self-documenting code");
    println!("  - Zero runtime cost (same as wrapped type)");
    println!("  - Can implement traits on the wrapper");
}
