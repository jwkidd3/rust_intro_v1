// cargo run --example m6_closure_capture

fn main() {
    // Borrow immutably
    let s = String::from("hello");
    let print_s = || println!("{}", s);
    print_s();
    println!("s still valid: {}", s);

    // Borrow mutably
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    println!("count: {}", count);

    // Move ownership
    let name = String::from("Alice");
    let consume = move || println!("Moved: {}", name);
    consume();
    // name no longer valid
}
