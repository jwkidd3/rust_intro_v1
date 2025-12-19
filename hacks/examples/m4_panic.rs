// cargo run --example m4_panic

fn main() {
    let v = vec![1, 2, 3];

    // Safe access
    match v.get(99) {
        Some(val) => println!("Value: {}", val),
        None => println!("No value at index 99"),
    }

    // Uncomment to see panic:
    // panic!("crash and burn!");
    // v[99];
}
