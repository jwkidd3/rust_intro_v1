// cargo run --example m5_tuple_structs
//
// Demonstrates tuple structs - structs with unnamed fields.
// Useful when you want a distinct type but field names aren't needed.

// Tuple structs: like tuples, but with a type name
// Fields are accessed by index (0, 1, 2...) not by name
struct Color(i32, i32, i32);  // RGB color
struct Point(i32, i32, i32);  // 3D point

// Unit struct: no fields at all (useful for traits)
struct AlwaysEqual;

fn main() {
    // Create instances using tuple-like syntax
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access fields by index
    println!("Black color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Origin point - X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    // IMPORTANT: Color and Point are different types!
    // Even though they have the same field types, they cannot be mixed
    // This provides type safety:
    // let point: Point = black;  // ERROR: mismatched types

    // Create a red color
    let red = Color(255, 0, 0);
    println!("\nRed color - R: {}, G: {}, B: {}", red.0, red.1, red.2);

    // Destructure tuple structs
    let Color(r, g, b) = red;
    println!("Destructured - R: {}, G: {}, B: {}", r, g, b);

    // Unit struct instance (no parentheses needed)
    let _equal = AlwaysEqual;
}
