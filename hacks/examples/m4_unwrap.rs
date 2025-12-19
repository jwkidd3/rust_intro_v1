// cargo run --example m4_unwrap

fn main() {
    let err_result: Result<i32, &str> = Err("error");
    let ok_result: Result<i32, &str> = Ok(42);

    println!("Err.unwrap_or(0) = {}", err_result.unwrap_or(0));
    println!("Ok(42).unwrap_or(0) = {}", ok_result.unwrap_or(0));

    let result: Result<i32, &str> = Err("oops");
    let value = result.unwrap_or_else(|e| {
        println!("Error: {}", e);
        -1
    });
    println!("Result: {}", value);
}
