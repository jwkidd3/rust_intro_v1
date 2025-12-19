// cargo run --example m6_closures

fn main() {
    let add_one = |x| x + 1;
    let add = |x, y| x + y;
    let greet = || println!("Hello!");

    println!("add_one(5) = {}", add_one(5));
    println!("add(2, 3) = {}", add(2, 3));
    greet();

    // Capture from environment
    let factor = 10;
    let scale = |x| x * factor;
    println!("scale(5) = {}", scale(5));
}
