// cargo run --example m4_custom_result
//
// Demonstrates creating your own functions that return Result.
// Use Result when an operation can fail in an expected way.

// A function that might fail returns Result<SuccessType, ErrorType>
// Here: Success = i32 (the division result), Error = String (error message)
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        // Return an error - division by zero is not allowed
        Err(String::from("Cannot divide by zero"))
    } else {
        // Return success with the computed value
        Ok(a / b)
    }
}

// Another example: parsing a positive number
fn parse_positive(s: &str) -> Result<u32, String> {
    // Try to parse the string as u32
    match s.parse::<u32>() {
        Ok(n) if n > 0 => Ok(n),           // Positive number - success!
        Ok(_) => Err(String::from("Number must be positive")),
        Err(_) => Err(String::from("Invalid number format")),
    }
}

fn main() {
    // Test divide function
    println!("Testing divide function:");

    match divide(10, 2) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("  10 / 0 = {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    // Test parse_positive function
    println!("\nTesting parse_positive function:");

    for input in ["42", "0", "-5", "abc"] {
        match parse_positive(input) {
            Ok(n) => println!("  '{}' -> {}", input, n),
            Err(e) => println!("  '{}' -> Error: {}", input, e),
        }
    }
}
