// cargo run --example m4_unwrap
//
// Demonstrates unwrap, expect, and unwrap_or methods on Result.
// These provide shortcuts for extracting values from Result types.

fn main() {
    // Create example Results to work with
    let err_result: Result<i32, &str> = Err("something went wrong");
    let ok_result: Result<i32, &str> = Ok(42);

    // unwrap_or: Returns the Ok value, or a default if Err
    // Safe - never panics
    let value1 = err_result.unwrap_or(0);  // Returns 0 (the default)
    let value2 = ok_result.unwrap_or(0);   // Returns 42 (the Ok value)
    println!("Err.unwrap_or(0) = {}", value1);
    println!("Ok(42).unwrap_or(0) = {}", value2);

    // unwrap_or_else: Like unwrap_or, but computes default with a closure
    // Useful when the default is expensive to compute
    let result: Result<i32, &str> = Err("oops");
    let value3 = result.unwrap_or_else(|e| {
        println!("Error occurred: {}", e);  // Can log the error
        -1  // Return a computed default
    });
    println!("Result: {}", value3);

    // WARNING: unwrap() and expect() will PANIC on Err!
    // Only use when you're certain the Result is Ok
    // let dangerous = err_result.unwrap();  // Would panic!
    // let also_dangerous = err_result.expect("custom panic message");

    println!("\nSummary:");
    println!("  unwrap_or(default)     - safe, returns default on Err");
    println!("  unwrap_or_else(|e|...) - safe, computes default on Err");
    println!("  unwrap()               - PANICS on Err!");
    println!("  expect(\"message\")      - PANICS on Err with custom message!");
}
